<script lang="ts">
    import { dev } from "$app/environment";
    import ModelSelection from "$lib/components/model-selection.svelte";
    import ModelError from "$lib/components/model-error.svelte";
    import OutputArea from "$lib/components/output-area.svelte";
    import * as Empty from "$lib/components/ui/empty/index.js";
    import { Zap } from "@lucide/svelte";

    let modelType = $state<
        "Constitutive" | "TetOff" | "Chemogenetic" | "Oscillation"
    >("Constitutive");

    let solution = $derived.by(() => {
        return {
            t: [],
            y: [],
            model: modelType,
        };
    });

    let summary = $derived.by(() => {
        if (solution.t.length > 0) {
            return [];
        }
    });

    // Error handling state
    let errorMessage = $state<string | null>(null);
    let errorOpen = $state<boolean>(false);

    // Sync errorOpen with errorMessage
    $effect(() => {
        if (errorMessage !== null) {
            errorOpen = true;
        }
    });

    // Clear error message when dialog is closed
    $effect(() => {
        if (!errorOpen) {
            errorMessage = null;
        }
    });

    if (dev) {
        $inspect("modelType: ", modelType);
        $inspect("solution: ", solution);
        $inspect("summary_stats: ", summary);
    }
</script>

<main class="grid grid-cols-[1fr_2fr] gap-10 p-10">
    <ModelSelection
        bind:modelType
        bind:solution
        bind:summary
        bind:errorMessage
    />
    {#if solution.t.length > 0}
        <OutputArea bind:modelType bind:solution bind:summary />
    {:else}
        <Empty.Root>
            <Empty.Header>
                <Empty.Media variant="icon">
                    <Zap />
                </Empty.Media>
                <Empty.Title>No Results Available</Empty.Title>
                <Empty.Description class="flex flex-col gap-5"
                    >Choose a tab to select a model and run a simulation to view
                    results.
                    <a
                        href="https://nsbuitrago.github.io/rma-kinetics-rs/"
                        target="_blank"
                        class="text-muted-foreground">Need help?</a
                    >
                </Empty.Description>
            </Empty.Header>
        </Empty.Root>
    {/if}
</main>

{#if errorMessage !== null}
    <ModelError bind:open={errorOpen} {errorMessage} />
{/if}
