
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

    // Elementleri bul
    updateElementReferences();

    // Event listener'ları bağla
    initializeEventListeners();

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
        componentsList: document.getElementById('components-list'),
        settingsModal: document.getElementById('settings-modal'),
        themeToggle: document.getElementById('theme-toggle'),
        installedCount: document.getElementById('installed-count'),
        availableCount: document.getElementById('available-count'),
        updatesCount: document.getElementById('updates-count'),
        sidebarTotal: document.getElementById('sidebar-total-count'),
        sidebarInstalled: document.getElementById('sidebar-installed-count'),
        sidebarUpdates: document.getElementById('sidebar-updates-count'),
        sidebarCompTotal: document.getElementById('sidebar-comp-total-count')
    };
}

async function refreshData() {
    try {
        const stats = await invoke('get_package_stats');
        updateUIStats(stats);

        showLoading(true);
        packages = await invoke('get_packages');
        const components = await invoke('get_components');
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
    });
}

function filterAndRender() {
    const query = elements.searchInput?.value.toLowerCase().trim() || '';

    filteredPackages = packages.filter(pkg => {
        if (query && !pkg.name.toLowerCase().includes(query) && !pkg.summary.toLowerCase().includes(query)) {
            return false;
        }
        if (currentComponent !== 'all' && pkg.part_of !== currentComponent) return false;
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

function createPackageCard(pkg) {
    return `
        <div class="package-card">
            <div class="package-header">
                <div>
                    <div class="package-name">${pkg.name}</div>
                    <div class="package-version">v${pkg.version || '0.1'}</div>
                </div>
                <div class="package-icon">📦</div>
            </div>
            <div class="package-summary">${pkg.summary}</div>
            <div class="package-info">
                <span>📁 ${pkg.part_of}</span>
            </div>
        </div>
    `;
}

function selectPackage(pkg) {
    selectedPackage = pkg;
    if (elements.detailsPanel) {
        elements.detailsPanel.style.display = 'block';
        if (elements.packageDetails) {
            elements.packageDetails.innerHTML = `
                <h2>${pkg.name}</h2>
                <p>${pkg.summary}</p>
                <div class="detail-actions">
                    <button class="btn-install">📥 Kur</button>
                    <button class="btn-remove">🗑️ Kaldır</button>
                </div>
            `;
        }
    }
}

function renderComponents(components) {
    if (!elements.componentsList) return;
    elements.componentsList.innerHTML = components.map(comp => `
        <button class="component-btn ${comp.name === currentComponent ? 'active' : ''}" data-component="${comp.name}">
            ${comp.name} (${comp.package_count})
        </button>
    `).join('');

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
    if (elements.themeToggle) {
        elements.themeToggle.innerHTML = isDarkMode ? '<i class="fa fa-sun-o"></i> Light' : '<i class="fa fa-moon-o"></i> Dark';
    }
    localStorage.setItem('darkMode', isDarkMode);
}

function debounce(func, wait) {
    let timeout;
    return (...args) => {
        clearTimeout(timeout);
        timeout = setTimeout(() => func(...args), wait);
    };
}