<script lang="ts">
    import { SelectedTemplate } from "../../lib/types";

    export let selectedTemplate: SelectedTemplate | undefined;
</script>

<div class="box has-text-centered">
    <h3 class="title is-3">Create new template</h3>

    <div class="buttons is-centered">
        <button
            class="button"
            on:click={() => {
                selectedTemplate = SelectedTemplate.Blank;
            }}
        >
            Create new&nbsp;<strong>blank template</strong>
        </button>
    </div>
</div>
