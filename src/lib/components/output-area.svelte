<script lang="ts">
    import * as Select from "$lib/components/ui/select/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import * as Table from "$lib/components/ui/table/index.js";
    import { ImageDown, Save, Trash } from "@lucide/svelte";
    import { fade } from "svelte/transition";
    import * as Plot from "@observablehq/plot";
    import { dev } from "$app/environment";
    import { isTauriEnv } from "$lib/models.svelte";
    import { getSpecies } from "$lib/solution.js";

    let {
        modelType = $bindable(),
        solution = $bindable(),
        summary = $bindable(),
    } = $props();

    interface speciesTypeInterface {
        value: string;
        label: string;
    }

    let speciesTypes: speciesTypeInterface[] = $derived.by(() => {
        switch (modelType) {
            case "Constitutive":
                return [
                    { value: "brainRMA", label: "Brain RMA" },
                    { value: "plasmaRMA", label: "Plasma RMA" },
                ];
            case "Oscillating":
                return [
                    { value: "brainRMA", label: "Brain RMA" },
                    { value: "plasmaRMA", label: "Plasma RMA" },
                ];
            case "TetOff":
                return [
                    { value: "brainRMA", label: "Brain RMA" },
                    { value: "plasmaRMA", label: "Plasma RMA" },
                    { value: "tTA", label: "tTA" },
                    { value: "dox", label: "Dox" },
                ];
            case "Chemogenetic":
                return [
                    { value: "brainRMA", label: "Brain RMA" },
                    { value: "plasmaRMA", label: "Plasma RMA" },
                    { value: "tTA", label: "tTA" },
                    { value: "dox", label: "Dox" },
                    { value: "hM3Dq", label: "hM3Dq" },
                    { value: "CNO", label: "CNO" },
                    { value: "CLZ", label: "CLZ" },
                ];
            default:
                return [];
        }
    });

    let speciesType = $state<string>("plasmaRMA");
    let selectedSpecies = $derived(
        speciesTypes.find((s) => s.value === speciesType)?.label ??
            "Select a species",
    );

    let plotArea: HTMLElement;
    let time = $derived(solution.t);

    // visualize result
    $effect(() => {
        // skip plotting if
        // 1. plotArea element hasn't been mounted in the DOM
        // 2. solution has no data
        if (!plotArea || !solution.t || solution.t.length === 0) {
            if (plotArea) plotArea.innerHTML = "";
            return;
        }

        async function plotSpecies() {
            let species = getSpecies(solution, speciesType);

            const data = time.map((t: number, i: number) => ({
                Time: t,
                Concentration: species[i],
            }));

            if (plotArea && species) {
                const plot = Plot.plot({
                    grid: true,
                    marks: [
                        Plot.line(data, { x: "Time", y: "Concentration" }),
                        Plot.crosshair(data, { x: "Time", y: "Concentration" }),
                    ],
                });
                plotArea.replaceChildren(plot);
            }
        }

        plotSpecies();
    });
</script>

<div class="flex w-full" in:fade={{ duration: 300 }}>
    <div class="flex flex-col gap-2">
        <h1 class="text-xl">Results</h1>
        <Select.Root type="single" bind:value={speciesType}>
            <Select.Trigger class="w-[180px]">
                {selectedSpecies}
            </Select.Trigger>
            <Select.Content>
                {#each speciesTypes as speciesType (speciesType.value)}
                    <Select.Item
                        value={speciesType.value}
                        label={speciesType.label}
                    >
                        {speciesType.label}
                    </Select.Item>
                {/each}
            </Select.Content>
        </Select.Root>
        <div bind:this={plotArea} class="w-full"></div>
        <!-- summary table -->
        <Table.Root>
            <Table.Header>
                <Table.Row>
                    <Table.Head class="font-bold">Species</Table.Head>
                    <Table.Head class="font-bold">Max Concentration</Table.Head>
                    <Table.Head class="font-bold">T<sub>max</sub></Table.Head>
                </Table.Row>
            </Table.Header>
            <Table.Body>
                {#each summary as summaryData}
                    <Table.Row>
                        <Table.Cell>{summaryData.species}</Table.Cell>
                        <Table.Cell>
                            {summaryData.max_concentration.toFixed(2)}
                        </Table.Cell>
                        <Table.Cell>{summaryData.tmax.toFixed(2)}</Table.Cell>
                    </Table.Row>
                {/each}
            </Table.Body>
        </Table.Root>
    </div>
    <div class="flex flex-col justify-between gap-2 h-1/2 p-10">
        <!-- <div class="flex flex-col gap-5">
            <Button>
                <ImageDown />
                Save Plot</Button
            >
            <Button variant="outline">
                <Save />
                Export to CSV</Button
            >
        </div> -->
        <!-- <Button variant="destructive" onclick={clearOutput}>
            <Trash />
            Clear Output</Button
        > -->
    </div>
</div>
