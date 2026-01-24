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
        <Button variant="outline" onclick={() => initState.reset()}
            >Reset</Button
        >
    </Dialog.Content>
</Dialog.Root>
