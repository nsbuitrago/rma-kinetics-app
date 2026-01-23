<script lang="ts">
    import * as Tabs from "$lib/components/ui/tabs/index.js";
    import ConstitutiveModel from "$lib/components/constitutive-model.svelte";
    import TetoffModel from "$lib/components/tetoff-model.svelte";
    import ChemogeneticModel from "$lib/components/chemogenetic-model.svelte";
    import OscillationModel from "$lib/components/oscillating-model.svelte";

    let {
        modelType = $bindable(),
        solution = $bindable(),
        summary = $bindable(),
    } = $props();
</script>

<div class="grid gap-2">
    <h1 class="text-xl">Select Model</h1>
    <Tabs.Root bind:value={modelType} class="w-[400px]">
        <Tabs.List>
            <Tabs.Trigger value="Constitutive">Constitutive</Tabs.Trigger>
            <Tabs.Trigger value="TetOff">Tet-Off</Tabs.Trigger>
            <Tabs.Trigger value="Chemogenetic">Chemogenetic</Tabs.Trigger>
            <Tabs.Trigger value="Oscillating">Oscillating</Tabs.Trigger>
        </Tabs.List>
        <Tabs.Content value="Constitutive">
            <ConstitutiveModel bind:solution bind:summary />
        </Tabs.Content>
        <Tabs.Content value="TetOff">
            <TetoffModel bind:solution />
        </Tabs.Content>
        <Tabs.Content value="Chemogenetic">
            <ChemogeneticModel bind:solution />
        </Tabs.Content>
        <Tabs.Content value="Oscillating">
            <OscillationModel bind:solution bind:summary />
        </Tabs.Content>
    </Tabs.Root>
</div>
