import type { JSONSchema } from 'json-schema-typed';

export enum SelectedTemplate {
    Blank
}

export interface EntityTemplate {
    id: string,
    slug: string,
    name: string,
    schema: JSONSchema.Object,
}
