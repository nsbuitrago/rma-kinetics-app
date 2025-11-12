<script lang="ts">
    import { dev } from "$app/environment";
    import ModelSelection from "$lib/components/model-selection.svelte";
    import OutputArea from "$lib/components/output-area.svelte";
    import * as Empty from "$lib/components/ui/empty/index.js";
    import { Zap } from "@lucide/svelte";

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
        <Empty.Root>
            <Empty.Header>
                <Empty.Media variant="icon">
                    <Zap />
                </Empty.Media>
                <Empty.Title>No Results Available</Empty.Title>
                <Empty.Description class="flex flex-col gap-5"
                    >Choose a tab to select a model and run simulation to view
                    results.
                    <a
                        href="https://szablowskilab.github.io/rma-kinetics/docs"
                        target="_blank"
                        class="text-muted-foreground">Need help?</a
                    >
                </Empty.Description>
            </Empty.Header>
        </Empty.Root>
    {/if}
</main>
