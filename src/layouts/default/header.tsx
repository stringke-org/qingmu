import { Translation } from 'react-i18next';

import { AppCommand } from '~/components/command';
import { Button } from '~/components/ui/button';
import {
    Menubar,
    MenubarContent,
    MenubarItem,
    MenubarMenu,
    MenubarSeparator,
    MenubarShortcut,
    MenubarTrigger,
} from '~/components/ui/menubar';
import { IconClose, IconReload } from '~/icons';
import { LanguageSwitcher } from '~/layouts/default/language-switcher';

import { ColorSwitcher } from './color-switcher';
import { MenuButton } from './menu-button';
import { SettingsButton } from './settings-button';

export function AppHeader() {
    return (
        <div className={'relative w-full border-b py-0.5'}>
            <Menubar className={'app-header relative z-10 border-none '}>
                <div
                    data-tauri-drag-region='true'
                    className={'drag-handle absolute left-0 top-0 h-full w-full touch-none'}
                />
                <MenubarMenu>
                    <MenubarTrigger>
                        <Translation ns={'common'}>{($t) => $t('title')}</Translation>
                    </MenubarTrigger>
                    <MenubarContent>
                        <MenubarItem>
                            <Translation ns={'menu'}>{($t) => $t('app.about')}</Translation>
                        </MenubarItem>
                        <MenubarItem>
                            <Translation ns={'menu'}>{($t) => $t('app.check-for-updates')}</Translation>
                        </MenubarItem>
                        <MenubarSeparator />
                        <MenubarItem>
                            <Translation ns={'menu'}>{($t) => $t('app.settings')}</Translation>
                            <MenubarShortcut>Ctrl+,</MenubarShortcut>
                        </MenubarItem>
                        <MenubarSeparator />
                        <MenubarItem>
                            <Translation ns={'menu'}>{($t) => $t('app.quit')}</Translation>
                        </MenubarItem>
                    </MenubarContent>
                </MenubarMenu>
                <Button
                    variant={'ghost'}
                    size={'sm'}
                >
                    <Translation ns={'menu'}>{($t) => $t('pluginMarket')}</Translation>
                </Button>
                <div className={'!mx-auto'}>
                    <AppCommand />
                </div>
                <SettingsButton />
                <LanguageSwitcher />
                <ColorSwitcher />
                <MenuButton>
                    <IconReload />
                </MenuButton>
                <Button
                    variant={'ghost'}
                    size={'sm'}
                    className={'h-auto p-1 text-destructive'}
                >
                    <IconClose />
                </Button>
            </Menubar>
        </div>
    );
}
