<script lang="ts">
    import { Button } from "$lib/components/ui/button/index.js";
    import { browser } from "$app/environment";

    let { run_simulation } = $props();

    const isMac = browser && navigator.userAgent.toUpperCase().includes("MAC");

    function handleKeyDown(event: KeyboardEvent) {
        if ((event.metaKey || event.ctrlKey) && event.key === "Enter") {
            event.preventDefault();
            run_simulation();
        }
    }
</script>

<svelte:window onkeydown={handleKeyDown} />

<Button type="submit">
    Run Simulation
    <span class="opacity-75">
        {#if isMac}
            ⌘+Return
        {:else}
            Ctrl+Enter
        {/if}
    </span>
</Button>
