<script lang="ts">
    import favicon from '$lib/assets/favicon.svg';
    import {onMount} from 'svelte';

    onMount(async () => {
        // If the static splash was injected (only on the fallback file), remove it.
        document.getElementById('cf-splash')?.remove();
        document.getElementById('cf-splash-style')?.remove();
    });

    let {children} = $props();
</script>

<svelte:head>
    <link rel="icon" href={favicon}/>
</svelte:head>

<style>
    :global(.link-button) {
        display: inline;
        padding: 0;
        border: 0;
        font: inherit;
        text-decoration: underline;
        cursor: pointer;
        background: transparent;
        color: currentColor;

        appearance: none;
    }
</style>

{@render children?.()}
