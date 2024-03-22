/* eslint-disable @typescript-eslint/consistent-type-definitions */

import { type defaultNS, type resources } from '~/locales';

declare module 'i18next' {
    interface CustomTypeOptions {
        defaultNS: typeof defaultNS;
        resources: (typeof resources)['en'];
    }
}

export {};
