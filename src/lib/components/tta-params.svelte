<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import { buttonVariants } from "$lib/components/ui/button/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Input } from "$lib/components/ui/input/index.js";

    let {
        model = $bindable(),
        simulationParams = $bindable(),
        dialogOpen = $bindable(),
    } = $props();
</script>

<Dialog.Root bind:open={dialogOpen}>
    <Dialog.Trigger class={buttonVariants({ variant: "outline" })}>
        tTA Parameters
    </Dialog.Trigger>
    <Dialog.Content>
        <div class="grid gap-2">
            <Label for="tta-prod-rate">
                Production Rate ({simulationParams.concentrationUnits}/{simulationParams.timeUnits})
            </Label>
            <Input
                type="number"
                min="0"
                step="any"
                id="tta-prod-rate"
                bind:value={model.tta_prod}
            />
        </div>
        {#if 'leaky_tta_prod' in model}
            <div class="grid gap-2">
                <Label for="leaky-tta-prod-rate">
                    Leaky Production Rate ({simulationParams.concentrationUnits}/{simulationParams.timeUnits})
                </Label>
                <Input
                    type="number"
                    min="0"
                    step="any"
                    id="leaky-tta-prod-rate"
                    bind:value={model.leaky_tta_prod}
                />
            </div>
        {/if}
        <div class="grid gap-2">
            <Label for="tta-deg-rate">
                Degradation Rate (1/{simulationParams.timeUnits})
            </Label>
            <Input
                type="number"
                min="0"
                step="any"
                id="tta-deg-rate"
                bind:value={model.tta_deg}
            />
        </div>
        <div class="grid gap-2">
            <Label for="tta-kd">Kd ({simulationParams.concentrationUnits})</Label>
            <Input
                type="number"
                min="0"
                step="any"
                id="tta-kd"
                bind:value={model.tta_kd}
            />
        </div>
        <div class="grid gap-2">
            <Label for="tta-coop">tTA-Dox Binding Hill Coefficient</Label>
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
