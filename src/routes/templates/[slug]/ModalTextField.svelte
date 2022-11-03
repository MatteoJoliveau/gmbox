<script lang="ts">
    import { TextField, type TextFieldProps } from "$lib/fields";
    import { createEventDispatcher } from "svelte";

    const dispatch = createEventDispatcher();

    enum Section {
        General = "General",
        Validation = "Validation",
    }

    let currentSection = Section.General;

    let nameDirty = false;
    let name = "";
    let props: TextFieldProps = {}
    
    function namify(s: string): string {
        return s.toLowerCase().replaceAll(' ', '-')
    }

    function generateName(ev: Event) {
        if (nameDirty) return;

        name = namify(ev.target?.value);
    }

    function updateName(ev: Event) {
        nameDirty = true;
        name = namify(ev.target?.value)
    }

    function submit() {
        const field = new TextField(name, props);
        dispatch('create', field);
    }
</script>

<header class="modal-card-head">
    <div class="modal-card-title tabs is-fullwidth is-boxed">
        <ul>
            {#each Object.values(Section) as section}
                <li class:is-active={currentSection === Section[section]}>
                    <a
                        on:click={() => {
                            currentSection = Section[section];
                        }}>{section}</a
                    >
                </li>
            {/each}
        </ul>
    </div>
  </header>
  <section class="modal-card-body">
    {#if currentSection === Section.General}
    <div class="field">
        <label class="label" for="label">Label</label>
        <div class="control">
            <input class="input" type="text" name="label" placeholder="Label" bind:value={props.label} on:input={generateName}>
        </div>
        <p class="help">Human-readable label for this field</p>
    </div>
    <div class="field">
        <label class="label" for="name">Name</label>
        <div class="control">
            <input class="input" type="text" name="name" placeholder="Name" value={name} on:input={updateName}>
        </div>
        <p class="help">Human-readable label for this field</p>
    </div>
    {:else if currentSection === Section.Validation}
    <div class="field">
        <label class="label" for="minimum">Minimum</label>
        <div class="control">
            <input class="input" type="number" name="minimum" placeholder="Minimum" bind:value={props.minLength}>
        </div>
        <p class="help">The minimum characters this field must have</p>
    </div>
    <div class="field">
        <label class="label" for="maximum">Maximum</label>
        <div class="control">
            <input class="input" type="number" name="maximum" placeholder="Maximum" bind:value={props.maxLength}>
        </div>
        <p class="help">The maximum characters this field must have</p>
    </div>
    {/if}
  </section>
  <footer class="modal-card-foot">
    <button class="button is-success" on:click={submit}>Save changes</button>
    <button class="button" on:click={() => dispatch('cancel')}>Cancel</button>
</footer>
