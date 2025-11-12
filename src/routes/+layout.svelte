<script lang="ts">
    import AppSidebar from "$lib/components/app-sidebar.svelte";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import "../app.css";

  let { children } = $props();
</script>

<Sidebar.Provider>
    <AppSidebar />
    <main>
        <Sidebar.Trigger />
        {@render children?.()}
    </main>
</Sidebar.Provider>
