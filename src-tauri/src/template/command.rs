use serde::Deserialize;
use sqlx::SqlitePool;
use tauri::State;

use crate::slug::{id, slugify};

use super::EntityTemplate;

#[tauri::command]
#[tracing::instrument(skip(pool))]
pub async fn list(pool: State<'_, SqlitePool>) -> Result<Vec<EntityTemplate>, String> {
    tracing::debug!("listing entity templates");
    sqlx::query_as("select * from entity_templates")
        .fetch_all(&*pool)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
#[tracing::instrument(skip(pool))]
pub async fn find(
    pool: State<'_, SqlitePool>,
    id: String,
) -> Result<Option<EntityTemplate>, String> {
    tracing::debug!("fetching entity template");
    sqlx::query_as("select * from entity_templates where id = ?1 or slug = ?1")
        .bind(id)
        .fetch_optional(&*pool)
        .await
        .map_err(|err| err.to_string())
}

#[derive(Debug, Deserialize)]
pub struct CreateEntityTemplate {
    name: String,
    schema: serde_json::Value,
}

#[tauri::command]
#[tracing::instrument(skip(pool))]
pub async fn create(
    pool: State<'_, SqlitePool>,
    template: CreateEntityTemplate,
) -> Result<EntityTemplate, String> {
    tracing::debug!("creating entity template");
    sqlx::query_as("insert into entity_templates (id, slug, name, schema) values (?, ?, ?, ?)")
        .bind(id())
        .bind(slugify(&template.name))
        .bind(template.name)
        .bind(template.schema)
        .fetch_one(&*pool)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
#[tracing::instrument(skip(pool))]
pub async fn update(
    pool: State<'_, SqlitePool>,
    template: EntityTemplate,
) -> Result<EntityTemplate, String> {
    tracing::debug!("creating entity template");
    sqlx::query_as(
        r#"
    update entity_templates 
    set
        slug = ?,
        name = ?,
        schema = ?
    where
        id = ?
    returning *
    "#,
    )
    .bind(slugify(&template.name))
    .bind(template.name)
    .bind(template.schema)
    .bind(template.id)
    .fetch_one(&*pool)
    .await
    .map_err(|err| err.to_string())
}
