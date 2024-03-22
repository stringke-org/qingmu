import { IconThemeDark, IconThemeLight, IconThemeSystem } from '~/icons';
import { useTheme } from '~/lib/hooks';

import { MenuButton } from './menu-button';

export function ColorSwitcher() {
    const [theme, setTheme] = useTheme();

    function toggleColorMode() {
        setTheme(theme === 'system' ? 'dark' : theme === 'dark' ? 'light' : 'system');
    }

    return (
        <MenuButton onClick={() => toggleColorMode()}>
            {theme === 'system' ? <IconThemeSystem /> : theme === 'light' ? <IconThemeLight /> : <IconThemeDark />}
        </MenuButton>
    );
}
