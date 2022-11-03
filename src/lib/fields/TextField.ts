import { Field, type FieldProps } from "./Field";
import type { JSONSchema } from 'json-schema-typed';

export interface TextFieldProps extends FieldProps {
    maxLength?: number,
    minLength?: number,
}

export class TextField extends Field {
    readonly #name: string;
    readonly #props: TextFieldProps;

    constructor(name: string, props: TextFieldProps) {
        super()
        this.#name = name;
        this.#props = props;
    }


    get name(): string {
        return this.#name
    }

    toJsonSchema(): JSONSchema.String {
        return {
            type: 'string',
            title: this.#props.label || this.#name,
            maxLength: this.#props.maxLength,
            minLength: this.#props.minLength,
        }
    }

}