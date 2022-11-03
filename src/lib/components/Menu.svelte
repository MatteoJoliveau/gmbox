<script lang="ts">
    import type { EntityTemplate } from "$lib/types";

    export let templates: EntityTemplate[] = [];
</script>

<aside class="menu">
    <a href="/"><h2 class="title">GM Box</h2></a>
    <p class="menu-label">Content</p>
    <ul class="menu-list">
        <li><a href="#">Entities</a></li>
        <li><a href="/templates">Entity Templates</a></li>
    </ul>
    <p class="menu-label">Templates</p>
    <ul class="menu-list">
        {#each templates as template}
            <li><a href={`/entities/${template.slug}`}>{template.name}</a></li>
        {:else}
            <p>No entity templates found</p>
        {/each}
    </ul>
</aside>
