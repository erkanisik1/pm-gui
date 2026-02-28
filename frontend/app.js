import { i18n } from './js/i18n.js';

// Tauri API'nin yüklenmesini bekle
let invoke;

function waitForTauriAPI() {
    return new Promise((resolve) => {
        if (window.__TAURI__?.core?.invoke) {
            invoke = window.__TAURI__.core.invoke;
            resolve();
        } else {
            let attempts = 0;
            const maxAttempts = 50;

            const checkInterval = setInterval(() => {
                attempts++;
                if (window.__TAURI__?.core?.invoke) {
                    invoke = window.__TAURI__.core.invoke;
                    clearInterval(checkInterval);
                    resolve();
                } else if (attempts >= maxAttempts) {
                    clearInterval(checkInterval);
                    setupMockInvoke();
                    resolve();
                }
            }, 100);
        }
    });
}

function setupMockInvoke() {
    console.warn('Tauri API yüklenemedi, mock kullanılacak');
    invoke = async (command, args) => {
        console.log(`Mock çağrı: ${command}`, args);
        switch (command) {
            case 'get_package_stats':
                return { total_count: 5, installed_count: 0, available_count: 5, updates_count: 1 };
            case 'get_packages':
                return [{ name: 'firefox', summary: 'Mozilla Firefox', part_of: 'desktop.web', package_size: 97000000 }];
            case 'get_components':
                return [{ name: 'All', package_count: 1 }, { name: 'desktop.web', package_count: 1 }];
            default:
                return `${command} komutu başarıyla çalıştırıldı`;
        }
    };
}

// Global state
let packages = [];
let installedPackageNames = [];
let upgradablePackageNames = [];
let filteredPackages = [];
let selectedPackage = null;
let currentCategory = 'all';
let currentComponent = 'all';
let currentFilter = 'all';
let isDarkMode = localStorage.getItem('darkMode') === 'true';

// Elements
let elements = {};

// Component Yükleyici
async function loadComponent(path) {
    const response = await fetch(path);
    return await response.text();
}

// Initialize app
document.addEventListener('DOMContentLoaded', async () => {
    await waitForTauriAPI();

    // Dil desteğini başlat
    await i18n.init();

    // Layout ve Bileşenleri Yükle
    const appEl = document.getElementById('app');
    const layoutHTML = await loadComponent('components/layout.html');
    appEl.innerHTML = layoutHTML;

    const headerHTML = await loadComponent('components/header.html');
    document.getElementById('header-wrapper').innerHTML = headerHTML;

    const sidebarHTML = await loadComponent('components/sidebar.html');
    document.getElementById('sidebar-wrapper').innerHTML = sidebarHTML;

    const mainViewHTML = await loadComponent('components/main-view.html');
    document.getElementById('view-content').innerHTML = mainViewHTML;

    if (isDarkMode) document.body.classList.add('dark');

    // Çevirileri ilk render sonrası uygula
    i18n.applyTranslations();

    // Elementleri bul
    updateElementReferences();

    // Event listener'ları bağla
    initializeEventListeners();

    // Veriyi yüklemeden önce depoları güncelle
    console.log('Updating repositories...');
    try {
        await invoke('update_repo');
        console.log('Repositories updated successfully.');
    } catch (e) {
        console.error('Failed to update repositories:', e);
    }

    // Veriyi yükle
    await refreshData();
});

