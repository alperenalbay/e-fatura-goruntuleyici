# Güvenlik Politikası

## Desteklenen sürümler

En son yayımlanan sürüm güvenlik düzeltmeleri alır.

| Sürüm | Destek |
| --- | --- |
| 1.x | Evet |

## Açık bildirimi

Güvenlik açıklarını herkese açık issue olarak **bildirmeyin**. Bunun yerine deponun
**Security → Report a vulnerability** bölümünü kullanın veya doğrudan
**alperenalbayin@protonmail.com** adresine yazın.

Bildirimde bulunurken:

- Etkilenen sürüm ve Windows sürümü
- Açığın türü ve etkisi
- Yeniden üretme adımları (gerçek fatura/kişisel veri olmadan)
- Varsa önerilen düzeltme

Makul süre içinde yanıt vermeye ve düzeltmeyi yayımlamaya çalışıyoruz. Lütfen düzeltme
yayımlanmadan önce açığı kamuya açıklamayın.

## Kapsam

- Uygulama yerel dosyaları okur ve yalnızca kullanıcı **Sorgula** dediğinde
  `https://sorgu.efatura.gov.tr/` adresine istek atar.
- Telemetri, otomatik güncelleme veya arka plan ağ erişimi yoktur.
- Mali mühür/e-imza için kriptografik doğrulama yapılmadığı açıkça belirtilir; bu bir
  güvenlik açığı değil, bilinen bir sınırlamadır.
