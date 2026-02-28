# Pisi Paket Yöneticisi - Migration ve Yenilikler

Bu belge, uygulamanın **egui** (Rust Native GUI) mimarisinden **Tauri** (Web Frontend + Rust Backend) mimarisine geçiş sürecindeki teknik değişiklikleri ve eklenen yeni özellikleri özetler.

## 1. Mimari Değişiklikler
- **Framework Değişimi**: `egui/eframe` tamamen kaldırıldı. Yerine modern bir web arayüzü sunan `Tauri (v2)` entegre edildi.
- **Proje Yapısı**: 
    - `src/`: Backend mantığı (`pm_core` kütüphanesi).
    - `src-tauri/`: Tauri yapılandırması ve Rust komutları.
    - `frontend/`: HTML, CSS ve modüler JavaScript dosyaları.
- **Kütüphane İsimlendirmeleri**: Çakışmaları önlemek için ana paket `pisi-package-manager`, tauri paketi `pm-gui` olarak güncellendi.

## 2. Frontend Layout Sistemi
- **Bileşen Bazlı Yapı**: Arayüz; `Header`, `Sidebar`, `MainView` ve `Layout` olarak `.html` dosyalarına ayrıldı.
- **Dinamik Yükleme**: `app.js` üzerinden `fetch` ile çalışma anında yüklenen modüler bir yapı kuruldu.
- **Tema Desteği**: Dark/Light mode desteği ve merkezi CSS yönetimi eklendi.

## 3. Performans ve Backend İyileştirmeleri
- **Binary Cache (Bincode)**: 6000+ paket içeren Pisi XML indeksi artık her açılışta parse edilmiyor. `bincode` formatında `/tmp/pisi-pm-index-cache.bin` adresine önbelleğe alınıyor. Bu, açılış hızını yaklaşık **20 kat** artırdı.
- **RAM Önbelleği (Lazy Static)**: Uygulama açıkken veriler RAM'de tutulur, gereksiz disk okumaları engellenir.
- **Akıllı Geçersiz Kılma**: Eğer `/var/lib/pisi/index/` altındaki XML dosyası değişirse, sistem bunu otomatik fark edip önbelleği tazeler.

## 4. Yeni Eklenen Tauri Komutları
- `get_packages`: XML/Cache üzerinden paket listesini çeker.
- `get_package_stats`: Kurulu ve mevcut paket sayılarını dinamik hesaplar.
- `install_package` / `remove_package`: `pisi` komutlarını asenkron olarak çalıştırır.
- `get_components`: Paketleri kategorilerine göre gruplar.

---
*Pisi Linux Geliştirme Ekibi*
