import { createContext, useContext, useRef } from 'react';

import type { PropsWithChildren } from 'react';

import { pluginManager } from '~/plugins';

export declare type PluginContextValue = {
    manager: typeof pluginManager;
    setState: (pluginName: string, state: Record<string, any>) => void;
    getState: (pluginName: string) => Record<string, any> | undefined;
};

const PluginContext = createContext<PluginContextValue>({
    manager: pluginManager,
    setState: () => {},
    getState: () => {
        return {};
    },
});

export function usePlugin() {
    return useContext(PluginContext);
}

export function PluginProvider({ children }: PropsWithChildren<{}>) {
    const manager = useRef(pluginManager);

    const providerValue: PluginContextValue = {
        manager: manager.current,
        setState: (pluginName: string, state: Record<string, any>) => {
            manager.current.setState(pluginName, state);
        },
        getState: (pluginName: string) => {
            return manager.current.getState(pluginName);
        },
    };

    return <PluginContext.Provider value={providerValue}>{children}</PluginContext.Provider>;
}
