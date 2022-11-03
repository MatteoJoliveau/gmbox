import { emit } from "@tauri-apps/api/event";

export async function notify(message: string) {
    emit('notification', message);
}