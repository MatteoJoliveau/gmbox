<script lang="ts">
    import { createEventDispatcher } from "svelte";
    import type { EntityTemplate } from "../../lib/types";

    const dispatch = createEventDispatcher();

    let name = "";
    let nameError = "";

    function create() {
        if (!name) {
            nameError = "name cannot be empty";
            return;
        }

        const entity: Pick<EntityTemplate, 'name' | 'schema'> = {
            name,
            schema: {
                type: "object",
                properties: {},
            },
        };

        dispatch("create", entity);
    }
</script>

<div class="box">
    <h3 class="title is-3">Add blank template</h3>

    <div class="field">
        <label class="label" for="name">Name</label>
        <div class="control">
            <input
                on:input={() => { nameError = ""; }}
                class="input"
                class:is-danger={!!nameError}
                type="text"
                name="name"
                bind:value={name}
                placeholder="Location"
            />
            {#if nameError}
                <p class="help is-danger">{nameError}</p>
            {/if}
        </div>
    </div>
    <div class="level">
        <div class="level-left">
            <button
                class="button level-item is-white"
                on:click={() => dispatch("back")}
            >
                <span class="icon is-small">
                    <i class="fas fa-chevron-left" />
                </span>
                <span>Back</span>
            </button>
        </div>
        <div class="level-right">
            <button class="button level-item" on:click={() => dispatch("close")}
                >Cancel</button
            >
            <button class="button level-item is-link" on:click={create}>Create Template</button>
        </div>
    </div>
</div>
