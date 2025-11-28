mod config;
mod backend;
mod gui;

use eframe::egui;
use gui::PackageManagerApp;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
        .with_inner_size([1200.0, 800.0])
        .with_title("Pisi GNU/Linux Software Center"),

        // Sistem temasını takip et (basitleştirilmiş)
        follow_system_theme: false, // Şimdilik false, System teması henüz eklenmedi
        default_theme: eframe::Theme::Dark,

            ..Default::default()
    };

    eframe::run_native(
        "Pisi Package Manager",
        options,
        Box::new(|cc| {
            // Başlangıçta dark tema uygula
            cc.egui_ctx.set_visuals(egui::Visuals::dark());

            Box::new(PackageManagerApp::new(cc))
        }),
    )
}

