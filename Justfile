CONFIG_DIR := env_var_or_default('XDG_DATA_HOME', '$HOME/.local/share')
APP_IDENTIFIER := "com.matteojoliveau.gmbox"
DB := join(CONFIG_DIR, APP_IDENTIFIER, "gmbox.db")

set dotenv-load := false

default:
  @just --list | grep -v "    default"

clean:
    rm -rf build
    just src-tauri/clean

run: 
    pnpm tauri dev

_build_ui:
    pnpm build

build: _build_ui

setup:
    pnpm install
    just src-tauri/setup

fmt: _build_ui
    just src-tauri/fmt

sqlite_web:
    sqlite_web {{ DB }} --no-browser

migration_create name:
    #!/usr/bin/env sh
    name="$(date -u +%Y%m%d%H%M%S)_{{name}}"
    touch "src-tauri/migrations/$name.up.sql"
    touch "src-tauri/migrations/$name.down.sql"
    echo "created migration $name"

cargo *args:
    cd src-tauri && cargo {{args}}