
export class I18n {
    constructor() {
        this.locales = {};
        this.currentLang = localStorage.getItem('lang') ||
            (navigator.language.startsWith('tr') ? 'tr' : 'en');
    }

    async init() {
        await this.loadLang(this.currentLang);
        this.applyTranslations();
    }

    async loadLang(lang) {
        try {
            const response = await fetch(`locales/${lang}.json`);
            this.locales[lang] = await response.json();
            this.currentLang = lang;
            localStorage.setItem('lang', lang);
        } catch (error) {
            console.error(`Could not load locale ${lang}:`, error);
        }
    }

    t(key) {
        return this.locales[this.currentLang]?.[key] || key;
    }

    applyTranslations() {
        document.querySelectorAll('[data-i18n]').forEach(el => {
            const key = el.getAttribute('data-i18n');
            const translation = this.t(key);

            if (el.tagName === 'INPUT' && el.placeholder) {
                el.placeholder = translation;
            } else {
                // Eğer içinde başka bir element (ikon vb.) yoksa direkt text değiştir
                if (el.children.length === 0) {
                    el.textContent = translation;
                } else {
                    // İkonlu button vb. için sadece text node'u bul ve değiştir
                    for (let node of el.childNodes) {
                        if (node.nodeType === Node.TEXT_NODE && node.textContent.trim() !== "") {
                            node.textContent = translation;
                            break;
                        }
                    }
                }
            }
        });
    }

    async setLanguage(lang) {
        await this.loadLang(lang);
        this.applyTranslations();
        // Custom event yayınla ki app.js veya diğer kısımlar haberdar olsun
        document.dispatchEvent(new CustomEvent('langChanged', { detail: lang }));
    }
}

export const i18n = new I18n();
