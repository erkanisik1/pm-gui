use std::process::Command;
use crate::backend::xml_parser::{PackageInfo, Component, XmlParser};
use base64::{Engine as _, engine::general_purpose};

#[tauri::command]
pub async fn get_packages() -> Result<Vec<PackageInfo>, String> {
    XmlParser::load_pisi_index().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn get_components() -> Result<Vec<Component>, String> {
    XmlParser::get_components().map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn search_packages(query: String) -> Result<Vec<PackageInfo>, String> {
    let packages = XmlParser::load_pisi_index().map_err(|e| e.to_string())?;
    let query = query.to_lowercase();
    Ok(packages.into_iter()
        .filter(|p| p.name.to_lowercase().contains(&query) || p.summary.to_lowercase().contains(&query))
        .collect())
}

#[tauri::command]
pub async fn get_installed_packages() -> Result<Vec<String>, String> {
    let output = Command::new("pisi")
        .arg("li")
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let installed = stdout.lines()
            .map(|l| l.split_whitespace().next().unwrap_or("").to_string())
            .filter(|s| !s.is_empty())
            .collect();
        Ok(installed)
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("pisi li failed: {}", stderr))
    }
}

#[tauri::command]
pub async fn install_package(package_name: String) -> Result<String, String> {
    let output = Command::new("pisi")
        .args(["it", &package_name, "-y"])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(format!("Package {} installed successfully", package_name))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to install {}: {}", package_name, stderr))
    }
}

#[tauri::command]
pub async fn remove_package(package_name: String) -> Result<String, String> {
    let output = Command::new("pisi")
        .args(["rm", &package_name, "-y"])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(format!("Package {} removed successfully", package_name))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to remove {}: {}", package_name, stderr))
    }
}

#[tauri::command]
pub async fn update_package(package_name: String) -> Result<String, String> {
    let output = Command::new("pisi")
        .args(["up", &package_name, "-y"])
        .output()
        .map_err(|e| e.to_string())?;

    if output.status.success() {
        Ok(format!("Package {} updated successfully", package_name))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(format!("Failed to update {}: {}", package_name, stderr))
    }
}

#[tauri::command]
pub async fn get_upgradable_packages() -> Result<Vec<String>, String> {
    let output = Command::new("pisi")
        .args(["list-upgrades"])
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
        Err(format!("Failed to get upgradable packages: {}", error))
    }
}

#[tauri::command]
pub async fn update_repo() -> Result<(), String> {
    let output = Command::new("pisi")
        .args(["ur"])
        .output()
        .map_err(|e| e.to_string())?;

    if !output.status.success() {
        let err = String::from_utf8_lossy(&output.stderr);
        return Err(format!("Pisi update-repo failed: {}", err));
    }
    Ok(())
}

#[tauri::command]
pub async fn get_package_icon(icon_name: String) -> Result<String, String> {
    if icon_name.is_empty() {
        return Err("Icon name is empty".to_string());
    }

    let paths = [
        format!("/usr/share/icons/hicolor/48x48/apps/{}.png", icon_name),
        format!("/usr/share/icons/hicolor/32x32/apps/{}.png", icon_name),
        format!("/usr/share/pixmaps/{}.png", icon_name),
    ];

    for path_str in paths {
        let path = std::path::Path::new(&path_str);
        if path.exists() {
            let data = std::fs::read(path).map_err(|e| e.to_string())?;
            let b64 = general_purpose::STANDARD.encode(data);
            return Ok(format!("data:image/png;base64,{}", b64));
        }
    }

    Err("Icon not found".to_string())
}
