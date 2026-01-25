<script lang="ts">
    import { Button } from "$lib/components/ui/button/index.js";
    import { browser } from "$app/environment";

    const isMac = browser && navigator.userAgent.toUpperCase().includes("MAC");
</script>

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
