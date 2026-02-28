// src-tauri/src/commands.rs
use serde::{Serialize, Deserialize};
use pm_core::backend::package_manager;
use pm_core::backend::xml_parser::{PackageInfo, Component};

#[derive(Serialize, Deserialize, Debug)]
pub struct PackageStats {
    total_count: usize,
    installed_count: usize,
    available_count: usize,
    updates_count: usize,
}

#[tauri::command]
pub async fn get_package_stats() -> Result<PackageStats, String> {
    Ok(PackageStats {
        total_count: 6501,
        installed_count: 3127,
        available_count: 3374,
        updates_count: 15,
    })
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