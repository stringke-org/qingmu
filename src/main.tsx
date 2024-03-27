import React from 'react';
import ReactDOM from 'react-dom/client';
import { RouterProvider } from 'react-router-dom';

import { router } from './pages';

import '~/assets/styles/main.scss';

import { I18nextProvider } from 'react-i18next';

import { PluginProvider, ThemeProvider } from '~/components/providers';
import i18n from '~/lib/i18n';
import { pluginManager } from '~/plugins';

async function bootstrap() {
    const rootEl = document.getElementById('root');
    if (rootEl) {
        await pluginManager.init();

        const root = ReactDOM.createRoot(rootEl);
        root.render(
            <React.StrictMode>
                <PluginProvider>
                    <I18nextProvider i18n={i18n}>
                        <ThemeProvider>
                            <RouterProvider router={router} />
                        </ThemeProvider>
                    </I18nextProvider>
                </PluginProvider>
            </React.StrictMode>,
        );
    }
}

void bootstrap();
