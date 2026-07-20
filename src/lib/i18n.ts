import type { Translations } from '../commands/index';

export type I18n = {
    t: (key: string, params?: Record<string, string | number>) => string;
    locale: string;
};

export function createI18n(translations: Translations, locale: string): I18n {
    return {
        t(key: string, params?: Record<string, string | number>): string {
            const [group, ...rest] = key.split('.');
            const name = rest.join('.');
            const dict = translations[group as keyof Translations];
            let value = dict?.[name] ?? key;

            if (params) {
                for (const [k, v] of Object.entries(params)) {
                    value = value.replace(new RegExp(`:${k}`, 'g'), String(v));
                }
            }

            return value;
        },
        locale,
    };
}
