import { definePlugin } from '~/plugins/_base';

import JsonPluginView from './view';

export default definePlugin({
    name: 'json',
    description: 'JSON Viewer',
    version: '0.1.0',
    entrypoint: JsonPluginView,
    state: {},
    i18n: {},
    worker: new ComlinkSharedWorker(new URL('./worker.ts', import.meta.url)),
});
