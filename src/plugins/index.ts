import { type MethodOutput } from '~/plugins/_base';
import json from '~/plugins/json';

export const plugins = [json];

class PluginManager {
    states = new Map<string, Record<string, any>>();

    constructor() {
        plugins.forEach((plugin) => {
            this.states.set(plugin.name, plugin.state);
        });
    }

    onPaste(pastes: any, callback: (name: String, data: any) => void) {
        plugins.forEach((plugin) => {
            const handle = plugin.worker.onPaste(this.getState(plugin.name), pastes) as
                | Promise<MethodOutput<any>>
                | undefined;

            if (handle) {
                handle.then((result) => {
                    this.setState(plugin.name, result.state);
                    callback(plugin.name, result.result);
                });
            }
        });
    }

    getState(pluginName: string) {
        return this.states.get(pluginName);
    }

    setState(pluginName: string, state: Record<string, any>) {
        this.states.set(pluginName, state);
    }

    async init() {
        for (const plugin of plugins) {
            console.log('Plugin:', plugin.name);
        }
    }
}

export const pluginManager = new PluginManager();
