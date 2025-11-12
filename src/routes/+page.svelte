<script lang="ts">
    import { dev } from "$app/environment";
    import ModelSelection from "$lib/components/model-selection.svelte";
    import OutputArea from "$lib/components/output-area.svelte";

    let modelType = $state<
        "Constitutive" | "TetOff" | "Chemogenetic" | "Oscillation"
    >("Constitutive");

    let solution = $derived.by(() => {
        let sol = {
            ts: [],
            ys: [],
            model: modelType,
        };

        return sol;
    });

    if (dev) {
        $inspect("modelType: ", modelType);
        $inspect("solution: ", solution);
    }
</script>

<main class="flex gap-10 p-10">
    <ModelSelection bind:modelType bind:solution />
    {#if solution.ts.length > 0}
        <OutputArea bind:modelType bind:solution />
    {:else}
        <p>Set parameters and click `Run Simulation` to view results</p>
    {/if}
</main>
