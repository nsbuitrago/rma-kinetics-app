<script lang="ts">
    import { invoke } from "@tauri-apps/api/core";

    import * as Card from "$lib/components/ui/card/index.js";
    import * as Select from "$lib/components/ui/select/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";

    let { solution = $bindable() } = $props();

    // simulation config
    let timeUnits = $state<"hr" | "min" | "s">("hr");
    let concentrationUnits = $state<"nM" | "µM">("nM");
    let tf = $state<number>(504);

    // RMA rates
    let rmaProdRate = $state<number>(0.2);
    let leakyRmaProdRate = $state<number>(0.002);
    let rmaRtRate = $state<number>(0.6);
    let rmaDegRate = $state<number>(0.007);

    // tTA rates
    let ttaProdRate = $state<number>(10);
    let ttaDegRate = $state<number>(1);
    let ttaKd = $state<number>(10);
    let ttaCoop = $state<number>(2);

    // dox rates and dose
    let doxDose = $state<number>(40);
    let doxT0 = $state<number>(0);
    let doxT1 = $state<number>(96);
    let foodIntake = $state<number>(0.0001875);
    let doxBioavailability = $state<number>(0.9);
    let doxAbsorptionRate = $state<number>(0.8);
    let doxEliminationRate = $state<number>(0.2);
    let doxBrainTransportRate = $state<number>(0.2);
    let doxPlasmaTransportRate = $state<number>(1);
    let doxPlasmaVd = $state<number>(0.21);
    let doxKd = $state<number>(10);

    // initial conditions
    let initBrainRMA = $state<number>(0);
    let initPlasmaRMA = $state<number>(0);
    let initTta = $derived(ttaProdRate / ttaDegRate);
    let initBrainDox = $state<number>(0);
    let initPlasmaDox = $state<number>(0);

    let initCondDialogOpen = $state<boolean>(false);
    let ttaDialogOpen = $state<boolean>(false);
    let doxDialogOpen = $state<boolean>(false);

    function resetInitConditions() {
        initBrainRMA = 0;
        initPlasmaRMA = 0;
        initTta = 0;
    }

    async function run_simulation() {
        solution = await invoke("tetoff_model", {
            rma_config: {
                prod_rate: rmaProdRate,
                leaky_prod_rate: leakyRmaProdRate,
                rt_rate: rmaRtRate,
                deg_rate: rmaDegRate,
            },
            tta_config: {
                prod_rate: ttaProdRate,
                leaky_prod_rate: 0,
                deg_rate: ttaDegRate,
                tta_kd: ttaKd,
                tta_coop: ttaCoop,
            },
            dox_config: {
                dose: doxDose,
                t0: doxT0,
                t1: doxT1,
                vehicle_intake_rate: foodIntake,
                bioavailability: doxBioavailability,
                absorption_rate: doxAbsorptionRate,
                elimination_rate: doxEliminationRate,
                brain_transport_rate: doxBrainTransportRate,
                plasma_transport_rate: doxPlasmaTransportRate,
                plasma_vd: doxPlasmaVd,
                dox_kd: doxKd,
            },
            init: [
                initBrainRMA,
                initPlasmaRMA,
                initTta,
                initBrainDox,
                initPlasmaDox,
            ],
            tf: tf,
        });
    }
</script>

