# Pisi Paket Yöneticisi - Migration ve Yenilikler

Bu belge, uygulamanın **egui** (Rust Native GUI) mimarisinden **Tauri** (Web Frontend + Rust Backend) mimarisine geçiş sürecindeki teknik değişiklikleri ve eklenen yeni özellikleri özetler.

## 1. Mimari Değişiklikler
- **Framework Değişimi**: `egui/eframe` tamamen kaldırıldı. Yerine modern bir web arayüzü sunan `Tauri (v2)` entegre edildi.
- **Proje Yapısı**: 
    - `src/`: Backend mantığı (`pm_core` kütüphanesi).
    - `src-tauri/`: Tauri yapılandırması ve Rust komutları.
    - `frontend/`: HTML, CSS ve modüler JavaScript dosyaları.
- **Kütüphane İsimlendirmeleri**: Çakışmaları önlemek için ana paket `pisi-package-manager`, tauri paketi `pm-gui` olarak güncellendi.

## 2. Çoklu Dil Desteği (i18n)
Uygulama artık tam kapsamlı bir çoklu dil desteğine sahiptir:
- **Otomatik Dil Algılama**: Sistem dili (tarayıcı/işletim sistemi) otomatik olarak algılanır.
- **Dil Dosyaları**: `frontend/locales/` dizini altında `tr.json` ve `en.json` dosyaları üzerinden yönetilir.
- **Dinamik Değişim**: Ayarlar menüsü üzerinden uygulama yeniden başlatılmadan dil değiştirilebilir. Tercihler `localStorage` üzerinde saklanır.
- **Çeviri Motoru**: `frontend/js/i18n.js` modülü, DOM üzerindeki `data-i18n` özniteliklerini otomatik olarak tarayıp günceller.

## 3. Frontend Layout Sistemi
- **HTML Tabanlı Yapı**: Arayüz; `Header`, `Sidebar`, `MainView` ve `Layout` olarak `.html` dosyalarına ayrılmıştır.
- **Dinamik Yükleme**: Bileşenler `app.js` üzerinden `fetch` ile çalışma anında yüklenir.
- **Tema Desteği**: Dark/Light mode desteği ve merkezi CSS yönetimi meklendi.

## 4. Performans ve Backend İyileştirmeleri
- **Binary Cache (Bincode)**: Pisi XML indeksi parse edildikten sonra `/tmp/pisi-pm-index-cache.bin` dosyasına binary formatta kaydedilir. Bu, sonraki açılışları **20 kat** hızlandırır.
- **RAM Önbelleği (Lazy Static)**: Veriler uygulama boyunca RAM'de tutulur.
- **Akıllı Geçersiz Kılma**: Orijinal XML dosyası güncellendiğinde önbellek otomatik yenilenir.
- **Git Yönetimi**: Build dosyalarının takibini önlemek için `target/` ve `Cargo.lock` dosyalarını kapsayan `.gitignore` eklendi.

## 5. Yeni Eklenen Tauri Komutları
- `get_packages`: XML/Cache üzerinden paket listesini çeker.
- `get_package_stats`: Kurulu ve mevcut paket sayılarını dinamik hesaplar.
- `install_package` / `remove_package`: `pisi` komutlarını asenkron çalıştırır.
- `get_components`: Paketleri kategorilerine göre gruplar.

---
*Pisi Linux Geliştirme Ekibi*
