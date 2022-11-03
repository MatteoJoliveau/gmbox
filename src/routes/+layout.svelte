<script lang="ts">
  import { goto } from "$app/navigation";
  import Menu from "$lib/components/Menu.svelte";
  import "$lib/styles/app.scss";
  import {
    isPermissionGranted,
    requestPermission,
    sendNotification,
  } from "@tauri-apps/api/notification";
  import { message } from "@tauri-apps/api/dialog";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";

  export let data: import('./$types').LayoutData;

  let unlistens: UnlistenFn[] = [];

  let permissionGranted = false;

  onMount(async () => {
    permissionGranted = await isPermissionGranted();
    if (!permissionGranted) {
      const permission = await requestPermission();
      permissionGranted = permission === "granted";
    }
    const unNavigate = await listen<string>("navigate", async (event) => {
      await goto(event.payload);
    });

    const unNotification = await listen<string>(
      "notification",
      async (event) => {
        if (permissionGranted) {
          sendNotification({ title: "GM Box", body: event.payload });
        }
      }
    );

    const unError = await listen<string>("error", async (event) => {
      await message(event.payload, { title: "Error", type: "error" });
    });

    unlistens.push(unNavigate);
    unlistens.push(unNotification);
    unlistens.push(unError);
  });

  onDestroy(() => {
    for (const unlisten of unlistens) {
      unlisten();
    }
  });
</script>

<!-- {#each notifications.iter() as notification}
  <Notification on:close={}>{notification.message}</Notification>
{/each} -->
<div class="columns mt-1 mx-1">
  <div class="column is-one-fifth">
    <Menu templates={data.templates}/>
  </div>
  <main class="column">
    <slot name="navbar" />
    <slot />
  </main>
</div>

<style lang="scss">
  .mt-1 {
    margin-top: 10px;
  }

  .mx-1 {
    margin-left: 10px;
    margin-right: 10px;
  }
</style>
