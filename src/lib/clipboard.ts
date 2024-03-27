import { invoke } from '@tauri-apps/api/core';

export const clipboard = {
    text: async () => {
        return invoke('get_clipboard_text');
    },
    writeText: async (text: string) => {
        return await invoke<string>('set_clipboard_text', { value: text });
    },
    rtf: async () => {
        return await invoke<string>('get_clipboard_rtf');
    },
    writeRtf: async (text: string) => {
        return await invoke<string>('set_clipboard_rtf', { value: text });
    },
    html: async () => {
        return await invoke('get_clipboard_html');
    },
    writeHtml: async (text: string) => {
        return await invoke('set_clipboard_html', { value: text });
    },
    image: async () => {
        return await invoke('get_clipboard_image');
    },
    writeImage: async (path: string) => {
        return await invoke('set_clipboard_image', { value: path });
    },
    files: async () => {
        return await invoke<string[]>('get_clipboard_files');
    },
    writeFiles: async (paths: string[]) => {
        return await invoke('set_clipboard_files', { value: paths });
    },
};

export async function getClipboard() {}
