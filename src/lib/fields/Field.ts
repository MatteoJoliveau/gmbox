import type { JSONSchema } from 'json-schema-typed';

export interface FieldProps {
    label?: string,
}
export abstract class Field {
    abstract get name(): string;
    abstract toJsonSchema(): JSONSchema;
}
