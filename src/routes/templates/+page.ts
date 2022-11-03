import { invoke } from '@tauri-apps/api'
import type { EntityTemplate } from '../../lib/types'

export interface TemplatesData {
    templates: EntityTemplate[],
}

export const load: import('./$types').PageLoad<TemplatesData> = async () => {
    const templates = await invoke<EntityTemplate[]>("plugin:templates|list");
    return { templates }
}