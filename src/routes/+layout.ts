import { browser } from '$app/environment';
import type { EntityTemplate } from '$lib/types';

export const prerender = true;
export const csr = true;
export const ssr = false;

export interface TemplatesData {
    templates: EntityTemplate[],
}

export const load: import('./$types').LayoutLoad<TemplatesData> = async () => {
    if (browser) {
        const { invoke } = await import('@tauri-apps/api');
        const templates = invoke<EntityTemplate[]>("plugin:templates|list");
        return { templates }
    }

    return { templates: [] }
}