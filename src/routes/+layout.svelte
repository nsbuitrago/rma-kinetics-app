<script lang="ts">
  import { onMount } from "svelte";
  import AppSidebar from "$lib/components/app-sidebar.svelte";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import UpdateDialog from "$lib/components/UpdateDialog.svelte";
  import { isTauriEnv } from "$lib/backend";
  import "../app.css";

  let { children } = $props();

  onMount(async () => {
    if (isTauriEnv) {
      // Check for updates on startup with a short delay
      setTimeout(async () => {
        const { checkForUpdate } = await import("$lib/updater");
        await checkForUpdate();
      }, 1000);
    }
  });
</script>

<Sidebar.Provider>
  <AppSidebar />
  <main>
    <Sidebar.Trigger />
    {@render children?.()}
  </main>
</Sidebar.Provider>

{#if isTauriEnv}
  <UpdateDialog />
{/if}
