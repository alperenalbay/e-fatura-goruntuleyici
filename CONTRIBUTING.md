# Katkı Rehberi

Katkılar için teşekkürler. Bu proje küçük ve tek amaçlı bir masaüstü uygulamasıdır;
değişikliklerin kapsamı dar tutulmalıdır.

## Geliştirme ortamı

Gereksinimler: Rust (MSVC toolchain), Visual Studio Build Tools (C++ iş yükü), Node.js 18+.

```bat
npm install
npm run dev
```

Arayüzün tek kaynağı `index.html` dosyasıdır. `ui/index.html` derleme sırasında üretilir,
doğrudan düzenlenmez.

## Değişiklik gönderme

1. Depoyu çatallayın (fork) ve konuya özel bir dal açın: `fix/gib-sorgu`, `feat/pdf`.
2. Değişikliği küçük ve tek amaçlı tutun; ilgisiz biçimlendirme değişikliklerinden kaçının.
3. Arayüz değişikliklerini hem masaüstünde hem tarayıcıda deneyin.
4. Derlemenin geçtiğini doğrulayın:

   ```bat
   npm run build
   ```

5. Açıklayıcı bir başlıkla pull request açın: ne değişti, neden değişti, nasıl test edildi.

## Kod stili

- JavaScript: mevcut dosyadaki 2 boşluk girinti, `const`/`let`, Türkçe kullanıcı metinleri.
- Rust: `cargo fmt` ve `cargo clippy` çıktısına uyun.
- Kullanıcıya görünen metinler Türkçe olmalı; kod tanımlayıcıları İngilizce veya mevcut
  dosyadaki Türkçe adlandırma ile tutarlı olabilir.

## Gizlilik kuralları

- **Gerçek fatura XML'i, VKN/TCKN, isim, adres veya tutar paylaşmayın.**
- Hata bildirimlerinde kurgusal örnek kullanın; ekran görüntülerinde hassas alanları kapatın.
- `examples/` altındaki dosyalar tamamen kurgusal olmalıdır.

## Hata bildirimi

Lütfen şunları ekleyin:

- Uygulama sürümü ve Windows sürümü
- Belge türü (e-Fatura / e-Arşiv / e-İrsaliye) ve sorunun adımları
- Beklenen ve gerçekleşen davranış
- Varsa hata mesajı veya ekran görüntüsü (hassas veri olmadan)

Güvenlik açıkları için [SECURITY.md](SECURITY.md) dosyasındaki yolu izleyin.
