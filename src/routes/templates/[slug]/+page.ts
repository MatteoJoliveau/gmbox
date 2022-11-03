import { goto } from '$app/navigation';
import { invoke } from '@tauri-apps/api'
import { emit } from '@tauri-apps/api/event';
import type { EntityTemplate } from '../../../lib/types'

export interface TemplateData {
    template: EntityTemplate
}
export const load: import('./$types').PageLoad<TemplateData> = async ({ params }) => {
    try {
        const template = await invoke<EntityTemplate | null>("plugin:templates|find", { id: params.slug });
        
        if (!template) {
            emit('error', `Template '${params.slug}' not found`);
            goto('/templates');
        }
        
        return { template }
    } catch (e) {
        emit('error', e);
    }
}