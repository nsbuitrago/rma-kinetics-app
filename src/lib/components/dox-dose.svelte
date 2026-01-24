<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import * as Table from "$lib/components/ui/table/index.js";
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Trash } from "@lucide/svelte";
    import { AccessPeriod } from "$lib/models.svelte";

    let {
        model = $bindable(),
        simulationParams = $bindable(),
        dialogOpen = $bindable(),
    } = $props();

    let doxDose = $state<number>(0);
    let doxT0 = $state<number>(0);
    let doxT1 = $state<number>(0);
    let doxView = $state<"main" | "pk">("main");

    function pushDoxAccessPeriod(period: AccessPeriod) {
        model.dox_pk_model.schedule.push(period);
    }

    function deleteDoxAccessPeriod(index: number) {
        model.dox_pk_model.schedule.splice(index, 1);
    }
</script>

<Dialog.Root bind:open={dialogOpen} onOpenChange={() => (doxView = "main")}>
    <Dialog.Trigger class={buttonVariants({ variant: "outline" })}>
        Dox Dose
    </Dialog.Trigger>
    <Dialog.Content class="overflow-hidden p-0">
        <div
            class="flex transition-transform duration-300 ease-in-out"
            style="transform: translateX({doxView === 'main' ? '0%' : '-100%'})"
        >
            <!-- Main View -->
            <div class="w-full shrink-0 flex flex-col gap-3 p-6">
                <div class="flex justify-between gap-2">
                    <div class="grid gap-2">
                        <Label for="dox-dose">Dose (mg/kg)</Label>
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="dox-dose"
                            bind:value={doxDose}
                        />
                    </div>
                    <div class="grid gap-2">
                        <Label for="dox-start">
                            Start Time ({simulationParams.timeUnits})
                        </Label>
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="dox-start"
                            bind:value={doxT0}
                        />
                    </div>
                    <div class="grid gap-2">
                        <Label for="dox-stop">
                            Stop Time ({simulationParams.timeUnits})
                        </Label>
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="dox-stop"
                            bind:value={doxT1}
                        />
                    </div>
                </div>
                <Button
                    onclick={() =>
                        pushDoxAccessPeriod(
                            new AccessPeriod(doxDose, [doxT0, doxT1]),
                        )}
                >
                    Add new dox access period
                </Button>
                <Button
                    variant="outline"
                    onclick={() => (doxView = "pk")}
                    class="hover:cursor-pointer"
                >
                    See More Parameters
                </Button>
                <div class="max-h-48 overflow-y-auto">
                    <Table.Root>
                        <Table.Header>
                            <Table.Row>
                                <Table.Head class="font-bold">Dose</Table.Head>
                                <Table.Head class="font-bold">Start Time</Table.Head>
                                <Table.Head class="font-bold">Stop Time</Table.Head>
                            </Table.Row>
                        </Table.Header>
                        <Table.Body class="scroll-auto">
                            {#each model.dox_pk_model.schedule as schedule, index}
                                <Table.Row>
                                    <Table.Cell>{schedule.dose}</Table.Cell>
                                    <Table.Cell>{schedule.time[0]}</Table.Cell>
                                    <Table.Cell>{schedule.time[1]}</Table.Cell>
                                    <Table.Cell>
                                        <Button
                                            variant="ghost"
                                            class="text-destructive"
                                            onclick={() =>
                                                deleteDoxAccessPeriod(index)}
                                        >
                                            <Trash />
                                        </Button>
                                    </Table.Cell>
                                </Table.Row>
                            {/each}
                        </Table.Body>
                    </Table.Root>
                </div>
            </div>
            <!-- PK Parameters View -->
            <div class="w-full shrink-0 flex flex-col gap-3 p-6">
                <h3 class="font-semibold">PK Parameters</h3>
                <div class="flex justify-between gap-2">
                    <div class="grid gap-2">
                        <Label for="dox-intake">
                            Food Intake (mg/{simulationParams.timeUnits})
                        </Label>
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="dox-intake"
                            bind:value={model.dox_pk_model.vehicle_intake}
                        />
                    </div>
                    <div class="grid gap-2">
                        <Label for="dox-bioavailability">
                            Bioavailability [0, 1]
                        </Label>
                        <Input
                            type="number"
                            min="0"
                            max="1"
                            step="any"
                            id="dox-bioavailability"
                            bind:value={model.dox_pk_model.bioavailability}
                        />
                    </div>
                </div>
                <div class="flex justify-between gap-2">
                    <div class="grid gap-2">
                        <Label for="dox-absorption">
                            Absorption Rate (1/{simulationParams.timeUnits})
                        </Label>
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="dox-absorption"
                            bind:value={model.dox_pk_model.absorption}
                        />
                    </div>
                    <div class="grid gap-2">
                        <Label for="dox-elimination">
                            Elimination Rate (1/{simulationParams.timeUnits})
                        </Label>
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="dox-elimination"
                            bind:value={model.dox_pk_model.elimination}
                        />
                    </div>
                </div>
                <div class="flex justify-between gap-2">
                    <div class="grid gap-2">
                        <Label for="dox-brain-transport">
                            Brain Transport Rate (1/{simulationParams.timeUnits})
                        </Label>
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="dox-brain-transport"
                            bind:value={model.dox_pk_model.brain_transport}
                        />
                    </div>
                    <div class="grid gap-2">
                        <Label for="dox-plasma-transport">
                            Plasma Transport Rate (1/{simulationParams.timeUnits})
                        </Label>
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="dox-plasma-transport"
                            bind:value={model.dox_pk_model.plasma_transport}
                        />
                    </div>
                </div>
                <div class="flex justify-between gap-2">
                    <div class="grid gap-2">
                        <Label for="dox-vd">Volume of Distribution (L)</Label>
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="dox-vd"
                            bind:value={model.dox_pk_model.plasma_vd}
                        />
                    </div>
                    <div class="grid gap-2">
                        <Label for="dox-kd">
                            Dox-tTA Kd ({simulationParams.concentrationUnits})
                        </Label>
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="dox-kd"
                            bind:value={model.dox_tta_kd}
                        />
                    </div>
                </div>
                <div class="flex justify-between">
                    <Button
                        variant="outline"
                        onclick={() => (doxView = "main")}
                        class="hover:cursor-pointer w-fit"
                    >
                        ← Back
                    </Button>
                </div>
            </div>
        </div>
    </Dialog.Content>
</Dialog.Root>