function updateElementReferences() {
    elements = {
        packagesGrid: document.getElementById('packages-grid'),
        loading: document.getElementById('loading'),
        emptyState: document.getElementById('empty-state'),
        searchInput: document.getElementById('search-input'),
        packageDetails: document.getElementById('package-details'),
        detailsPanel: document.getElementById('details-panel'),
        mainContainer: document.querySelector('.main-container'),
        componentsList: document.getElementById('components-list'),
        settingsModal: document.getElementById('settings-modal'),
        themeToggle: document.getElementById('theme-toggle'),
        installedCount: document.getElementById('installed-count'),
        availableCount: document.getElementById('available-count'),
        updatesCount: document.getElementById('updates-count'),
        sidebarTotal: document.getElementById('sidebar-total-count'),
        sidebarInstalled: document.getElementById('sidebar-installed-count'),
        sidebarUpdates: document.getElementById('sidebar-updates-count'),
        sidebarCompTotal: document.getElementById('sidebar-comp-total-count'),
        langSelect: document.getElementById('lang-select')
    };

    if (elements.langSelect) {
        elements.langSelect.value = i18n.currentLang;
    }
}

async function refreshData() {
    try {
        showLoading(true);

        // Verileri paralel olarak çek
        const [pkgs, installed, upgradable, components] = await Promise.all([
            invoke('get_packages'),
            invoke('get_installed_packages'),
            invoke('get_upgradable_packages'),
            invoke('get_components')
        ]);

        packages = pkgs;
        installedPackageNames = installed;
        upgradablePackageNames = upgradable;

        console.log(`Data loaded: ${packages.length} total, ${installed.length} installed, ${upgradable.length} updates`);

        // İstatistikleri yerel veriden güncelle
        const stats = {
            total_count: packages.length,
            installed_count: installed.length,
            available_count: Math.max(0, packages.length - installed.length),
            updates_count: upgradable.length
        };
        updateUIStats(stats);

        renderComponents(components);
        filterAndRender();
    } catch (error) {
        console.error('Data refresh failed:', error);
    } finally {
        showLoading(false);
    }
}

function updateUIStats(stats) {
    if (elements.installedCount) elements.installedCount.textContent = stats.installed_count;
    if (elements.availableCount) elements.availableCount.textContent = stats.available_count;
    if (elements.updatesCount) elements.updatesCount.textContent = stats.updates_count;

    if (elements.sidebarTotal) elements.sidebarTotal.textContent = stats.total_count;
    if (elements.sidebarInstalled) elements.sidebarInstalled.textContent = stats.installed_count;
    if (elements.sidebarUpdates) elements.sidebarUpdates.textContent = stats.updates_count;
    if (elements.sidebarCompTotal) elements.sidebarCompTotal.textContent = stats.total_count;
}

function initializeEventListeners() {
    elements.themeToggle?.addEventListener('click', toggleTheme);

    // Dil seçimi
    elements.langSelect?.addEventListener('change', async (e) => {
        await i18n.setLanguage(e.target.value);
    });

    // Dil değiştiğinde UI'ı güncelle
    document.addEventListener('langChanged', () => {
        updateThemeToggleUI();
        // Bileşenleri tekrar render etmek gerekebilir veya sadece statik metinleri güncelle
        i18n.applyTranslations();
    });

    document.getElementById('settings-btn')?.addEventListener('click', () => {
        if (elements.settingsModal) elements.settingsModal.style.display = 'block';
    });

    document.querySelector('.modal-close')?.addEventListener('click', () => {
        if (elements.settingsModal) elements.settingsModal.style.display = 'none';
    });

    elements.searchInput?.addEventListener('input', debounce(() => filterAndRender(), 300));

    document.querySelectorAll('.category-btn').forEach(btn => {
        btn.addEventListener('click', (e) => {
            document.querySelectorAll('.category-btn').forEach(b => b.classList.remove('active'));
            e.currentTarget.classList.add('active');
            currentCategory = e.currentTarget.dataset.category;
            filterAndRender();
        });
    });

    document.querySelectorAll('.filter-btn').forEach(btn => {
        btn.addEventListener('click', (e) => {
            document.querySelectorAll('.filter-btn').forEach(b => b.classList.remove('active'));
            e.currentTarget.classList.add('active');
            currentFilter = e.currentTarget.dataset.filter;
            filterAndRender();
        });
    });

    document.getElementById('close-details')?.addEventListener('click', () => {
        if (elements.detailsPanel) elements.detailsPanel.style.display = 'none';
        elements.mainContainer?.classList.remove('has-details');
    });
}

