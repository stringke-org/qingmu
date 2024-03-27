import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';

import react from '@vitejs/plugin-react';
import Icons from 'unplugin-icons/vite';
import { defineConfig } from 'vite';
import { comlink } from 'vite-plugin-comlink';

// https://vitejs.dev/config/
export default defineConfig(async () => ({
    plugins: [
        react(),
        Icons({
            compiler: 'jsx',
            jsx: 'react',
        }),
        comlink(),
    ],
    clearScreen: false,
    resolve: {
        alias: {
            '~': join(dirname(fileURLToPath(import.meta.url)), 'src'),
        },
    },
    worker: {
        plugins: () => [comlink()],
    },
    server: {
        port: 1420,
        strictPort: true,
        watch: {
            ignored: ['**/src-tauri/**'],
        },
    },
}));