<Card.Root>
    <Card.Header>
        <Card.Title>TetOff RMA</Card.Title>
        <Card.Description class="grid gap-2">
            <p>tTA induced RMA expression using the TetOff system.</p>
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
            <div class="grid gap-2">
                <Label for="t1">Stop Time ({timeUnits})</Label>
                <Input
                    type="number"
                    min="0"
                    step="any"
                    id="tf"
                    bind:value={tf}
                />
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
                    bind:value={rmaProdRate}
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
                    bind:value={leakyRmaProdRate}
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
                    bind:value={rmaRtRate}
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
                    bind:value={rmaDegRate}
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
                            bind:value={ttaProdRate}
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
                            bind:value={ttaDegRate}
                        />
                    </div>
                    <div class="grid gap-2">
                        <Label for="tta-kd">Kd ({concentrationUnits})</Label>
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="tta-kd"
                            bind:value={ttaKd}
                        />
                    </div>
                    <div class="grid gap-2">
                        <Label for="tta-coop"
                            >tTA-Dox Binding Hill Coefficient</Label
                        >
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="tta-coop"
                            bind:value={ttaCoop}
                        />
                    </div>
                    <div class="flex justify-between">
                        <Button
                            variant="outline"
                            onclick={() => (ttaDialogOpen = false)}
                            class="hover:cursor-pointer">Cancel</Button
                        >
                        <div class="flex justify-evenly gap-2">
                            <Button
                                onclick={() => (ttaDialogOpen = false)}
                                class="hover:cursor-pointer">Save</Button
                            >
                        </div>
                    </div>
                </Dialog.Content>
            </Dialog.Root>

            <!-- Dox Config -->
            <Dialog.Root bind:open={doxDialogOpen}>
                <Dialog.Trigger class={buttonVariants({ variant: "outline" })}
                    >Dox Dose</Dialog.Trigger
                >
                <Dialog.Content>
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
                            <Label for="dox-stop">Stop Time ({timeUnits})</Label
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
                                bind:value={foodIntake}
                            />
                        </div>
                        <div class="grid gap-2">
                            <Label for="dox-bioavailability"
                                >Bioavailability (%)</Label
                            >
                            <Input
                                type="number"
                                min="0"
                                max="1"
                                step="any"
                                id="dox-bioavailability"
                                bind:value={doxBioavailability}
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
                                bind:value={doxAbsorptionRate}
                            />
                        </div>
                        <div class="grid gap-2">
                            <Label for="dox-elimination"
                                >Elimination Rate (1/{timeUnits})</Label
                            >
                            <Input
                                type="number"
                                min="0"
                                max="1"
                                step="any"
                                id="dox-elimination"
                                bind:value={doxEliminationRate}
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
                                bind:value={doxBrainTransportRate}
                            />
                        </div>
                        <div class="grid gap-2">
                            <Label for="dox-plasma-transport"
                                >Plasma Transport Rate (1/{timeUnits})</Label
                            >
                            <Input
                                type="number"
                                min="0"
                                max="1"
                                step="any"
                                id="dox-plasma-transport"
                                bind:value={doxPlasmaTransportRate}
                            />
                        </div>
                    </div>
                    <div class="flex justify-between gap-2">
                        <div class="grid gap-2">
                            <Label for="dox-vd"
                                >Volume of distribution (L)</Label
                            >
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="dox-vd"
                                bind:value={doxPlasmaVd}
                            />
                        </div>
                        <div class="grid gap-2">
                            <Label for="dox-kd"
                                >Dox-tTA Kd ({concentrationUnits})</Label
                            >
                            <Input
                                type="number"
                                min="0"
                                max="1"
                                step="any"
                                id="dox-plasma-transport"
                                bind:value={doxPlasmaTransportRate}
                            />
                        </div>
                    </div>
                    <div class="flex justify-between">
                        <Button
                            variant="outline"
                            onclick={() => (doxDialogOpen = false)}
                            class="hover:cursor-pointer">Cancel</Button
                        >
                        <div class="flex justify-evenly gap-2">
                            <Button
                                onclick={() => (doxDialogOpen = false)}
                                class="hover:cursor-pointer">Save</Button
                            >
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
                                bind:value={initBrainRMA}
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
                                bind:value={initPlasmaRMA}
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
                            bind:value={initTta}
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
                                bind:value={initPlasmaDox}
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
                                bind:value={initBrainDox}
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
                                onclick={resetInitConditions}
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
            <Button type="submit" class="hover:cursor-pointer"
                >Run Simulation</Button
            >
        </form>
    </Card.Content>
</Card.Root>
