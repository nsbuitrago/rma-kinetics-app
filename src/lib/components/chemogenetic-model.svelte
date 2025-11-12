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
    let rmaProdRate = $state<number>(0.428);
    let leakyRmaProdRate = $state<number>(7.01e-3);
    let rmaRtRate = $state<number>(0.727);
    let rmaDegRate = $state<number>(5.5e-3);

    // tTA rates
    let ttaProdRate = $state<number>(12.46);
    let leakyTtaProdRate = $state<number>(1.22e-1);
    let ttaDegRate = $state<number>(2.81e-2);
    let ttaKd = $state<number>(4.19);
    let ttaCoop = $state<number>(2);

    // DREADD rates
    let dqProdRate = $state<number>(8.05);
    let dqDegRate = $state<number>(1);
    let dqEc50 = $state<number>(6.79);
    let dqCoop = $state<number>(1);

    // dox rates and dose
    let doxDose = $state<number>(40);
    let doxT0 = $state<number>(0);
    let doxT1 = $state<number>(0);
    let foodIntake = $state<number>(0.0001875);
    let doxBioavailability = $state<number>(0.9);
    let doxAbsorptionRate = $state<number>(0.8);
    let doxEliminationRate = $state<number>(0.2);
    let doxBrainTransportRate = $state<number>(0.2);
    let doxPlasmaTransportRate = $state<number>(1);
    let doxPlasmaVd = $state<number>(0.021);
    let doxKd = $state<number>(10);

    // cno rates and dose
    let cnoDose = $state<number>(0.03);
    let cnoT0 = $state<number>(48);
    let cnoAbsorptionRate = $state<number>(23.94);
    let cnoEliminationRate = $state<number>(5.51e-2);
    let cnoRevMetRate = $state<number>(1.44);
    let clzMetRate = $state<number>(3e-1);
    let cnoBrainTransportRate = $state<number>(2.33);
    let cnoPlasmaTransportRate = $state<number>(71.85);
    let clzBrainTransportRate = $state<number>(35.61);
    let clzPlasmaTransportRate = $state<number>(34.07);
    let clzEliminationRate = $state<number>(3.94);
    let cnoPlasmaVd = $state<number>(3.99e-2);
    let cnoBrainVd = $state<number>(0.21);
    let clzPlasmaVd = $state<number>(0.24);
    let clzBrainVd = $state<number>(8.87e-2);
    let cnoEc50 = $state<number>(7.97);
    let clzEc50 = $state<number>(4.34);
    let cnoCoop = $state<number>(1);
    let clzCoop = $state<number>(1);

    // initial conditions
    let initBrainRMA = $state<number>(0);
    let initPlasmaRMA = $state<number>(0);
    let initTta = $state<number>(0);
    let initBrainDox = $state<number>(0);
    let initPlasmaDox = $state<number>(0);
    let initDq = $derived<number>(dqProdRate / dqDegRate);
    let initPeritonealCno = $state<number>(0);
    let initPlasmaCno = $state<number>(0);
    let initBrainCno = $state<number>(0);
    let initPlasmaClz = $state<number>(0);
    let initBrainClz = $state<number>(0);

    let initCondDialogOpen = $state<boolean>(false);
    let ttaDialogOpen = $state<boolean>(false);
    let dqDialogOpen = $state<boolean>(false);
    let doxDialogOpen = $state<boolean>(false);
    let cnoDialogOpen = $state<boolean>(false);

    function resetInitConditions() {
        initBrainRMA = 0;
        initPlasmaRMA = 0;
        initTta = 0;
        initBrainDox = 0;
        initPlasmaDox = 0;
        initDq = dqProdRate / dqDegRate;
        initPeritonealCno = 0;
        initBrainCno = 0;
        initPlasmaCno = 0;
        initBrainClz = 0;
        initPlasmaClz = 0;
    }

    async function run_simulation() {
        solution = await invoke("chemogenetic_model", {
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
            dq_config: {
                prod_rate: dqProdRate,
                deg_rate: dqDegRate,
                ec50: dqEc50,
                coop: dqCoop,
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
            cno_config: {
                dose: cnoDose,
                t0: cnoT0,
                cno_absorption_rate: cnoAbsorptionRate,
                cno_elimination_rate: cnoEliminationRate,
                cno_reverse_metabolism_rate: cnoRevMetRate,
                clz_metabolism_rate: clzMetRate,
                cno_brain_transport_rate: cnoBrainTransportRate,
                cno_plasma_transport_rate: cnoPlasmaTransportRate,
                clz_brain_transport_rate: clzBrainTransportRate,
                clz_plasma_transport_rate: clzPlasmaTransportRate,
                clz_elimination_rate: clzEliminationRate,
                cno_plasma_vd: cnoPlasmaVd,
                cno_brain_vd: cnoBrainVd,
                clz_plasma_vd: clzPlasmaVd,
                clz_brain_vd: clzBrainVd,
                cno_ec50: cnoEc50,
                clz_ec50: clzEc50,
                cno_coop: cnoCoop,
                clz_coop: clzCoop,
            },
            init: [
                initBrainRMA,
                initPlasmaRMA,
                initTta,
                initBrainDox,
                initPlasmaDox,
                initDq,
                initPeritonealCno,
                initBrainCno,
                initPlasmaCno,
                initBrainClz,
                initPlasmaClz,
            ],
            tf: tf,
        });
    }
</script>

<Card.Root>
    <Card.Header>
        <Card.Title>Chemogenetic RMA</Card.Title>
        <Card.Description class="grid gap-2">
            <p>
                RMA expression induced by neuronal activity using the activatory
                designed receptor hM3Dq and optional doxycycline inhibition with
                Tet-Off gating.
            </p>
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
                        <Label for="leaky-tta-prod-rate"
                            >Leaky Production Rate ({concentrationUnits}/{timeUnits})</Label
                        >
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="leaky-tta-prod-rate"
                            bind:value={leakyTtaProdRate}
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
                            min="1"
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

            <!-- Dq config -->
            <Dialog.Root bind:open={dqDialogOpen}>
                <Dialog.Trigger class={buttonVariants({ variant: "outline" })}
                    >hM3Dq Parameters</Dialog.Trigger
                >
                <Dialog.Content>
                    <div class="grid gap-2">
                        <Label for="dq-prod-rate"
                            >Production Rate ({concentrationUnits}/{timeUnits})</Label
                        >
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="dq-prod-rate"
                            bind:value={dqProdRate}
                        />
                    </div>
                    <div class="grid gap-2">
                        <Label for="dq-deg-rate"
                            >Degradation Rate (1/{timeUnits})</Label
                        >
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="dq-deg-rate"
                            bind:value={dqDegRate}
                        />
                    </div>
                    <div class="grid gap-2">
                        <Label for="dq-ec50">EC50 ({concentrationUnits})</Label>
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="dq-ec50"
                            bind:value={dqEc50}
                        />
                    </div>
                    <div class="grid gap-2">
                        <Label for="dq-coop">hM3Dq Hill Coefficient</Label>
                        <Input
                            type="number"
                            min="1"
                            step="any"
                            id="dq-coop"
                            bind:value={dqCoop}
                        />
                    </div>
                    <div class="flex justify-between">
                        <Button
                            variant="outline"
                            onclick={() => (dqDialogOpen = false)}
                            class="hover:cursor-pointer">Cancel</Button
                        >
                        <div class="flex justify-evenly gap-2">
                            <Button
                                onclick={() => (dqDialogOpen = false)}
                                class="hover:cursor-pointer">Save</Button
                            >
                        </div>
                    </div>
                </Dialog.Content>
            </Dialog.Root>

            <div class="flex justify-evenly">
                <!-- Dox Config -->
                <Dialog.Root bind:open={doxDialogOpen}>
                    <Dialog.Trigger
                        class={buttonVariants({ variant: "outline" })}
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
                                    >Bioavailability [0, 1]</Label
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
                                    step="any"
                                    id="dox-kd"
                                    bind:value={doxKd}
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

                <!-- CNO Config -->
                <Dialog.Root bind:open={cnoDialogOpen}>
                    <Dialog.Trigger
                        class={buttonVariants({ variant: "outline" })}
                        >CNO Dose</Dialog.Trigger
                    >
                    <Dialog.Content>
                        <div class="flex justify-between gap-2">
                            <div class="grid gap-2">
                                <Label for="cno-dose">Dose (mg/kg)</Label>
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="cno-dose"
                                    bind:value={cnoDose}
                                />
                            </div>
                            <div class="grid gap-2">
                                <Label for="cno-t0"
                                    >Injection Time ({timeUnits})</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="dox-t0"
                                    bind:value={cnoT0}
                                />
                            </div>
                        </div>
                        <div class="flex justify-between gap-2">
                            <div class="grid gap-2">
                                <Label for="cno-abs" class="text-center"
                                    >CNO Absorption Rate (1/{timeUnits})</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="cno-abs"
                                    bind:value={cnoAbsorptionRate}
                                />
                            </div>
                            <div class="grid gap-2">
                                <Label for="cno-el" class="text-center"
                                    >CNO Elimination Rate (1/{timeUnits})</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="cno-el"
                                    bind:value={cnoEliminationRate}
                                />
                            </div>
                            <div class="grid gap-2">
                                <Label for="clz-el" class="text-center"
                                    >CLZ Elimination Rate (1/{timeUnits})</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="clz-el"
                                    bind:value={clzEliminationRate}
                                />
                            </div>
                        </div>
                        <div class="flex justify-between gap-2">
                            <div class="grid gap-2">
                                <Label for="cno-rev-met"
                                    >CNO Reverse Metabolism Rate (1/{timeUnits})</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="cno-rev-met"
                                    bind:value={cnoRevMetRate}
                                />
                            </div>
                            <div class="grid gap-2">
                                <Label for="clz--met"
                                    >CLZ Metabolism Rate (1/{timeUnits})</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="clz-met"
                                    bind:value={clzMetRate}
                                />
                            </div>
                        </div>
                        <div class="flex justify-between gap-2">
                            <div class="grid gap-2">
                                <Label for="cno-brain-trans"
                                    >CNO Brain Transport Rate (1/{timeUnits})</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="cno-brain-trans"
                                    bind:value={cnoBrainTransportRate}
                                />
                            </div>
                            <div class="grid gap-2">
                                <Label for="cno-plasma-trans"
                                    >CNO Plasma Transport Rate (1/{timeUnits})</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="cno-plasma-trans"
                                    bind:value={cnoPlasmaTransportRate}
                                />
                            </div>
                        </div>
                        <div class="flex justify-between gap-2">
                            <div class="grid gap-2">
                                <Label for="clz-brain-trans"
                                    >CLZ Brain Transport Rate (1/{timeUnits})</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="clz-brain-trans"
                                    bind:value={clzBrainTransportRate}
                                />
                            </div>
                            <div class="grid gap-2">
                                <Label for="clz-plasma-trans"
                                    >CLZ Plasma Transport Rate (1/{timeUnits})</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="clz-plasma-trans"
                                    bind:value={clzPlasmaTransportRate}
                                />
                            </div>
                        </div>
                        <div class="flex justify-between gap-2">
                            <div class="grid gap-2">
                                <Label for="cno-plasma-vd"
                                    >CNO Plasma Vd (L)</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="cno-plasma-vd"
                                    bind:value={cnoPlasmaVd}
                                />
                            </div>
                            <div class="grid gap-2">
                                <Label for="cno-brain-vd"
                                    >CNO Brain Vd (L)</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="cno-brain-vd"
                                    bind:value={cnoBrainVd}
                                />
                            </div>
                        </div>
                        <div class="flex justify-between gap-2">
                            <div class="grid gap-2">
                                <Label for="clz-plasma-vd"
                                    >CLZ Plasma Vd (L)</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="clz-plasma-vd"
                                    bind:value={clzPlasmaVd}
                                />
                            </div>
                            <div class="grid gap-2">
                                <Label for="clz-brain-vd"
                                    >CLZ Brain Vd (L)</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="clz-brain-vd"
                                    bind:value={clzBrainVd}
                                />
                            </div>
                        </div>
                        <div class="flex justify-between gap-2">
                            <div class="grid gap-2">
                                <Label for="cno-ec50"
                                    >CNO EC50 ({concentrationUnits})</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="cno-ec50"
                                    bind:value={cnoEc50}
                                />
                            </div>
                            <div class="grid gap-2">
                                <Label for="clz-ec50"
                                    >CLZ EC50 ({concentrationUnits})</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="clz-ec50"
                                    bind:value={clzEc50}
                                />
                            </div>
                        </div>
                        <div class="flex justify-between gap-2">
                            <div class="grid gap-2">
                                <Label for="cno-coop"
                                    >CNO-DREADD Hill Coefficient</Label
                                >
                                <Input
                                    type="number"
                                    min="1"
                                    step="any"
                                    id="cno-coop"
                                    bind:value={cnoCoop}
                                />
                            </div>
                            <div class="grid gap-2">
                                <Label for="clz-coop"
                                    >CLZ-DREADD Hill Coefficient</Label
                                >
                                <Input
                                    type="number"
                                    min="1"
                                    step="any"
                                    id="clz-coop"
                                    bind:value={clzCoop}
                                />
                            </div>
                        </div>
                        <div class="flex justify-between">
                            <Button
                                variant="outline"
                                onclick={() => (cnoDialogOpen = false)}
                                class="hover:cursor-pointer">Cancel</Button
                            >
                            <div class="flex justify-evenly gap-2">
                                <Button
                                    onclick={() => (cnoDialogOpen = false)}
                                    class="hover:cursor-pointer">Save</Button
                                >
                            </div>
                        </div>
                    </Dialog.Content>
                </Dialog.Root>

                <!-- Initial conditions -->
                <Dialog.Root bind:open={initCondDialogOpen}>
                    <Dialog.Trigger
                        class={buttonVariants({ variant: "outline" })}
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
                            <Label for="init-tta"
                                >tTA ({concentrationUnits})</Label
                            >
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
                        <div class="flex justify-between gap-2">
                            <div class="grid gap-2">
                                <Label for="init-dq"
                                    >hM3Dq ({concentrationUnits})</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="init-dq"
                                    bind:value={initDq}
                                />
                            </div>
                            <div class="grid gap-2">
                                <Label for="init-peritoneal-cno"
                                    >Peritoneal CNO (nmol)</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="init-peritoneal-cno"
                                    bind:value={initPeritonealCno}
                                />
                            </div>
                        </div>
                        <div class="flex justify-between gap-2">
                            <div class="grid gap-2">
                                <Label for="init-brain-cno"
                                    >Brain CNO (nmol)</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="init-brain-cno"
                                    bind:value={initBrainCno}
                                />
                            </div>
                            <div class="grid gap-2">
                                <Label for="init-plasma-cno"
                                    >Plasma CNO (nmol)</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="init-plasma-cno"
                                    bind:value={initPlasmaCno}
                                />
                            </div>
                        </div>
                        <div class="flex justify-between gap-2">
                            <div class="grid gap-2">
                                <Label for="init-brain-clz"
                                    >Brain CLZ (nmol)</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="init-brain-clz"
                                    bind:value={initBrainClz}
                                />
                            </div>
                            <div class="grid gap-2">
                                <Label for="init-plasma-clz"
                                    >Plasma CLZ (nmol)</Label
                                >
                                <Input
                                    type="number"
                                    min="0"
                                    step="any"
                                    id="init-plasma-clz"
                                    bind:value={initPlasmaClz}
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
            </div>

            <Button type="submit" class="hover:cursor-pointer"
                >Run Simulation</Button
            >
        </form>
    </Card.Content>
</Card.Root>
