# Pisi GNU/Linux Paket Yöneticisi (Tauri)

Bu uygulama, Pisi GNU/Linux için geliştirilmiş, modern ve hızlı bir paket yöneticisi grafik arayüzüdür. **Tauri v2** framework'ü kullanılarak Rust (backend) ve HTML/JS (frontend) mimarisiyle inşa edilmiştir.

## 🚀 Öne Çıkan Özellikler
- **Modüler Layout**: HTML tabanlı bileşen sistemi ile kolay özelleştirilebilir arayüz.
- **Yüksek Performans**: `bincode` tabanlı XML önbellekleme ile anlık veri erişimi.
- **Gerçek Zamanlı İstatistikler**: Sistemdeki paket durumuna göre anlık güncellenen sayacı ve veriler.
- **Modern Arayüz**: Dark/Light tema desteği ve hızlı kategori geçişleri.

## 📁 Proje Yapısı
- `src/`: Core Pisi mantığı ve XML parser (Rust).
- `src-tauri/`: Tauri yapılandırması ve Rust API komutları.
- `frontend/`: 
    - `components/`: Ayrı HTML bileşenleri (Header, Sidebar, Layout).
    - `app.js`: Ana uygulama mantığı ve asenkron veri yönetimi.
    - `style.css`: Modern ve responsive tasarım.

## 🛠️ Teknik Detaylar ve Geçiş Süreci
Uygulama daha önce `egui` framework'ü ile geliştirilmekteydi. Tauri'ye geçiş süreci, yeni layout sistemi ve performans iyileştirmeleri hakkında detaylı bilgi için:

👉 **[MIGRATION.md - Teknik Detaylar ve Yenilikler](./MIGRATION.md)**

## 🔨 Geliştirme Notları
Uygulamayı geliştirmek veya test etmek için:

```bash
# Bağımlılıkları kontrol et
cargo check

# Geliştirme modunda çalıştır
cargo tauri dev
```

---
*Pisi Linux Takımı*
