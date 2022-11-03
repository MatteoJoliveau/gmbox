<script lang="ts">
    export let active = false;
    export let card = false;

    function close() {
        active = false;
    }
</script>

<div class="modal" class:is-active={active}>
    <div class="modal-background" on:click={close} />
    <div class={card ? 'modal-card' : 'modal-content'}>
        <slot />
    </div>
    <button on:click={close} class="modal-close is-large" aria-label="close" />
</div>
