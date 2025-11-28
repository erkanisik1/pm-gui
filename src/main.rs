mod config;
mod backend;
mod gui;

use eframe::egui;
use gui::PackageManagerApp;

fn main() -> eframe::Result<()> {
    // Asset dosyalarının varlığını kontrol et
    check_assets();
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([1200.0, 800.0])
            .with_title("Pisi GNU/Linux Software Center"),
        
        follow_system_theme: false,
        default_theme: eframe::Theme::Dark,
        
        ..Default::default()
    };

    eframe::run_native(
        "Pisi Package Manager",
        options,
        Box::new(|cc| {
            cc.egui_ctx.set_visuals(egui::Visuals::dark());
            Box::new(PackageManagerApp::new(cc))
        }),
    )
}

fn check_assets() {
    let current_dir = std::env::current_dir().unwrap_or_default();
    
    let asset_files = [
        current_dir.join("assets/pisi-logo-light.png"),
        current_dir.join("assets/pisi-logo-dark.png"), 
        current_dir.join("assets/package-manager-icon-systemtr.png"),
    ];
    
    println!("Checking asset files from: {}", current_dir.display());
    for asset in &asset_files {
        if asset.exists() {
            println!("✓ Found: {}", asset.display());
        } else {
            println!("✗ Missing: {}", asset.display());
        }
    }
    
    // Assets dizinini de kontrol et
    let assets_dir = current_dir.join("assets");
    if assets_dir.exists() {
        println!("✓ Assets directory exists");
        if let Ok(entries) = std::fs::read_dir(&assets_dir) {
            for entry in entries.flatten() {
                println!("  - {}", entry.file_name().to_string_lossy());
            }
        }
    } else {
        println!("✗ Assets directory missing!");
    }
}
