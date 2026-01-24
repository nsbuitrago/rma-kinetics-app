<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { buttonVariants } from "$lib/components/ui/button/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Button } from "$lib/components/ui/button/index.js";

    let {
        initState = $bindable(),
        simulationParams = $bindable(),
        dialogOpen = $bindable(),
    } = $props();
</script>

<Dialog.Root bind:open={dialogOpen}>
    <Dialog.Trigger class={buttonVariants({ variant: "outline" })}>
        Initial Conditions
    </Dialog.Trigger>
    <Dialog.Content>
        <div class="flex justify-between gap-2">
            <div class="grid gap-2">
                <Label for="init-brain-rma">
                    Brain RMA ({simulationParams.concentrationUnits})
                </Label>
                <Input
                    type="number"
                    min="0"
                    step="any"
                    id="init-brain-rma"
                    bind:value={initState.brain_rma}
                />
            </div>
            <div class="grid gap-2">
                <Label for="init-plasma-rma">
                    Plasma RMA ({simulationParams.concentrationUnits})
                </Label>
                <Input
                    type="number"
                    min="0"
                    step="any"
                    id="init-plasma-rma"
                    bind:value={initState.plasma_rma}
                />
            </div>
        </div>

        <!-- TetOff/Chemogenetic: tTA -->
        {#if 'tta' in initState}
            <div class="grid gap-2">
                <Label for="init-tta">
                    tTA ({simulationParams.concentrationUnits})
                </Label>
                <Input
                    type="number"
                    min="0"
                    step="any"
                    id="init-tta"
                    bind:value={initState.tta}
                />
            </div>
        {/if}

        <!-- TetOff/Chemogenetic: Dox -->
        {#if 'plasma_dox' in initState}
            <div class="flex justify-between gap-2">
                <div class="grid gap-2">
                    <Label for="init-plasma-dox">
                        Plasma Dox ({simulationParams.concentrationUnits})
                    </Label>
                    <Input
                        type="number"
                        min="0"
                        step="any"
                        id="init-plasma-dox"
                        bind:value={initState.plasma_dox}
                    />
                </div>
                <div class="grid gap-2">
                    <Label for="init-brain-dox">
                        Brain Dox ({simulationParams.concentrationUnits})
                    </Label>
                    <Input
                        type="number"
                        min="0"
                        step="any"
                        id="init-brain-dox"
                        bind:value={initState.brain_dox}
                    />
                </div>
            </div>
        {/if}

        <!-- Chemogenetic: DREADD -->
        {#if 'dreadd' in initState}
            <div class="grid gap-2">
                <Label for="init-dreadd">
                    hM3Dq ({simulationParams.concentrationUnits})
                </Label>
                <Input
                    type="number"
                    min="0"
                    step="any"
                    id="init-dreadd"
                    bind:value={initState.dreadd}
                />
            </div>
        {/if}

        <!-- Chemogenetic: CNO -->
        {#if 'peritoneal_cno' in initState}
            <div class="flex justify-between gap-2">
                <div class="grid gap-2">
                    <Label for="init-peritoneal-cno">Peritoneal CNO (nmol)</Label>
                    <Input
                        type="number"
                        min="0"
                        step="any"
                        id="init-peritoneal-cno"
                        bind:value={initState.peritoneal_cno}
                    />
                </div>
                <div class="grid gap-2">
                    <Label for="init-plasma-cno">Plasma CNO (nmol)</Label>
                    <Input
                        type="number"
                        min="0"
                        step="any"
                        id="init-plasma-cno"
                        bind:value={initState.plasma_cno}
                    />
                </div>
            </div>
            <div class="grid gap-2">
                <Label for="init-brain-cno">Brain CNO (nmol)</Label>
                <Input
                    type="number"
                    min="0"
                    step="any"
                    id="init-brain-cno"
                    bind:value={initState.brain_cno}
                />
            </div>
        {/if}

        <!-- Chemogenetic: CLZ -->
        {#if 'plasma_clz' in initState}
            <div class="flex justify-between gap-2">
                <div class="grid gap-2">
                    <Label for="init-plasma-clz">Plasma CLZ (nmol)</Label>
                    <Input
                        type="number"
                        min="0"
                        step="any"
                        id="init-plasma-clz"
                        bind:value={initState.plasma_clz}
                    />
                </div>
                <div class="grid gap-2">
                    <Label for="init-brain-clz">Brain CLZ (nmol)</Label>
                    <Input
                        type="number"
                        min="0"
                        step="any"
                        id="init-brain-clz"
                        bind:value={initState.brain_clz}
                    />
                </div>
            </div>
        {/if}

        <Button variant="outline" onclick={() => initState.reset()}>
            Reset
        </Button>
    </Dialog.Content>
</Dialog.Root>
