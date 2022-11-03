export enum FieldType {
    Text = "Text"
}

export function fieldTypeIcon(type: FieldType): string {
    switch(type) {
        case FieldType.Text:
            return 'fa-font';
    }
}
export * from './Field';
export * from './TextField';