import { default as IconZh } from '~icons/flag/cn-4x3';
import { default as IconEn } from '~icons/flag/um-4x3';

import { default as en } from './en';
import { default as zh } from './zh';

export const defaultNS = 'common';
export const resources = {
    en,
    zh,
} as const;

export const locales = [
    {
        code: 'en',
        name: 'English',
        icon: IconEn,
    },
    {
        code: 'zh',
        name: '简体中文',
        icon: IconZh,
    },
];