function filterAndRender() {
    const query = elements.searchInput?.value.toLowerCase().trim() || '';

    filteredPackages = packages.filter(pkg => {
        // Devel paketlerini gizle (Eğer kullanıcı aramıyorsa)
        if (!query && (pkg.name.toLowerCase().includes('devel') || pkg.part_of.toLowerCase().includes('devel'))) {
            return false;
        }

        // Arama sorgusu filtresi
        if (query && !pkg.name.toLowerCase().includes(query) && !pkg.summary.toLowerCase().includes(query)) {
            return false;
        }

        // Bileşen (Component) filtresi
        if (currentComponent !== 'all' && pkg.part_of !== currentComponent) {
            return false;
        }

        // Kategori / Filtre mantığı (Sidebar ve Üst Filtre Butonları)
        const filterStr = currentCategory !== 'all' ? currentCategory : currentFilter;

        if (filterStr === 'installed') {
            return installedPackageNames.includes(pkg.name);
        } else if (filterStr === 'updates') {
            return upgradablePackageNames.includes(pkg.name);
        } else if (filterStr === 'available') {
            return !installedPackageNames.includes(pkg.name);
        }

        return true;
    });

    renderPackages();
}

function renderPackages() {
    if (!elements.packagesGrid) return;

    if (filteredPackages.length === 0) {
        elements.packagesGrid.style.display = 'none';
        if (elements.emptyState) elements.emptyState.style.display = 'block';
        return;
    }

    elements.packagesGrid.style.display = 'grid';
    if (elements.emptyState) elements.emptyState.style.display = 'none';

    elements.packagesGrid.innerHTML = filteredPackages.map(pkg => createPackageCard(pkg)).join('');

    elements.packagesGrid.querySelectorAll('.package-card').forEach((card, idx) => {
        card.addEventListener('click', () => selectPackage(filteredPackages[idx]));
    });
}

function getPackageIcon(partOf) {
    if (!partOf) return 'assets/icons/package.png';
    const p = partOf.toLowerCase();

    if (p.includes('office')) return 'assets/icons/office.png';
    if (p.includes('multimedia.sound') || p.includes('audio')) return 'assets/icons/multimedia-audio.png';
    if (p.includes('multimedia.video') || p.includes('video')) return 'assets/icons/multimedia-video.png';
    if (p.includes('multimedia.graphics') || p.includes('image')) return 'assets/icons/multimedia-photo.png';
    if (p.includes('web') || p.includes('internet')) return 'assets/icons/web-browser.png';
    if (p.includes('programming') || p.includes('devel')) return 'assets/icons/programming.png';
    if (p.includes('system.base') || p.includes('kernel')) return 'assets/icons/system-core.png';
    if (p.includes('desktop.kde') || p.includes('desktop.plasma')) return 'assets/icons/kde.png';
    if (p.includes('game')) return 'assets/icons/games.png';
    if (p.includes('security')) return 'assets/icons/security.png';
    if (p.includes('network')) return 'assets/icons/network.png';

    return 'assets/icons/package.png';
}

function createPackageCard(pkg) {
    const icon = getPackageIcon(pkg.part_of);
    return `
        <div class="package-card">
            <div class="package-header">
                <div>
                    <div class="package-name">${pkg.name}</div>
                    <div class="package-version">v${pkg.version || '0.1'}</div>
                </div>
                <div class="package-icon"><img src="${icon}" alt="${pkg.name}" onerror="this.src='assets/icons/package.png'"></div>
            </div>
            <div class="package-summary">${pkg.summary}</div>
            <div class="package-info">
                <span class="package-category">${pkg.part_of}</span>
            </div>
        </div>
    `;
}

