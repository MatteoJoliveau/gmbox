use std::{fmt::Display, str::FromStr, convert::Infallible};

use tauri::{AboutMetadata, CustomMenuItem, Menu, MenuItem, Submenu, WindowMenuEvent, Runtime};

enum MenuItemId {
    NewEntity,
    NewEntityTemplate,
}

impl Display for MenuItemId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            MenuItemId::NewEntity => "new_entity",
            MenuItemId::NewEntityTemplate => "new_entity_template",
        };

        Display::fmt(s, f)
    }
}

impl FromStr for MenuItemId {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "new_entity" => Self::NewEntity,
            "new_entity_template" => Self::NewEntityTemplate,
            s => return Err(s.into()),
        })
    }
}

impl From<MenuItemId> for String {
    fn from(item: MenuItemId) -> Self {
        item.to_string()
    }
}

fn file() -> Submenu {
    Submenu::new(
        "File",
        Menu::new()
            .add_item(CustomMenuItem::new(MenuItemId::NewEntity, "New Entity").accelerator("CMDorCTRL + N").disabled())
            .add_item(CustomMenuItem::new(MenuItemId::NewEntityTemplate, "New Entity Template").accelerator("CMDorCTRL + Shift + N"))
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::Quit),
    )
}

fn help() -> Submenu {
    let repository = Some(env!("CARGO_PKG_REPOSITORY")).filter(|s| s.is_empty());
    let homepage = Some(env!("CARGO_PKG_HOMEPAGE")).filter(|s| s.is_empty());

    let website = homepage.or(repository).unwrap_or_default();
    Submenu::new(
        "Help",
        Menu::new()
            .add_native_item(MenuItem::Separator)
            .add_native_item(MenuItem::About(
                "GM Box".into(),
                AboutMetadata::new()
                    .version(env!("CARGO_PKG_VERSION"))
                    .copyright("Copyright 2022 Matteo Joliveau")
                    .license(env!("CARGO_PKG_LICENSE"))
                    .website(website)
                    .authors(
                        env!("CARGO_PKG_AUTHORS")
                            .split(":")
                            .map(str::to_string)
                            .collect(),
                    ),
            )),
    )
}

pub fn new() -> Menu {
    Menu::new().add_submenu(file()).add_submenu(help())
}

pub fn on_menu_event<R: Runtime>(event: WindowMenuEvent<R>) {
    match MenuItemId::from_str(event.menu_item_id()) {
        Ok(MenuItemId::NewEntity) => todo!(),
        Ok(MenuItemId::NewEntityTemplate) => event.window().emit("navigate", "/templates/new").unwrap(),
        Err(menu_item_id) => tracing::warn!(menu_item_id, "unhandled menu event"),
    }
}