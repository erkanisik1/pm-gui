use std::process::Command;
use crate::backend::xml_parser::{XmlParser, PackageInfo, Component};

#[tauri::command]
pub async fn get_packages() -> Result<Vec<PackageInfo>, String> {
    XmlParser::load_pisi_index().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_components() -> Result<Vec<Component>, String> {
    let packages = XmlParser::load_pisi_index().map_err(|e| e.to_string())?;
    Ok(XmlParser::parse_components(&packages))
}

#[tauri::command]
pub async fn install_package(package_name: String) -> Result<String, String> {
    let output = Command::new("pisi")
        .args(["install", "-y", &package_name])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(format!("Package {} installed successfully", package_name))
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to install package: {}", error))
    }
}

#[tauri::command]
pub async fn remove_package(package_name: String) -> Result<String, String> {
    let output = Command::new("pisi")
        .args(["remove", "-y", &package_name])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(format!("Package {} removed successfully", package_name))
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to remove package: {}", error))
    }
}

#[tauri::command]
pub async fn update_package(package_name: String) -> Result<String, String> {
    let output = Command::new("pisi")
        .args(["update", "-y", &package_name])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(format!("Package {} updated successfully", package_name))
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to update package: {}", error))
    }
}

#[tauri::command]
pub async fn search_packages(query: String) -> Result<Vec<PackageInfo>, String> {
    // Simple search in loaded packages
    let packages = XmlParser::load_pisi_index().map_err(|e| e.to_string())?;
    let filtered = packages.into_iter()
        .filter(|p| p.name.contains(&query) || p.summary.contains(&query))
        .collect();
    Ok(filtered)
}

#[tauri::command]
pub async fn get_installed_packages() -> Result<Vec<String>, String> {
    let output = Command::new("pisi")
        .args(["list-installed"])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let output_str = String::from_utf8_lossy(&output.stdout);
        let packages: Vec<String> = output_str
            .lines()
            .map(|line| line.split_whitespace().next().unwrap_or("").to_string())
            .filter(|s| !s.is_empty())
            .collect();
        Ok(packages)
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to get installed packages: {}", error))
    }
}
