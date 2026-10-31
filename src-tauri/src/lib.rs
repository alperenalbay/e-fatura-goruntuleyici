use std::sync::Mutex;
use std::time::Duration;

use tauri::State;

/// Uygulama dosya ile açıldığında (çift tık / sürükle-bırak) yolu saklar.
struct BaslangicDosyasi(Mutex<Option<String>>);

const MAX_DOSYA_BOYUTU: u64 = 64 * 1024 * 1024;
const MAX_GIB_YANIT: usize = 4 * 1024 * 1024;

/// Uygulama bir XML dosyası ile başlatıldıysa yolunu döndürür.
#[tauri::command]
fn baslangic_dosyasi(state: State<'_, BaslangicDosyasi>) -> Option<String> {
    state.0.lock().ok().and_then(|yol| yol.clone())
}

/// Diskten metin dosyası okur (XML / XSLT).
#[tauri::command]
fn dosya_oku(yol: String) -> Result<String, String> {
    let p = std::path::Path::new(&yol);
    let uzanti = p
        .extension()
        .and_then(|e| e.to_str())
        .unwrap_or("")
        .to_ascii_lowercase();
    if !matches!(uzanti.as_str(), "xml" | "xsl" | "xslt") {
        return Err("Yalnızca .xml, .xsl ve .xslt dosyaları açılabilir.".into());
    }
    let kanonik = p
        .canonicalize()
        .map_err(|e| format!("Dosya bulunamadı: {e}"))?;
    let meta = std::fs::metadata(&kanonik).map_err(|e| format!("Dosya bulunamadı: {e}"))?;
    if !meta.is_file() {
        return Err("Dosya bulunamadı.".into());
    }
    if meta.len() > MAX_DOSYA_BOYUTU {
        return Err("Dosya çok büyük (en fazla 64 MB).".into());
    }
    let bayt = std::fs::read(&kanonik).map_err(|e| format!("Dosya okunamadı: {e}"))?;
    if bayt.len() as u64 > MAX_DOSYA_BOYUTU {
        return Err("Dosya çok büyük (en fazla 64 MB).".into());
    }
    let metin = String::from_utf8_lossy(&bayt).to_string();
    Ok(metin.trim_start_matches('\u{feff}').to_string())
}

/// GİB sorgu servisine istek atar ve yanıt metnini döndürür.
///
/// Tarayıcıda CORS nedeniyle okunamayan yanıt, bu komut sayesinde
/// uygulama içinde doğrudan gösterilebilir.
#[tauri::command]
async fn gib_sorgula(url: String) -> Result<String, String> {
    let adres = reqwest::Url::parse(&url).map_err(|_| "Geçersiz adres.".to_string())?;
    if adres.scheme() != "https" || adres.host_str() != Some("sorgu.efatura.gov.tr") {
        return Err("Yalnızca GİB sorgu adresine istek yapılabilir.".into());
    }

    let istemci = reqwest::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) e-Fatura-Goruntuleyici/1.0")
        .timeout(Duration::from_secs(40))
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .map_err(|e| format!("İstemci oluşturulamadı: {e}"))?;

    let yanit = istemci
        .get(adres)
        .send()
        .await
        .map_err(|e| format!("GİB'e bağlanılamadı: {e}"))?;

    let durum = yanit.status();
    if !durum.is_success() {
        return Err(format!("GİB yanıtı: HTTP {}", durum.as_u16()));
    }
    if yanit.content_length().unwrap_or(0) > MAX_GIB_YANIT as u64 {
        return Err("GİB yanıtı çok büyük.".into());
    }

    let govde = yanit
        .bytes()
        .await
        .map_err(|e| format!("Yanıt okunamadı: {e}"))?;
    if govde.len() > MAX_GIB_YANIT {
        return Err("GİB yanıtı çok büyük.".into());
    }

    // Yanıt windows-1254 olabilir; sonuç alanları ASCII olduğu için
    // kayıplı dönüştürme yeterli.
    Ok(String::from_utf8_lossy(&govde).to_string())
}

/// Uygulamayı kapatır.
#[tauri::command]
fn cikis(uygulama: tauri::AppHandle) {
    uygulama.exit(0);
}

fn acilacak_dosya() -> Option<String> {
    std::env::args().skip(1).find_map(|arg| {
        let yol = std::path::PathBuf::from(&arg);
        let xml_mi = yol
            .extension()
            .and_then(|e| e.to_str())
            .map(|e| e.eq_ignore_ascii_case("xml"))
            .unwrap_or(false);
        if xml_mi && yol.is_file() {
            Some(yol.to_string_lossy().to_string())
        } else {
            None
        }
    })
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(BaslangicDosyasi(Mutex::new(acilacak_dosya())))
        .invoke_handler(tauri::generate_handler![
            baslangic_dosyasi,
            dosya_oku,
            gib_sorgula,
            cikis
        ])
        .run(tauri::generate_context!())
        .expect("Uygulama başlatılamadı");
}
