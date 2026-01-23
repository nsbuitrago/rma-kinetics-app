<script lang="ts">
    import * as Card from "$lib/components/ui/card/index.js";
    import * as Select from "$lib/components/ui/select/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { browser } from "$app/environment";

    import { ConstitutiveModel, ConstitutiveState } from "$lib/models.svelte";

    let { solution = $bindable(), summary = $bindable() } = $props();

    // simulation config
    let timeUnits = $state<"hr" | "min" | "s">("hr");
    let concentrationUnits = $state<"nM" | "µM">("nM");
    let t0 = $state<number>(0);
    let tf = $state<number>(504);
    let dt = $state<number>(1);

    // model
    let model = new ConstitutiveModel();
    let init_state = new ConstitutiveState();
    let initCondDialogOpen = $state<boolean>(false);

    async function run_simulation() {
        [solution, summary] = await model.simulate(init_state, t0, tf, dt);
    }

    const isMac = browser && navigator.userAgent.toUpperCase().includes("MAC");

    function handleKeyDown(event: KeyboardEvent) {
        if ((event.metaKey || event.ctrlKey) && event.key === "Enter") {
            event.preventDefault();
            run_simulation();
        }
    }
</script>

<svelte:window onkeydown={handleKeyDown} />

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
                    bind:value={model.prod}
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
                    bind:value={model.bbbTransport}
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
                    bind:value={model.deg}
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
                            bind:value={init_state.brain_rma}
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
                            bind:value={init_state.plasma_rma}
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
                                onclick={() => init_state.reset()}
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
