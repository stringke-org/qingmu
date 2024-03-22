import { Outlet } from 'react-router-dom';

import { AppHeader } from './header';

export function DefaultLayout() {
    return (
        <div className={'relative h-full w-full overflow-hidden rounded-lg border bg-background'}>
            <AppHeader />
            <div className={'h-min-0 relative w-full flex-1'}>
                <Outlet />
            </div>
        </div>
    );
}
