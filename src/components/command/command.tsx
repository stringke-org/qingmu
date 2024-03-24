import useToggle from 'beautiful-react-hooks/useToggle';
import {useRef} from 'react';
import {useTranslation} from 'react-i18next';

import {cn} from '~/lib/utils';

export function AppCommand() {
    const inputRef = useRef<HTMLInputElement>(null);
    const [isFocus, toggleFocus] = useToggle(false);
    const {t} = useTranslation('command');

    const text = `{"user":1....`;

    function onClipboard() {
        console.log(222);
    }

    function onFocus() {
        toggleFocus();
        onClipboard();

        // 监听粘贴事件
        inputRef.current?.addEventListener('paste', onClipboard);
    }

    function onBlur() {
        toggleFocus();
        inputRef.current?.removeEventListener('paste', onClipboard);
    }

    return (<div
            className={cn('max-2-[640px] flex h-8 min-w-[360px] gap-1 overflow-hidden rounded-md border bg-background px-1 text-sm transition cursor-pointer', {
                'ring-2 ring-ring ring-offset-2': isFocus,
            },)}
        >
            <div
                className={cn('m-0.5 flex gap-1 overflow-hidden rounded border transition', {
                    hidden: !isFocus,
                })}
            >
                <div className={'flex items-center justify-center rounded bg-primary p-0.5 text-xs text-white'}>
                    JSON
                </div>
                <div>{text}</div>
            </div>
            <input
                ref={inputRef}
                className={'h-full min-h-0 flex-1 outline-none placeholder:text-muted-foreground'}
                placeholder={t('placeholder')}
                onFocus={onFocus}
                onBlur={onBlur}
                autoFocus
            />
        </div>);
}
