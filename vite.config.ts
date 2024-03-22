import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

import react from '@vitejs/plugin-react';
import Icons from 'unplugin-icons/vite';
import { defineConfig } from 'vite';

// https://vitejs.dev/config/
export default defineConfig(async () => ({
    plugins: [
        react(),
        Icons({
            compiler: 'jsx',
            jsx: 'react',
        }),
    ],
    clearScreen: false,
    resolve: {
        alias: {
            '~': join(dirname(fileURLToPath(import.meta.url)), 'src'),
        },
    },
    server: {
        port: 1420,
        strictPort: true,
        watch: {
            ignored: ['**/src-tauri/**'],
        },
    },
}));
