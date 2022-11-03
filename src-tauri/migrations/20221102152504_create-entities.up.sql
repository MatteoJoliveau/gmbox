create table entities (
    id text primary key not null,
    slug text not null,
    name text not null,
    template_id text not null,
    data text not null,
    unique(template_id, name),
    foreign key (template_id) 
        references entity_templates (id)
        on delete restrict

);