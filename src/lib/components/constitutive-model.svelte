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
    let rmaRtRate = $state<number>(0.6);
    let rmaDegRate = $state<number>(0.007);

    // initial conditions
    let initBrainRMA = $state<number>(0);
    let initPlasmaRMA = $state<number>(0);

    let initCondDialogOpen = $state<boolean>(false);

    function resetInitConditions() {
        initBrainRMA = 0;
        initPlasmaRMA = 0;
    }

    async function run_simulation() {
        solution = await invoke("constitutive_model", {
            rma_prod_rate: rmaProdRate,
            rma_rt_rate: rmaRtRate,
            rma_deg_rate: rmaDegRate,
            init: [initBrainRMA, initPlasmaRMA],
            tf: tf,
        });
    }
</script>

<Card.Root>
    <Card.Header>
        <Card.Title>Constitutive RMA</Card.Title>
        <Card.Description class="grid gap-2">
            <p>
                RMA expression driven by a constitutive promoter such as hSyn,
                EF1a, CAG, etc.
            </p>
        </Card.Description>
    </Card.Header>
    <Card.Content>
        <form class="flex flex-col gap-3" onsubmit={run_simulation}>
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
            <Dialog.Root bind:open={initCondDialogOpen}>
                <Dialog.Trigger class={buttonVariants({ variant: "outline" })}
                    >Initial Conditions</Dialog.Trigger
                >
                <Dialog.Content>
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
