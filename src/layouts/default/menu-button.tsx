import { forwardRef } from 'react';

import type { HTMLAttributes, PropsWithChildren } from 'react';

import { Button } from '~/components/ui/button';
import { cn } from '~/lib/utils';

export declare type MenuButtonProps = PropsWithChildren<HTMLAttributes<HTMLButtonElement>>;

export const MenuButton = forwardRef<HTMLButtonElement, MenuButtonProps>(function LanguageSwitcher(
    { children, className, ...props },
    ref,
) {
    return (
        <Button
            ref={ref}
            variant={'ghost'}
            size={'sm'}
            className={cn(
                'h-auto p-1 text-gray-400 hover:text-gray-700 dark:text-white dark:hover:text-gray-300',
                className,
            )}
            {...props}
        >
            {children}
        </Button>
    );
});
