use eframe::egui;
use crate::config::{AppSettings, Theme};
use crate::backend::xml_parser::{XmlParser, PackageInfo, Component};
use crate::gui::components::{Sidebar, PackageGrid, PackageDetails};
use crate::gui::components::settings_modal;
use crate::config::SettingsModalState;
use crate::gui::events::{AppEvent, EventManager};

#[derive(PartialEq)]
pub enum AppView {
    Welcome,
    PackageList,
}

pub struct PackageManagerApp {
    pub current_theme: Theme,
    pub settings: AppSettings,
    pub show_settings: bool,
    pub current_view: AppView,

    // Data
    pub packages: Vec<PackageInfo>,
    pub components: Vec<Component>,
    pub selected_component: String,
    pub selected_category: String,
    pub selected_package: Option<PackageInfo>,

    // UI State
    pub sidebar: Sidebar,
    pub package_grid: PackageGrid,
    pub package_details: PackageDetails,
    pub settings_modal: SettingsModalState,

    // Event system
    pub event_manager: EventManager,
}

impl PackageManagerApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Sistem fontlarını yükle
        Self::setup_fonts(&cc.egui_ctx);

        // Initialize with default data (will be replaced with XML parsing)
        let packages = Vec::new();
        let components = Vec::new();

        Self {
            current_theme: Theme::Light,
            settings: AppSettings::default(),
            show_settings: false,
            current_view: AppView::Welcome,
            packages,
            components,
            selected_component: "All".to_string(),
            selected_category: "Installed".to_string(),
            selected_package: None,
            sidebar: Sidebar::default(),
            package_grid: PackageGrid::default(),
            package_details: PackageDetails::default(),
            settings_modal: SettingsModalState::default(),
            event_manager: EventManager::new(),
        }
    }

    /// Sistem fontlarını ve stillerini ayarla
    fn setup_fonts(ctx: &egui::Context) {
        let fonts = egui::FontDefinitions::default(); // mut kaldırıldı

        // Sistem fontlarını kullan
        // Not: Egui varsayılan olarak sistem fontlarını kullanır

        ctx.set_fonts(fonts);

        // Stil ayarları - sistem temasına uyumlu
        let mut style = (*ctx.style()).clone();

        // Daha modern görünüm için spacing ayarları
        style.spacing.item_spacing = egui::vec2(8.0, 6.0);
        style.spacing.button_padding = egui::vec2(12.0, 6.0);

        ctx.set_style(style);
    }

    pub fn load_packages_from_xml(&mut self) {
        // This will load actual data from XML file
        // For now, using mock data
        if self.packages.is_empty() {
            // In real implementation, read from /var/lib/pisi/index/Stable/pisi-index.xml
            self.components = XmlParser::parse_components(&self.packages);
        }
    }

    fn handle_events(&mut self) {
        while let Some(event) = self.event_manager.pop() {
            match event {
                AppEvent::CategorySelected(category) => {
                    self.selected_category = category;
                    self.current_view = AppView::PackageList;
                }
                AppEvent::ComponentSelected(component) => {
                    self.selected_component = component;
                    self.current_view = AppView::PackageList;
                }
                AppEvent::ShowSettings => {
                    self.show_settings = true;
                }
            }
        }
    }
}

impl eframe::App for PackageManagerApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Apply theme
        self.current_theme.apply(ctx);

        // Handle pending events
        self.handle_events();

        // Load data if needed
        self.load_packages_from_xml();

        // Top panel with header
        egui::TopBottomPanel::top("header").show(ctx, |ui| {
            self.render_header(ui);
        });

        // Sidebar
        egui::SidePanel::left("sidebar")
        .resizable(false)
        .min_width(250.0)
        .show(ctx, |ui| {
            self.sidebar.render(ui, &mut self.event_manager);
        });

        // Package details panel
        egui::SidePanel::right("details")
        .resizable(false)
        .min_width(300.0)
        .show(ctx, |ui| {
            let app_ref = &*self;
            self.package_details.render(ui, app_ref);
        });

        // Main content area
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_view {
                AppView::Welcome => self.render_welcome_screen(ui),
                                           AppView::PackageList => {
                                               let app_ref = &*self;
                                               self.package_grid.render(ui, app_ref);
                                           }
            }
        });

        // Settings modal
        if self.show_settings {
            settings_modal::SettingsModal::render(ctx, self);
        }
    }
}

impl PackageManagerApp {
    fn render_header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.heading("Pisi GNU/Linux Software Center");

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Theme toggle - sadece Light/Dark
                let theme_text = if self.current_theme == Theme::Light {
                    "🌙 Dark"
                } else {
                    "☀️ Light"
                };

                if ui.button(theme_text).clicked() {
                    self.current_theme = self.current_theme.next();
                }

                // Settings button
                if ui.button("⚙️ Settings").clicked() {
                    self.show_settings = true;
                }
            });
        });
    }

    fn render_welcome_screen(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(100.0);

            // Logo/Icon
            ui.heading("🦊 Pisi GNU/Linux");
            ui.add_space(20.0);

            // Welcome message
            ui.heading("Paket Yöneticisine Hoş Geldiniz");
            ui.add_space(10.0);
            ui.label("Sol taraftaki kategorilerden birini seçerek paketleri görüntüleyebilirsiniz.");

            ui.add_space(40.0);

            // Quick stats
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    ui.heading("6501");
                    ui.label("Toplam Paket");
                });

                ui.add_space(30.0);

                ui.vertical(|ui| {
                    ui.heading("15");
                    ui.label("Güncelleme");
                });

                ui.add_space(30.0);

                ui.vertical(|ui| {
                    ui.heading("3127");
                    ui.label("Kurulu Paket");
                });
            });

            ui.add_space(40.0);

            // Quick actions
            ui.horizontal(|ui| {
                if ui.button("📦 Tüm Paketleri Görüntüle").clicked() {
                    self.selected_category = "All".to_string();
                    self.current_view = AppView::PackageList;
                }

                if ui.button("🔄 Güncellemeleri Kontrol Et").clicked() {
                    self.selected_category = "Updates".to_string();
                    self.current_view = AppView::PackageList;
                }
            });
        });
    }
}
