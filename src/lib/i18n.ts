import i18n from 'i18next';
import LanguageDetector from 'i18next-browser-languagedetector';
import { initReactI18next } from 'react-i18next';

import { defaultNS, resources } from '~/locales';

i18n.use(LanguageDetector)
    .use(initReactI18next)
    .init({
        defaultNS,
        resources,
        lng: 'en',
        fallbackLng: 'en',
        supportedLngs: Object.keys(resources),
        interpolation: {
            escapeValue: false,
        },
    });

export default i18n;
