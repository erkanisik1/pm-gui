// src-tauri/src/commands.rs
use serde::{Serialize, Deserialize};
use pm_core::backend::package_manager;
use pm_core::backend::xml_parser::{PackageInfo, Component};

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageStats {
    pub total_count: usize,
    pub installed_count: usize,
    pub available_count: usize,
    pub updates_count: usize,
}

#[tauri::command]
pub async fn get_package_stats() -> Result<PackageStats, String> {
    let packages = package_manager::get_packages().await?;
    let installed = package_manager::get_installed_packages().await?;
    let updates = package_manager::get_upgradable_packages().await?;
    
    let total_count = packages.len();
    let installed_count = installed.len();
    let available_count = if total_count > installed_count { total_count - installed_count } else { 0 };
    let updates_count = updates.len();

    Ok(PackageStats {
        total_count,
        installed_count,
        available_count,
        updates_count,
    })
}

#[tauri::command]
pub async fn get_upgradable_packages() -> Result<Vec<String>, String> {
    package_manager::get_upgradable_packages().await
}

#[tauri::command]
pub async fn get_packages() -> Result<Vec<PackageInfo>, String> {
    package_manager::get_packages().await
}

#[tauri::command]
pub async fn get_components() -> Result<Vec<Component>, String> {
    package_manager::get_components().await
}

#[tauri::command]
pub async fn install_package(package_name: String) -> Result<String, String> {
    package_manager::install_package(package_name).await
}

#[tauri::command]
pub async fn remove_package(package_name: String) -> Result<String, String> {
    package_manager::remove_package(package_name).await
}

#[tauri::command]
pub async fn update_package(package_name: String) -> Result<String, String> {
    package_manager::update_package(package_name).await
}

#[tauri::command]
pub async fn search_packages(query: String) -> Result<Vec<PackageInfo>, String> {
    package_manager::search_packages(query).await
}

#[tauri::command]
pub async fn get_installed_packages() -> Result<Vec<String>, String> {
    package_manager::get_installed_packages().await
}