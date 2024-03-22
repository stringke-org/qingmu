import { useContext } from 'react';

import { ThemeProviderContext } from '~/components/providers';

export const useTheme = () => {
    const context = useContext(ThemeProviderContext);

    if (context === undefined) throw new Error('useTheme must be used within a ThemeProvider');
    const { theme, setTheme } = context;
    return [theme, setTheme] as const;
};
