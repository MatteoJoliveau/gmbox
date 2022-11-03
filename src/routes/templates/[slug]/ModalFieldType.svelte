<script lang="ts">
    import { FieldType, fieldTypeIcon } from "$lib/fields";


    const fieldTypes = Object.values(FieldType);

    export let fieldType: FieldType | undefined;
</script>

<div class="box">
    <h3 class="title is-3">Add Field</h3>
    <div class="buttons">
        {#each fieldTypes as type}
                <button class="button" on:click={() => { fieldType = FieldType[type]; }}>
                    <span class="icon is-small">
                        <i class={`fas ${fieldTypeIcon(FieldType[type])}`} />
                    </span>
                    <span>{type}</span>
                </button>
        {/each}
    </div>
</div>

<style lang="scss">
    .type-box {
        border: 1.5px solid transparent;
    }
</style>
