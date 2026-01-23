<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    import * as Card from "$lib/components/ui/card/index.js";
    import * as Select from "$lib/components/ui/select/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import * as Table from "$lib/components/ui/table/index.js";
    import { AccessPeriod } from "$lib/models.svelte";
    import { browser } from "$app/environment";

    import { TetoffModel, DoxModel, TetoffState } from "$lib/models.svelte";
    import { Trash } from "@lucide/svelte";

    let { solution = $bindable(), summary = $bindable() } = $props();

    // simulation config
    let timeUnits = $state<"hr" | "min" | "s">("hr");
    let concentrationUnits = $state<"nM" | "µM">("nM");
    let t0 = $state<number>(0);
    let tf = $state<number>(504);
    let dt = $state<number>(1);

    // model
    let model = new TetoffModel();
    let initTta = $derived(model.tta_prod / model.tta_deg);
    let initState = $derived(new TetoffState(initTta));

    let initCondDialogOpen = $state<boolean>(false);
    let ttaDialogOpen = $state<boolean>(false);
    let doxDialogOpen = $state<boolean>(false);

    let doxDose = $state<number>(0);
    let doxT0 = $state<number>(0);
    let doxT1 = $state<number>(0);
    let doxView = $state<"main" | "pk">("main");

    async function run_simulation() {
        [solution, summary] = await model.simulate(initState, t0, tf, dt);
    }

    const isMac = browser && navigator.userAgent.toUpperCase().includes("MAC");

    function handleKeyDown(event: KeyboardEvent) {
        if ((event.metaKey || event.ctrlKey) && event.key === "Enter") {
            event.preventDefault();
            run_simulation();
        }
    }

    function pushDoxAccessPeriod(period: AccessPeriod) {
        model.dox_pk_model.schedule.push(period);
    }

    function deleteDoxAccessPeriod(index: number) {
        model.dox_pk_model.schedule.splice(index, 1);
    }
</script>

