import { invoke } from '@tauri-apps/api/core';

import { Button } from '~/components/ui/button';

export function WelcomePage() {
    const testClipboard = async () => {
        invoke('get_clipboard_text').then(console.log).catch(console.error);
        invoke('get_clipboard_html').then(console.log).catch(console.error);
        invoke('get_clipboard_rtf').then(console.log).catch(console.error);
        invoke('get_clipboard_image').then(console.log).catch(console.error);
        invoke('get_clipboard_files').then(console.log).catch(console.error);
    };

    return (
        <div>
            <Button onClick={testClipboard}>test clipboard</Button>
        </div>
    );
}
