import { createBrowserRouter } from 'react-router-dom';

import { DefaultLayout } from '~/layouts';
import { SettingsPage } from '~/pages/settings';

import { WelcomePage } from './welcome';

export const router = createBrowserRouter([
    {
        element: <DefaultLayout />,
        children: [
            {
                path: '/',
                element: <WelcomePage />,
            },
            {
                path: '/settings',
                element: <SettingsPage />,
            },
        ],
    },
]);
