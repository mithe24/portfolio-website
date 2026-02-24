import type { ComponentType } from 'svelte';
import { writable } from 'svelte/store';

export const path = writable(window.location.pathname);

window.onpopstate = (): void => {
    path.set(window.location.pathname);
};

export function navigate(e: MouseEvent, href: string): void {
    e.preventDefault();
    path.set(href);
    history.pushState({}, '', href);
}

export function createRouter(routes: Record<string, ComponentType>): {
    Page: ComponentType;
} {
    return {
        get Page() {
            return routes[window.location.pathname] ?? routes['404'];
        }
    };
}
