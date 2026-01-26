<script lang="ts">
  import { Brain, BookOpen, Globe, FileText, RefreshCw } from "@lucide/svelte";
  import * as Sidebar from "$lib/components/ui/sidebar/index.js";
  import { isTauriEnv } from "$lib/backend";

  // link items.
  const links = [
    {
      title: "Documentation",
      url: "https://szablowskilab.github.io/rma-kinetics/docs",
      icon: BookOpen,
    },
    {
      title: "Research Paper",
      url: "https://www.biorxiv.org/content/10.1101/2025.11.17.688787v1",
      icon: FileText,
    },
    {
      title: "Szablowski Lab Website",
      url: "https://szablowskilab.org",
      icon: Globe,
    },
  ];

  let checkingForUpdates = $state(false);

  async function handleCheckForUpdates() {
    if (checkingForUpdates) return;

    checkingForUpdates = true;
    try {
      const { checkForUpdate } = await import("$lib/updater");
      const update = await checkForUpdate();
      if (!update) {
        // Show a brief notification that no updates are available
        console.log("No updates available");
      }
    } catch (error) {
      console.error("Failed to check for updates:", error);
    } finally {
      checkingForUpdates = false;
    }
  }
</script>

<Sidebar.Root collapsible="icon">
  <Sidebar.Header>
    <Sidebar.Menu>
      <Sidebar.MenuItem>
        <Sidebar.MenuButton size="lg" class="md:h-8 md:p-0">
          {#snippet child({ props })}
            <a href="##" {...props}>
              <div
                class="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg"
              >
                <Brain class="size-4" />
              </div>
              <div class="grid flex-1 text-left text-sm leading-tight">
                <span class="truncate text-lg">RMA Kinetics Simulator</span>
              </div>
            </a>
          {/snippet}
        </Sidebar.MenuButton>
      </Sidebar.MenuItem>
    </Sidebar.Menu>
  </Sidebar.Header>
  <Sidebar.Content>
    <Sidebar.Group>
      <Sidebar.GroupContent>
        <Sidebar.Menu>
          {#each links as item (item.title)}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton>
                {#snippet child({ props })}
                  <a href={item.url} target="_blank" {...props}>
                    <item.icon />
                    <span>{item.title}</span>
                  </a>
                {/snippet}
              </Sidebar.MenuButton>
            </Sidebar.MenuItem>
          {/each}
          {#if isTauriEnv}
            <Sidebar.MenuItem>
              <Sidebar.MenuButton onclick={handleCheckForUpdates}>
                {#snippet child({ props })}
                  <button type="button" {...props}>
                    <RefreshCw class={checkingForUpdates ? "animate-spin" : ""} />
                    <span>Check for Updates</span>
                  </button>
                {/snippet}
              </Sidebar.MenuButton>
            </Sidebar.MenuItem>
          {/if}
        </Sidebar.Menu>
      </Sidebar.GroupContent>
    </Sidebar.Group>
  </Sidebar.Content>
  <Sidebar.Rail />
</Sidebar.Root>
