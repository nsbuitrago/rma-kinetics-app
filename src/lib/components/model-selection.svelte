<script lang="ts">
    import * as Tabs from "$lib/components/ui/tabs/index.js";
    import ConstitutiveModel from "$lib/components/constitutive-model.svelte";
    import TetoffModel from "$lib/components/tetoff-model.svelte";
    import ChemogeneticModel from "$lib/components/chemogenetic-model.svelte";
    import OscillationModel from "$lib/components/oscillating-model.svelte";
    import { browser } from "$app/environment";
    import { isTauriEnv } from "$lib/models.svelte";

    let {
        modelType = $bindable(),
        solution = $bindable(),
        summary = $bindable(),
        errorMessage = $bindable(),
    } = $props();

    // References to each model's run_simulation function
    let constitutiveRunSimulation = $state<(() => Promise<void>) | undefined>();
    let tetoffRunSimulation = $state<(() => Promise<void>) | undefined>();
    let chemogeneticRunSimulation = $state<(() => Promise<void>) | undefined>();
    let oscillatingRunSimulation = $state<(() => Promise<void>) | undefined>();

    // Centralized keyboard shortcut handler
    function handleKeyDown(event: KeyboardEvent) {
        if ((event.metaKey || event.ctrlKey) && event.key === "Enter") {
            event.preventDefault();

            // Call the appropriate simulation based on the active tab
            switch (modelType) {
                case "Constitutive":
                    constitutiveRunSimulation?.();
                    break;
                case "TetOff":
                    tetoffRunSimulation?.();
                    break;
                case "Chemogenetic":
                    chemogeneticRunSimulation?.();
                    break;
                case "Oscillating":
                    oscillatingRunSimulation?.();
                    break;
            }
        } else if ((event.metaKey || event.ctrlKey) && event.key === "1") {
            modelType = "Constitutive";
        } else if ((event.metaKey || event.ctrlKey) && event.key === "2") {
            modelType = "TetOff";
        } else if ((event.metaKey || event.ctrlKey) && event.key === "3") {
            modelType = "Chemogenetic";
        } else if ((event.metaKey || event.ctrlKey) && event.key === "4") {
            modelType = "Oscillating";
        }
    }

    const showKeyboardShortcuts = isTauriEnv;
    const isMac = browser && navigator.userAgent.toUpperCase().includes("MAC");
</script>

<svelte:window onkeydown={handleKeyDown} />

<div class="grid gap-2">
    <h1 class="text-xl">Select Model</h1>
    <Tabs.Root bind:value={modelType}>
        <Tabs.List>
            <Tabs.Trigger value="Constitutive">
                Constitutive
                <span class="text-sm opacity-75 text-muted-foreground">
                    {#if showKeyboardShortcuts}
                        {#if isMac}
                            ⌘1
                        {:else}
                            ^1
                        {/if}
                    {/if}
                </span>
            </Tabs.Trigger>
            <Tabs.Trigger value="TetOff">
                Tet-Off
                <span class="text-sm opacity-75 text-muted-foreground">
                    {#if showKeyboardShortcuts}
                        {#if isMac}
                            ⌘2
                        {:else}
                            ^2
                        {/if}
                    {/if}
                </span>
            </Tabs.Trigger>
            <Tabs.Trigger value="Chemogenetic">
                Chemogenetic
                <span class="text-sm opacity-75 text-muted-foreground">
                    {#if showKeyboardShortcuts}
                        {#if isMac}
                            ⌘3
                        {:else}
                            ^3
                        {/if}
                    {/if}
                </span>
            </Tabs.Trigger>
            <Tabs.Trigger value="Oscillating">
                Oscillating
                <span class="text-sm opacity-75 text-muted-foreground">
                    {#if showKeyboardShortcuts}
                        {#if isMac}
                            ⌘4
                        {:else}
                            ^4
                        {/if}
                    {/if}
                </span>
            </Tabs.Trigger>
        </Tabs.List>
        <Tabs.Content value="Constitutive">
            <ConstitutiveModel
                bind:solution
                bind:summary
                bind:errorMessage
                bind:runSimulation={constitutiveRunSimulation}
            />
        </Tabs.Content>
        <Tabs.Content value="TetOff">
            <TetoffModel
                bind:solution
                bind:summary
                bind:errorMessage
                bind:runSimulation={tetoffRunSimulation}
            />
        </Tabs.Content>
        <Tabs.Content value="Chemogenetic">
            <ChemogeneticModel
                bind:solution
                bind:summary
                bind:errorMessage
                bind:runSimulation={chemogeneticRunSimulation}
            />
        </Tabs.Content>
        <Tabs.Content value="Oscillating">
            <OscillationModel
                bind:solution
                bind:summary
                bind:errorMessage
                bind:runSimulation={oscillatingRunSimulation}
            />
        </Tabs.Content>
    </Tabs.Root>
</div>
