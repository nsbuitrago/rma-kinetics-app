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
        hM3Dq Parameters
    </Dialog.Trigger>
    <Dialog.Content>
        <div class="grid gap-2">
            <Label for="dq-prod-rate">
                Production Rate ({simulationParams.concentrationUnits}/{simulationParams.timeUnits})
            </Label>
            <Input
                type="number"
                min="0"
                step="any"
                id="dq-prod-rate"
                bind:value={model.dreadd_prod}
            />
        </div>
        <div class="grid gap-2">
            <Label for="dq-deg-rate">
                Degradation Rate (1/{simulationParams.timeUnits})
            </Label>
            <Input
                type="number"
                min="0"
                step="any"
                id="dq-deg-rate"
                bind:value={model.dreadd_deg}
            />
        </div>
        <div class="grid gap-2">
            <Label for="dq-ec50">EC50 ({simulationParams.concentrationUnits})</Label>
            <Input
                type="number"
                min="0"
                step="any"
                id="dq-ec50"
                bind:value={model.dreadd_ec50}
            />
        </div>
        <div class="grid gap-2">
            <Label for="dq-coop">hM3Dq Hill Coefficient</Label>
            <Input
                type="number"
                min="1"
                step="any"
                id="dq-coop"
                bind:value={model.dreadd_cooperativity}
            />
        </div>
    </Dialog.Content>
</Dialog.Root>
