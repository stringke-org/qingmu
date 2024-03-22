import { forwardRef } from 'react';

import type { MenuButtonProps } from './menu-button';

import { MenubarContent, MenubarItem, MenubarMenu, MenubarTrigger } from '~/components/ui/menubar';
import { IconLanguage } from '~/icons';
import i18n from '~/lib/i18n';
import { locales } from '~/locales';

import { MenuButton } from './menu-button';

export const LanguageSwitcherButton = forwardRef<HTMLButtonElement, MenuButtonProps>(
    function LanguageSwitcher(props, ref) {
        return (
            <MenuButton
                ref={ref}
                {...props}
            >
                <IconLanguage />
            </MenuButton>
        );
    },
);

export function LanguageSwitcher() {
    function handleLanguageChange(code: string) {
        void i18n.changeLanguage(code);
    }

    return (
        <MenubarMenu>
            <MenubarTrigger asChild={true}>
                <LanguageSwitcherButton />
            </MenubarTrigger>
            <MenubarContent>
                {locales.map((locale) => (
                    <MenubarItem
                        key={locale.code}
                        onClick={() => handleLanguageChange(locale.code)}
                    >
                        {locale.icon && (
                            <div className={'mr-2'}>
                                <locale.icon />
                            </div>
                        )}
                        {locale.name}
                    </MenubarItem>
                ))}
            </MenubarContent>
        </MenubarMenu>
    );
}