function selectPackage(pkg) {
    selectedPackage = pkg;
    if (elements.detailsPanel) {
        elements.detailsPanel.style.display = 'block';
        elements.mainContainer?.classList.add('has-details');
        if (elements.packageDetails) {
            const isInstalled = installedPackageNames.includes(pkg.name);
            const hasUpdate = upgradablePackageNames.includes(pkg.name);

            elements.packageDetails.innerHTML = `
                <h2>${pkg.name}</h2>
                <div class="package-meta">
                    <span><strong>${i18n.t('version')}:</strong> ${pkg.version}</span>
                    <span><strong>${i18n.t('category')}:</strong> ${pkg.part_of}</span>
                </div>
                <p class="package-description">${pkg.description || pkg.summary}</p>
                <div class="detail-actions">
                    ${!isInstalled ? `<button class="btn-install" id="action-install">📥 ${i18n.t('install')}</button>` : ''}
                    ${isInstalled ? `<button class="btn-remove" id="action-remove">🗑️ ${i18n.t('remove')}</button>` : ''}
                    ${hasUpdate ? `<button class="btn-update" id="action-update">🔄 ${i18n.t('update')}</button>` : ''}
                </div>
            `;

            // Butonlara event listener ekle
            document.getElementById('action-install')?.addEventListener('click', async () => {
                try {
                    showLoading(true);
                    await invoke('install_package', { packageName: pkg.name });
                    await refreshData();
                } catch (e) { alert(e); }
                finally { showLoading(false); }
            });

            document.getElementById('action-remove')?.addEventListener('click', async () => {
                try {
                    showLoading(true);
                    await invoke('remove_package', { packageName: pkg.name });
                    await refreshData();
                } catch (e) { alert(e); }
                finally { showLoading(false); }
            });

            document.getElementById('action-update')?.addEventListener('click', async () => {
                try {
                    showLoading(true);
                    await invoke('update_package', { packageName: pkg.name });
                    await refreshData();
                } catch (e) { alert(e); }
                finally { showLoading(false); }
            });
        }
    }
}

function renderComponents(components) {
    if (!elements.componentsList) return;

    const allText = i18n.t('all');
    elements.componentsList.innerHTML = components.map(comp => {
        // "all" id'li bileşen için "Tümü" çevirisini kullan, diğerleri için name'i kullan
        const displayName = comp.id === 'all' ? allText : comp.name;

        return `
            <button class="component-btn ${comp.id === currentComponent ? 'active' : ''}" 
            data-component="${comp.id}">
                ${displayName} (${comp.package_count})
            </button>
        `;
    }).join('');

    document.querySelectorAll('.component-btn').forEach(btn => {
        btn.addEventListener('click', (e) => {
            document.querySelectorAll('.component-btn').forEach(b => b.classList.remove('active'));
            e.currentTarget.classList.add('active');
            currentComponent = e.currentTarget.dataset.component;
            filterAndRender();
        });
    });
}

function showLoading(show) {
    if (elements.loading) elements.loading.style.display = show ? 'flex' : 'none';
    if (elements.packagesGrid) elements.packagesGrid.style.display = show ? 'none' : 'grid';
}

function toggleTheme() {
    isDarkMode = !isDarkMode;
    document.body.classList.toggle('dark');
    updateThemeToggleUI();
    localStorage.setItem('darkMode', isDarkMode);
}

function updateThemeToggleUI() {
    if (elements.themeToggle) {
        const textKey = isDarkMode ? 'theme_light' : 'theme_dark';
        const icon = isDarkMode ? 'fa-sun-o' : 'fa-moon-o';
        elements.themeToggle.innerHTML = `<i class="fa ${icon}"></i> <span data-i18n="${textKey}">${i18n.t(textKey)}</span>`;
    }
}

function debounce(func, wait) {
    let timeout;
    return (...args) => {
        clearTimeout(timeout);
        timeout = setTimeout(() => func(...args), wait);
    };
}