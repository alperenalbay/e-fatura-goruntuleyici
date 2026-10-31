# Değişiklik Günlüğü

Bu dosya [Keep a Changelog](https://keepachangelog.com/tr/1.1.0/) biçimini izler ve
[semantik sürümleme](https://semver.org/lang/tr/) kullanır.

## [1.0.0] - 2026-10-31

İlk açık kaynak sürümü.

### Eklenenler

- Java gerektirmeyen Tauri 2 + Rust masaüstü uygulaması (Windows).
- e-Fatura, e-Arşiv ve e-İrsaliye XML görüntüleme.
- XML içindeki gömülü XSLT ile GİB görünümü; gömülü XSLT yoksa yerleşik görünüm.
- Mali mühür / XAdES bilgi paneli: Doğrulama Bilgileri ve Sertifika Bilgileri.
- GİB sorgulama: e-Fatura, e-Arşiv ve e-İrsaliye için ayrı sorgu mantığı; kayıtlı
  belgelerde fatura tarihi, imzalanma, GİB'e/alıcıya iletilme, alıcı yanıtı ve
  iptal/itiraz durumu.
- Yazdır / PDF desteği.
- Sürükle-bırak, komut satırı ve Windows "Birlikte aç" ile dosya açma.
- NSIS kurulum paketi ve taşınabilir exe.
- Tarayıcıda çalışabilen tek dosyalık arayüz (`index.html`).
- Kurgusal örnek fatura (`examples/`).

### Düzeltmeler

- Belge görünümü sandbox'lı çerçevede gösteriliyor; gömülü XSLT çıktısındaki
  `script`, `iframe`, olay öznitelikleri ve `javascript:` adresleri temizleniyor.
- İçerik Güvenliği Politikası (CSP) eklendi; ağ erişimi GİB sorgu adresiyle sınırlı.
- Ek dosya bağlantıları yalnızca geçerli base64 verisi ve izinli MIME türleriyle
  oluşturuluyor (HTML enjeksiyonu engellendi).
- `dosya_oku` yalnızca `.xml`, `.xsl` ve `.xslt` dosyalarını okur.
- GİB sorgusunda tam alan adı denetimi, yönlendirme engeli ve yanıt boyutu sınırı.
- GİB sorgusunda VKN/TCKN seçimi: Ticaret Sicil No yerine `schemeID` içinde VKN/TCKN
  geçen kimlik kullanılıyor.
- GİB `SEND_TIME` / `RECEIVE_TIME` etiketleri doğru alanlara eşlendi.
- GİB yanıtı tek satırda gelse de ayrıştırılıyor.

### Bilinen sınırlamalar

- İmza için kriptografik doğrulama yapılmaz.
- GİB şema doğrulaması yapılmaz.
