use serde::{Serialize, Deserialize};
use anyhow::Result;
use roxmltree::Document;
use std::collections::HashMap;
use std::path::Path;
use std::fs;
use once_cell::sync::Lazy;
use std::sync::Mutex;

// RAM Önbelleği (Uygulama açıkken tekrar parse etmemek için)
static CACHED_PACKAGES: Lazy<Mutex<Option<Vec<PackageInfo>>>> = Lazy::new(|| Mutex::new(None));

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub summary: String,
    pub description: String,
    pub version: String,
    pub release: u32,
    pub license: String,
    pub part_of: String,
    pub package_size: u64,
    pub installed_size: u64,
    pub package_format: String,
    pub distribution: String,
    pub distribution_release: String,
    pub architecture: String,
    pub source: Option<Source>,
    pub history: Vec<PackageHistory>,
    pub dependencies: Vec<Dependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Source {
    pub name: String,
    pub homepage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageHistory {
    pub version: String,
    pub release: u32,
    pub date: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub version: Option<String>,
    pub release: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Component {
    pub id: String,
    pub name: String,
    pub package_count: usize,
}

pub struct XmlParser;

impl XmlParser {
    const INDEX_PATH: &'static str = "/var/lib/pisi/index/stable2/pisi-index.xml";
    const CACHE_PATH: &'static str = "/tmp/pisi-pm-index-cache.bin";

    pub fn load_pisi_index() -> Result<Vec<PackageInfo>> {
        // 1. Durum: RAM'de var mı?
        {
            let cache = CACHED_PACKAGES.lock().unwrap();
            if let Some(packages) = &*cache {
                return Ok(packages.clone());
            }
        }

        // 2. Durum: Disk önbelleği (Binary Cache) kontrolü
        if let Some(cached) = Self::load_from_binary_cache()? {
            let mut cache = CACHED_PACKAGES.lock().unwrap();
            *cache = Some(cached.clone());
            return Ok(cached);
        }

        // 3. Durum: Hiçbiri yoksa XML'den parse et
        println!("No valid cache found. Parsing XML from: {}", Self::INDEX_PATH);
        let xml_content = fs::read_to_string(Self::INDEX_PATH)
            .map_err(|e| anyhow::anyhow!("Failed to read Pisi index file: {}", e))?;
        
        let packages = Self::parse_pisi_index(&xml_content)?;
        
        // Diske kaydet
        Self::save_to_binary_cache(&packages)?;
        
        // RAM'e kaydet
        let mut cache = CACHED_PACKAGES.lock().unwrap();
        *cache = Some(packages.clone());
        
        Ok(packages)
    }

    fn load_from_binary_cache() -> Result<Option<Vec<PackageInfo>>> {
        let xml_path = Path::new(Self::INDEX_PATH);
        let cache_path = Path::new(Self::CACHE_PATH);

        if !xml_path.exists() || !cache_path.exists() {
            return Ok(None);
        }

        // Tarih kontrolü: Eğer XML, Cache'den daha yeniyse geçersizdir
        let xml_mtime = fs::metadata(xml_path)?.modified()?;
        let cache_mtime = fs::metadata(cache_path)?.modified()?;

        if xml_mtime > cache_mtime {
            println!("XML index updated, cache is stale.");
            return Ok(None);
        }

        // Binary cache'i oku
        println!("Loading packages from binary cache: {}", Self::CACHE_PATH);
        let bytes = fs::read(cache_path)?;
        let packages: Vec<PackageInfo> = bincode::deserialize(&bytes)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize cache: {}", e))?;

        Ok(Some(packages))
    }

    fn save_to_binary_cache(packages: &[PackageInfo]) -> Result<()> {
        println!("Saving packages to binary cache...");
        let bytes = bincode::serialize(packages)
            .map_err(|e| anyhow::anyhow!("Failed to serialize cache: {}", e))?;
        
        fs::write(Self::CACHE_PATH, bytes)
            .map_err(|e| anyhow::anyhow!("Failed to write cache file: {}", e))?;
        
        Ok(())
    }

    pub fn parse_pisi_index(xml_content: &str) -> Result<Vec<PackageInfo>> {
        let doc = Document::parse(xml_content)?;
        let mut packages = Vec::new();

        // Tüm <Package> tag'lerini bul, ancak <Obsoletes> içindekileri atla
        for node in doc.descendants().filter(|n| n.has_tag_name("Package")) {
            // Eğer bu paket <Obsoletes> içindeyse atla
            if Self::is_in_obsoletes(&node) {
                continue;
            }

            let package_name = Self::get_text(&node, "Name")
                .unwrap_or_else(|| "Unknown".to_string());
            
            let part_of = Self::get_text(&node, "PartOf").unwrap_or_else(|| "system".to_string());
            let version = Self::get_text(&node, "Version").unwrap_or_default();
            
            let package_size = Self::get_text(&node, "PackageSize")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);
                
            let installed_size = Self::get_text(&node, "InstalledSize")
                .and_then(|s| s.parse().ok())
                .unwrap_or(0);

            let package = PackageInfo {
                name: package_name.clone(),
                summary: Self::get_text(&node, "Summary").unwrap_or_default(),
                description: Self::get_text(&node, "Description").unwrap_or_default(),
                version: version.clone(),
                release: 1,
                license: Self::get_text(&node, "License").unwrap_or_default(),
                part_of: part_of.clone(),
                package_size,
                installed_size,
                package_format: Self::get_text(&node, "PackageFormat").unwrap_or_else(|| "1.2".to_string()),
                distribution: Self::get_text(&node, "Distribution").unwrap_or_else(|| "PisiLinux".to_string()),
                distribution_release: Self::get_text(&node, "DistributionRelease").unwrap_or_else(|| "2.0".to_string()),
                architecture: Self::get_text(&node, "Architecture").unwrap_or_else(|| "x86_64".to_string()),
                source: Self::parse_source(&node),
                history: Self::parse_history(&node),
                dependencies: Self::parse_dependencies(&node),
            };
            
            // Sadece geçerli paketleri ekle (isim ve versiyonu olan)
            if package_name != "Unknown" && !version.is_empty() {
                packages.push(package);
            }
        }

        println!("Successfully parsed {} valid packages from Pisi index", packages.len());
        Ok(packages)
    }

    /// <Obsoletes> tag'i içindeki paketleri kontrol et
    fn is_in_obsoletes(node: &roxmltree::Node) -> bool {
        let mut current = node.parent();
        while let Some(parent) = current {
            if parent.has_tag_name("Obsoletes") {
                return true;
            }
            current = parent.parent();
        }
        false
    }

    fn get_text(node: &roxmltree::Node, tag_name: &str) -> Option<String> {
        // Önce direk child'larda ara
        for child in node.children() {
            if child.is_element() && child.tag_name().name() == tag_name {
                return child.text().map(|s| s.trim().to_string());
            }
        }
        
        // Sonra tüm descendant'larda ara
        node.descendants()
            .find(|n| n.has_tag_name(tag_name))
            .and_then(|n| n.text())
            .map(|s| s.trim().to_string())
    }

    /// Paketlerin içinde bulunduğu bileşenleri (Category/Component) ve sayılarını döner
    pub fn parse_components(packages: &[PackageInfo]) -> Vec<Component> {
        let mut component_counts: HashMap<String, usize> = HashMap::new();
        
        for package in packages {
            let component_id = package.part_of.clone();
            *component_counts.entry(component_id).or_insert(0) += 1;
        }

        let mut components: Vec<Component> = component_counts
            .into_iter()
            .map(|(id, count)| {
                let display_name = Self::format_component_name(&id);
                Component {
                    id: id.clone(),
                    name: display_name,
                    package_count: count,
                }
            })
            .collect();

        // "all" component'ını ekle
        let total_packages = packages.len();
        components.insert(0, Component {
            id: "all".to_string(),
            name: "All".to_string(),
            package_count: total_packages,
        });

        // Component'leri alfabetik sırala (isime görey)
        components.sort_by(|a, b| a.name.cmp(&b.name));
        
        components
    }

    /// Component isimlerini daha okunabilir hale getir
    fn format_component_name(raw_name: &str) -> String {
        match raw_name {
            "programming.devel" => "Programming - Development".to_string(),
            "programming.language.python3" => "Programming - Python 3".to_string(),
            "programming.docs" => "Programming - Documentation".to_string(),
            "system.locale" => "System - Localization".to_string(),
            "programming.language.perl" => "Programming - Perl".to_string(),
            "office.libreoffice" => "Office - LibreOffice".to_string(),
            "system.devel" => "System - Development".to_string(),
            "programming.library" => "Programming - Libraries".to_string(),
            "system.base" => "System - Base".to_string(),
            "desktop.kde.applications" => "Desktop - KDE Applications".to_string(),
            "multimedia.sound" => "Multimedia - Sound".to_string(),
            "x11.library" => "X11 - Libraries".to_string(),
            "desktop.kde5.framework" => "Desktop - KDE5 Framework".to_string(),
            "desktop.kde.framework" => "Desktop - KDE Framework".to_string(),
            "office.misc" => "Office - Miscellaneous".to_string(),
            "desktop.misc" => "Desktop - Miscellaneous".to_string(),
            "multimedia.misc" => "Multimedia - Miscellaneous".to_string(),
            "programming.misc" => "Programming - Miscellaneous".to_string(),
            "emul32" => "Emulation - 32-bit".to_string(),
            "multimedia.graphics.gimp.l10n" => "Multimedia - GIMP Localization".to_string(),
            _ => {
                // Genel formatlama
                raw_name
                    .replace('.', " - ")
                    .replace("_", " ")
                    .split(' ')
                    .map(|word| {
                        let mut chars = word.chars();
                        match chars.next() {
                            None => String::new(),
                            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                        }
                    })
                    .collect::<Vec<String>>()
                    .join(" ")
            }
        }
    }

    fn parse_source(node: &roxmltree::Node) -> Option<Source> {
        if let Some(source_node) = node.descendants().find(|n| n.has_tag_name("Source")) {
            let name = source_node.attribute("name").unwrap_or("").to_string();
            let homepage = source_node.attribute("homepage").unwrap_or("").to_string();
            
            if !name.is_empty() {
                return Some(Source { name, homepage });
            }
        }
        None
    }

    fn parse_history(node: &roxmltree::Node) -> Vec<PackageHistory> {
        let mut history = Vec::new();
        
        if let Some(history_node) = node.descendants().find(|n| n.has_tag_name("History")) {
            for update_node in history_node.descendants().filter(|n| n.has_tag_name("Update")) {
                let version = update_node.attribute("version").unwrap_or("").to_string();
                let release = update_node.attribute("release")
                    .and_then(|s| s.parse().ok())
                    .unwrap_or(1);
                let date = update_node.attribute("date").unwrap_or("").to_string();
                
                history.push(PackageHistory { version, release, date });
            }
        }
        
        history
    }

    fn parse_dependencies(node: &roxmltree::Node) -> Vec<Dependency> {
        let mut deps = Vec::new();
        
        if let Some(deps_node) = node.descendants().find(|n| n.has_tag_name("RuntimeDependencies")) {
            for package_node in deps_node.descendants().filter(|n| n.has_tag_name("Package")) {
                let name = package_node.text().unwrap_or("").trim().to_string();
                let version = package_node.attribute("version").map(|s| s.to_string());
                let release = package_node.attribute("release").and_then(|s| s.parse().ok());
                
                if !name.is_empty() {
                    deps.push(Dependency { name, version, release });
                }
            }
        }
        
        deps
    }
}
