# Mevcut Proje Yapısı
```
src/
├── main.rs
├── lib.rs
├── gui/
│   ├── app.rs
│   ├── mod.rs
│   ├── events.rs
│   ├── image_loader.rs
│   ├── themes.rs
│   └── components/
│       ├── mod.rs
│       ├── sidebar.rs
│       ├── package_grid.rs
│       ├── package_details.rs
│       └── settings_modal.rs
├── backend/
│   ├── mod.rs
│   ├── package_manager.rs
│   ├── repository.rs
│   └── xml_parser.rs
└── config/
    ├── mod.rs
    ├── settings.rs
    └── themes.rs
```
# Şu anki proje durumu:
- Temel paket yönetici arayüzü
- Welcome ekranı ve kategori geçişleri
- Ayarlar paneli
- Light/Dark tema desteği
- Modüler mimari (backend, config, gui)
- Gerçek XML parsing (şu an mock data)
- Pisi paket yöneticisi entegrasyonu
- Sistem teması algılama
- Paket arama ve filtreleme

# İş Bölümü Önerileri

Hangi alanlarda katkıda bulunmak istersiniz?

1. Backend Geliştirme
    * XML parser'ı gerçek verilerle çalışacak şekilde güncelleme
    * Pisi komutları entegrasyonu
    * Paket cache ve repo yönetimi

3. Frontend/UI Geliştirme
    - Paket arama ve filtreleme
    - Daha gelişmiş paket detay görünümü
    - Sistem teması algılama
    - Responsive tasarım iyileştirmeleri

5. Özellik Geliştirme
    - Paket güncelleme yöneticisi
    - Batch işlemler (toplu kurma/kaldırma)
    - Paket istatistikleri ve raporlama
