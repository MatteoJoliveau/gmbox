create table entity_templates (
    id text primary key not null,
    slug text not null,
    name text not null unique,
    schema text not null
);