<Card.Root>
    <Card.Header>
        <Card.Title>TetOff RMA</Card.Title>
        <Card.Description class="grid gap-2">
            <p>Inducible RMA expression using the TetOff system.</p>
        </Card.Description>
    </Card.Header>
    <Card.Content>
        <form class="flex flex-col gap-3" onsubmit={run_simulation}>
            <!-- main config -->
            <h1 class="font-bold">Simulation Parameters</h1>
            <div class="flex justify-between">
                <div class="flex gap-2">
                    <Label for="time-units">Time Scale</Label>
                    <Select.Root
                        type="single"
                        bind:value={timeUnits}
                        name="timeUnits"
                    >
                        <Select.Trigger>{timeUnits}</Select.Trigger>
                        <Select.Content>
                            <Select.Item value="hr" label="Hours" />
                            <Select.Item value="min" label="Minutes" />
                            <Select.Item value="s" label="Seconds" />
                        </Select.Content>
                    </Select.Root>
                </div>
                <div class="flex gap-2">
                    <Label for="concentration-units">Concentration</Label>
                    <Select.Root
                        type="single"
                        bind:value={concentrationUnits}
                        name="concentrationUnits"
                    >
                        <Select.Trigger>{concentrationUnits}</Select.Trigger>
                        <Select.Content>
                            <Select.Item value="nM" label="nM" />
                            <Select.Item value="uM" label="µM" />
                        </Select.Content>
                    </Select.Root>
                </div>
            </div>
            <div class="flex flex-row gap-2">
                <div class="grid gap-2">
                    <Label for="t0">Start Time ({timeUnits})</Label>
                    <Input
                        type="number"
                        min="0"
                        step="any"
                        id="t0"
                        bind:value={t0}
                    />
                </div>
                <div class="grid gap-2">
                    <Label for="tf">Stop Time ({timeUnits})</Label>
                    <Input
                        type="number"
                        min="0"
                        step="any"
                        id="tf"
                        bind:value={tf}
                    />
                </div>
                <div class="grid gap-2">
                    <Label for="dt">Step size ({timeUnits})</Label>
                    <Input
                        type="number"
                        min="0"
                        step="1"
                        id="dt"
                        bind:value={dt}
                    />
                </div>
            </div>
            <h1 class="font-bold">RMA Rates</h1>
            <div class="grid gap-2">
                <Label for="rma-prod-rate"
                    >Production Rate ({concentrationUnits}/{timeUnits})</Label
                >
                <Input
                    type="number"
                    min="0"
                    step="any"
                    id="rma-prod-rate"
                    bind:value={model.rma_prod}
                />
            </div>
            <div class="grid gap-2">
                <Label for="leaky-rma-prod-rate"
                    >Leaky Production Rate ({concentrationUnits}/{timeUnits})</Label
                >
                <Input
                    type="number"
                    min="0"
                    step="any"
                    id="leaky-rma-prod-rate"
                    bind:value={model.leaky_rma_prod}
                />
            </div>
            <div class="grid gap-2">
                <Label for="rma-rt-rate"
                    >Reverse Transcytosis Rate (1/{timeUnits})</Label
                >
                <Input
                    type="number"
                    min="0"
                    step="any"
                    id="rma-rt-rate"
                    bind:value={model.rma_bbb_transport}
                />
            </div>
            <div class="grid gap-2">
                <Label for="rma-rt-rate">Degradation Rate (1/{timeUnits})</Label
                >
                <Input
                    type="number"
                    min="0"
                    step="any"
                    id="rma-rt-rate"
                    bind:value={model.rma_deg}
                />
            </div>

            <!-- tTA config -->
            <Dialog.Root bind:open={ttaDialogOpen}>
                <Dialog.Trigger class={buttonVariants({ variant: "outline" })}
                    >tTA Parameters</Dialog.Trigger
                >
                <Dialog.Content>
                    <div class="grid gap-2">
                        <Label for="tta-prod-rate"
                            >Production Rate ({concentrationUnits}/{timeUnits})</Label
                        >
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="tta-prod-rate"
                            bind:value={model.tta_prod}
                        />
                    </div>
                    <div class="grid gap-2">
                        <Label for="tta-deg-rate"
                            >Degradation Rate (1/{timeUnits})</Label
                        >
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="tta-deg-rate"
                            bind:value={model.tta_deg}
                        />
                    </div>
                    <div class="grid gap-2">
                        <Label for="tta-kd">Kd ({concentrationUnits})</Label>
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="tta-kd"
                            bind:value={model.tta_kd}
                        />
                    </div>
                    <div class="grid gap-2">
                        <Label for="tta-coop"
                            >tTA-Dox Binding Hill Coefficient</Label
                        >
                        <Input
                            type="number"
                            min="1"
                            step="1"
                            id="tta-coop"
                            bind:value={model.tta_cooperativity}
                        />
                    </div>
                </Dialog.Content>
            </Dialog.Root>

            <!-- Dox Config -->
            <Dialog.Root
                bind:open={doxDialogOpen}
                onOpenChange={() => (doxView = "main")}
            >
                <Dialog.Trigger class={buttonVariants({ variant: "outline" })}
                    >Dox Dose</Dialog.Trigger
                >
                <Dialog.Content class="overflow-hidden p-0">
                    <div
                        class="flex transition-transform duration-300 ease-in-out"
                        style="transform: translateX({doxView === 'main'
                            ? '0%'
                            : '-100%'})"
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
                                    <Label for="dox-start"
                                        >Start Time ({timeUnits})</Label
                                    >
                                    <Input
                                        type="number"
                                        min="0"
                                        step="any"
                                        id="dox-start"
                                        bind:value={doxT0}
                                    />
                                </div>
                                <div class="grid gap-2">
                                    <Label for="dox-stop"
                                        >Stop Time ({timeUnits})</Label
                                    >
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
                                        new AccessPeriod(doxDose, [
                                            doxT0,
                                            doxT1,
                                        ]),
                                    )}>Add new dox access period</Button
                            >
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
                                            <Table.Head class="font-bold"
                                                >Dose</Table.Head
                                            >
                                            <Table.Head class="font-bold"
                                                >Start Time</Table.Head
                                            >
                                            <Table.Head class="font-bold"
                                                >Stop Time</Table.Head
                                            >
                                        </Table.Row>
                                    </Table.Header>
                                    <Table.Body class="scroll-auto">
                                        {#each model.dox_pk_model.schedule as schedule, index}
                                            <Table.Row>
                                                <Table.Cell
                                                    >{schedule.dose}</Table.Cell
                                                >
                                                <Table.Cell
                                                    >{schedule
                                                        .time[0]}</Table.Cell
                                                >
                                                <Table.Cell
                                                    >{schedule
                                                        .time[1]}</Table.Cell
                                                >
                                                <Table.Cell class="">
                                                    <Button
                                                        variant="ghost"
                                                        class="text-destructive"
                                                        onclick={() =>
                                                            deleteDoxAccessPeriod(
                                                                index,
                                                            )}
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
                                    <Label for="dox-intake"
                                        >Food Intake (mg/{timeUnits})</Label
                                    >
                                    <Input
                                        type="number"
                                        min="0"
                                        step="any"
                                        id="dox-intake"
                                        bind:value={
                                            model.dox_pk_model.vehicle_intake
                                        }
                                    />
                                </div>
                                <div class="grid gap-2">
                                    <Label for="dox-bioavailability"
                                        >Bioavailability [0, 1]</Label
                                    >
                                    <Input
                                        type="number"
                                        min="0"
                                        max="1"
                                        step="any"
                                        id="dox-bioavailability"
                                        bind:value={
                                            model.dox_pk_model.bioavailability
                                        }
                                    />
                                </div>
                            </div>
                            <div class="flex justify-between gap-2">
                                <div class="grid gap-2">
                                    <Label for="dox-absorption"
                                        >Absorption Rate (1/{timeUnits})</Label
                                    >
                                    <Input
                                        type="number"
                                        min="0"
                                        step="any"
                                        id="dox-absorption"
                                        bind:value={
                                            model.dox_pk_model.absorption
                                        }
                                    />
                                </div>
                                <div class="grid gap-2">
                                    <Label for="dox-elimination"
                                        >Elimination Rate (1/{timeUnits})</Label
                                    >
                                    <Input
                                        type="number"
                                        min="0"
                                        step="any"
                                        id="dox-elimination"
                                        bind:value={
                                            model.dox_pk_model.elimination
                                        }
                                    />
                                </div>
                            </div>
                            <div class="flex justify-between gap-2">
                                <div class="grid gap-2">
                                    <Label for="dox-brain-transport"
                                        >Brain Transport Rate (1/{timeUnits})</Label
                                    >
                                    <Input
                                        type="number"
                                        min="0"
                                        step="any"
                                        id="dox-brain-transport"
                                        bind:value={
                                            model.dox_pk_model.brain_transport
                                        }
                                    />
                                </div>
                                <div class="grid gap-2">
                                    <Label for="dox-plasma-transport"
                                        >Plasma Transport Rate (1/{timeUnits})</Label
                                    >
                                    <Input
                                        type="number"
                                        min="0"
                                        step="any"
                                        id="dox-plasma-transport"
                                        bind:value={
                                            model.dox_pk_model.plasma_transport
                                        }
                                    />
                                </div>
                            </div>
                            <div class="flex justify-between gap-2">
                                <div class="grid gap-2">
                                    <Label for="dox-vd"
                                        >Volume of Distribution (L)</Label
                                    >
                                    <Input
                                        type="number"
                                        min="0"
                                        step="any"
                                        id="dox-vd"
                                        bind:value={
                                            model.dox_pk_model.plasma_vd
                                        }
                                    />
                                </div>
                                <div class="grid gap-2">
                                    <Label for="dox-kd"
                                        >Dox-tTA Kd ({concentrationUnits})</Label
                                    >
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

            <!-- Initial conditions -->
            <Dialog.Root bind:open={initCondDialogOpen}>
                <Dialog.Trigger class={buttonVariants({ variant: "outline" })}
                    >Initial Conditions</Dialog.Trigger
                >
                <Dialog.Content
                    >>
                    <div class="flex justify-between gap-2">
                        <div class="grid gap-2">
                            <Label for="init-brain-rma"
                                >Brain RMA ({concentrationUnits})</Label
                            >
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="init-brain-rma"
                                bind:value={initState.brain_rma}
                            />
                        </div>
                        <div class="grid gap-2">
                            <Label for="init-plasma-rma"
                                >Plasma RMA ({concentrationUnits})</Label
                            >
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="init-plasma-rma"
                                bind:value={initState.plasma_rma}
                            />
                        </div>
                    </div>
                    <div class="grid gap-2">
                        <Label for="init-tta">tTA ({concentrationUnits})</Label>
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="init-tta"
                            bind:value={initState.tta}
                        />
                    </div>
                    <div class="flex justify-between gap-2">
                        <div class="grid gap-2">
                            <Label for="init-plasma-dox"
                                >Plasma Dox ({concentrationUnits})</Label
                            >
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="init-plasma-dox"
                                bind:value={initState.plasma_dox}
                            />
                        </div>
                        <div class="grid gap-2">
                            <Label for="init-brain-dox"
                                >Brain Dox ({concentrationUnits})</Label
                            >
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="init-brain-dox"
                                bind:value={initState.brain_dox}
                            />
                        </div>
                    </div>
                    <div class="flex justify-between">
                        <Button
                            variant="outline"
                            onclick={() => (initCondDialogOpen = false)}
                            class="hover:cursor-pointer">Cancel</Button
                        >
                        <div class="flex justify-evenly gap-2">
                            <Button
                                variant="destructive"
                                onclick={() => initState.reset()}
                                class="hover:cursor-pointer">Reset</Button
                            >
                            <Button
                                onclick={() => (initCondDialogOpen = false)}
                                class="hover:cursor-pointer">Save</Button
                            >
                        </div>
                    </div>
                </Dialog.Content>
            </Dialog.Root>
            <Button type="submit" class="hover:cursor-pointer">
                Run Simulation
                <span class="opacity-75">
                    {#if isMac}
                        `⌘+Return`
                    {:else}
                        `Ctrl+Enter`
                    {/if}
                </span>
            </Button>
        </form>
    </Card.Content>
</Card.Root>
