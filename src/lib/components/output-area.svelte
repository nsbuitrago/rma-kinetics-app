<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";
    import * as Select from "$lib/components/ui/select/index.js";
    import { Button } from "$lib/components/ui/button/index.js";
    import * as Table from "$lib/components/ui/table/index.js";
    import { ImageDown, Save, Trash } from "@lucide/svelte";
    import { fade } from "svelte/transition";
    import * as Plot from "@observablehq/plot";
    import { dev } from "$app/environment";

    let { modelType = $bindable(), solution = $bindable() } = $props();
    let plotArea: HTMLElement;
    let time = $derived(solution.ts);

    let speciesTypes = $derived.by(() => {
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
        }
    });

    let speciesType = $state("plasmaRMA");
    const selectedSpecies = $derived(
        speciesTypes.find((s) => s.value === speciesType)?.label ??
            "Select a species",
    );

    // visualize result
    $effect(() => {
        // skip plotting if
        // 1. plotArea element hasn't been mounted in the DOM
        // 2. solution has no data
        if (!plotArea || !solution.ts || solution.ts.length === 0) {
            if (plotArea) plotArea.innerHTML = "";
            return;
        }

        let active = true;

        async function plotSpecies() {
            let species;
            switch (speciesType) {
                case "plasmaRMA":
                    species = await invoke("get_plasma_rma", {
                        solution: solution,
                    });
                    break;
                case "brainRMA":
                    species = await invoke("get_brain_rma", {
                        solution: solution,
                    });
                    break;
                case "tTA":
                    species = await invoke("get_tta", { solution: solution });
                    break;
                case "dox":
                    species = await invoke("get_brain_dox", {
                        solution: solution,
                    });
                    break;
                case "hM3Dq":
                    species = await invoke("get_dq", { solution: solution });
                    break;
                case "CNO":
                    species = await invoke("get_cno", { solution: solution });
                    break;
                case "CLZ":
                    species = await invoke("get_clz", { solution: solution });
                    break;
                default:
                    console.warn("Unknown species type: ", speciesType);
                    return;
            }

            const data = time.map((t, i) => ({
                Time: t,
                Concentration: species[i],
            }));

            if (active && plotArea && species) {
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

        return () => {
            active = false;
        };
    });

    // calcualte summary statistics
    let summaryStats = $state([]);
    $effect(() => {
        async function getSummary() {
            summaryStats = await invoke("get_summary", { solution: solution });
        }

        getSummary();

        if (dev) {
            $inspect("Simulation Summary: ", summaryStats);
        }
    });

    function clearOutput() {
        solution = {
            ts: [],
            ys: [],
            model: modelType,
        };
    }
</script>

<div class="flex" in:fade={{ duration: 300 }}>
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
                {#each speciesTypes as speciesType, speciesIndex (speciesType.value)}
                    <Table.Row>
                        <Table.Cell>{speciesType.label}</Table.Cell>
                        {#if summaryStats && summaryStats.length > 0 && summaryStats[0].length > 0}
                            <Table.Cell
                                >{summaryStats[speciesIndex][0].toFixed(
                                    2,
                                )}</Table.Cell
                            >
                            <Table.Cell
                                >{summaryStats[speciesIndex][1].toFixed(
                                    2,
                                )}</Table.Cell
                            >
                        {/if}
                    </Table.Row>
                {/each}
            </Table.Body>
        </Table.Root>
    </div>
    <div class="flex flex-col justify-between gap-2 h-1/2 p-10">
        <div class="flex flex-col gap-5">
            <Button>
                <ImageDown />
                Save Plot</Button
            >
            <Button variant="outline">
                <Save />
                Export to CSV</Button
            >
        </div>
        <Button variant="destructive" onclick={clearOutput}>
            <Trash />
            Clear Output</Button
        >
    </div>
</div>
