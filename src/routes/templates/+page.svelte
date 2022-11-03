<script lang="ts">
    import { goto } from "$app/navigation";
    import Modal from "$lib/components/Modal.svelte";
    import Navbar from "$lib/components/Navbar.svelte";
    import { invoke } from "@tauri-apps/api";
    import { emit } from "@tauri-apps/api/event";
    import ModalBlankTemplate from "./ModalBlankTemplate.svelte";
    import ModalSelectKind from "./ModalSelectKind.svelte";
    import { SelectedTemplate, type EntityTemplate } from "../../lib/types";

    let showNewModal = false;
    let newSelectedTemplate: SelectedTemplate | undefined;

    export let data: import("./$types").PageData;

    function resetModal() {
        showNewModal = false;
        newSelectedTemplate = undefined;
    }

    async function createTemplate(ev: any) {
        try {
            const { slug } = await invoke<EntityTemplate>(
                "plugin:templates|create",
                {
                    entityTemplate: ev.detail,
                }
            );
            await goto(`/templates/${slug}`);
        } catch (e) {
            emit("error", e);
        }
    }
</script>

<Navbar slot="navbar">
    <h1 class="has-text-weight-bold" slot="title">Entity Templates</h1>
    <div class="navbar-end">
        <div class="navbar-item">
            <div class="buttons">
                <button
                    on:click={() => {
                        showNewModal = !showNewModal;
                    }}
                    class="button is-primary"
                >
                    <strong>Add Template</strong>
                </button>
            </div>
        </div>
    </div>
</Navbar>

<Modal bind:active={showNewModal}>
    {#if newSelectedTemplate === undefined}
        <ModalSelectKind bind:selectedTemplate={newSelectedTemplate} />
    {:else if newSelectedTemplate === SelectedTemplate.Blank}
        <ModalBlankTemplate
            on:back={() => {
                newSelectedTemplate = undefined;
            }}
            on:close={resetModal}
            on:create={createTemplate}
        />
    {/if}
</Modal>
<table class="table is-striped is-fullwidth">
    <thead>
        <tr>
            <th>Name</th>
            <th>Actions</th>
        </tr>
    </thead>
    <tbody>
        {#each data.templates as template}
            <tr>
                <td
                    ><a href={`/templates/${template.slug}`}>{template.name}</a
                    ></td
                >
                <td />
            </tr>
        {/each}
    </tbody>
</table>
