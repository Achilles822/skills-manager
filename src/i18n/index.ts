import { createI18n } from 'vue-i18n'
import zhCN from './zh-CN'
import en from './en'

const LOCALE_KEY = 'skills-manager-locale'

function getSavedLocale(): string {
  try {
    return localStorage.getItem(LOCALE_KEY) || 'zh-CN'
  } catch {
    return 'zh-CN'
  }
}

export function saveLocale(locale: string) {
  try {
    localStorage.setItem(LOCALE_KEY, locale)
  } catch { /* noop */ }
}

const i18n = createI18n({
  legacy: false,
  locale: getSavedLocale(),
  fallbackLocale: 'zh-CN',
  messages: {
    'zh-CN': zhCN,
    en,
  },
})

export default i18n
