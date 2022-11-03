<script lang="ts">
    import Modal from "$lib/components/Modal.svelte";
    import Navbar from "$lib/components/Navbar.svelte";
    import { FieldType } from "$lib/fields";
    import type { Field } from "$lib/fields/Field";
    import { notify } from "$lib/notifications";
    import type { EntityTemplate } from "$lib/types";
    import { emit } from "@tauri-apps/api/event";
    import { invoke } from "@tauri-apps/api/tauri";
    import ModalNewField from "./ModalFieldType.svelte";
    import ModalTextField from "./ModalTextField.svelte";

    export let data: import("./$types").PageData;
    let template: EntityTemplate = data.template;
    let properties = template.schema.properties || {};

    let dirty = false;
    let showNewModal = false;
    let newFieldType: FieldType | undefined;

    function resetModal() {
        showNewModal = false;
        newFieldType = undefined;
    }

    function addField(field: Field) {
        properties[field.name] = field.toJsonSchema();
        dirty = true;
        resetModal();
    }

    function removeField(name: string) {
        const { [name]: prop, ...props } = properties;
        properties = props;
        dirty = true;
    }

    async function save() {
        try {
            template.schema.properties = properties;
            template = await invoke('plugin:templates|update', { template })
            properties = template.schema.properties || {};
            notify('Entity template updated')
        } catch (e) {
            emit('error', e);
        }
    }
</script>

<Navbar slot="navbar">
    <nav class="breadcrumb" aria-label="breadcrumbs" slot="title">
        <ul>
            <li>
                <a href="/templates" class="has-text-weight-bold"
                    >Entity Templates</a
                >
            </li>
            <li class="is-active"><a href=".">{template.name}</a></li>
        </ul>
    </nav>
    <div class="navbar-end">
        <div class="navbar-item">
            <div class="buttons">
                <button class="button is-primary" disabled={!dirty} on:click={save}>
                    <strong>Save</strong>
                </button>
                <button class="button">
                    <span class="icon is-small">
                        <i class="fas fa-gear" />
                    </span>
                </button>
            </div>
        </div>
    </div>
</Navbar>

<Modal bind:active={showNewModal} card={newFieldType === FieldType.Text}>
    {#if newFieldType === undefined}
        <ModalNewField bind:fieldType={newFieldType} />
    {:else if newFieldType === FieldType.Text}
        <ModalTextField
            on:cancel={resetModal}
            on:create={(ev) => addField(ev.detail)}
        />
    {/if}
</Modal>

<div class="level">
    <div class="level-left">
        <h2 class="title is-2 level-item">Fields</h2>
    </div>

    <div class="level-right">
        <button
            class="button is-link"
            on:click={() => {
                showNewModal = true;
            }}>Add field</button
        >
    </div>
</div>

<div class="container">
    <div class="columns">
        <div class="column">
            {#each Object.entries(properties) as [name, prop]}
                <div class="box">
                    <div class="level">
                        <div class="level-left">
                            <p class="level-item is-capitalized">{name}</p>
                        </div>

                        <div class="level-right">
                            <p class="level-item is-capitalized">{prop.type}</p>
                            <div class="buttons level-item">
                                <button class="button">
                                    <span class="icon is-small">
                                        <i class="fas fa-gear" />
                                    </span>
                                </button>
                                <button class="button" on:click={() => removeField(name)}>
                                    <span class="icon is-small">
                                        <i class="fas fa-trash" />
                                    </span>
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            {/each}
        </div>
    </div>
</div>
