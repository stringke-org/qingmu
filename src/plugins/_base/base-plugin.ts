import { type Component, type FC } from 'react';

export declare type PluginOptions<Worker> = {
    name: string;
    description: string;
    version: string;
    worker: Worker;
    state?: Record<string, any>;
    i18n?: Record<string, Record<string, any>>;
};

export declare type PluginInstance<Worker, Options extends PluginOptions<Worker>> = {
    options: Required<Options>;
    t: (key: string) => string;
    state: Record<string, any>;
    setState: (state: Record<string, any>) => void;
    worker: Worker;
};

export declare type WithEntrypoint<Worker, Options extends PluginOptions<Worker>> = Options & {
    entrypoint: FC<PluginInstance<Worker, Options>> | Component<PluginInstance<Worker, Options>>;
};

export function isComponent(component: any): component is Component {
    return typeof component === 'function';
}

export function definePlugin<Worker, Options extends PluginOptions<Worker>>(options: WithEntrypoint<Worker, Options>) {
    if (!options.name) throw new Error('Plugin name is required');
    if (!options.description) throw new Error('Plugin description is required');
    if (!options.version) throw new Error('Plugin version is required');
    if (!options.entrypoint) throw new Error('Plugin entrypoint is required');
    if (!isComponent(options.entrypoint)) throw new Error('Plugin entrypoint must be a React component');

    return {
        ...options,
        entrypoint: options.entrypoint,
        state: options.state || {},
        i18n: options.i18n || {},
    };
}
