<script lang="ts">
    import { Label } from "$lib/components/ui/label/index.js";
    import { Input } from "$lib/components/ui/input/index.js";

    let {
        concentrationUnits = $bindable(),
        timeUnits = $bindable(),
        model = $bindable(),
    } = $props();
</script>

<h1 class="font-bold">RMA Rates</h1>

<div class="grid gap-2">
    <Label for="rma-prod"
        >Production Rate ({concentrationUnits}/{timeUnits})</Label
    >
    <Input
        type="number"
        min="0"
        step="any"
        id="rma-prod"
        bind:value={model.rma_prod}
    />
</div>

{#if model.leaky_rma_prod}
    <div class="grid gap-2">
        <Label for="leaky-rma-prod"
            >Leaky Production Rate ({concentrationUnits}/{timeUnits})</Label
        >
        <Input
            type="number"
            min="0"
            step="any"
            id="leakyrma-prod"
            bind:value={model.leaky_rma_prod}
        />
    </div>
{/if}

<div class="grid gap-2">
    <Label for="rma-bbb-transport"
        >Reverse Transcytosis Rate (1/{timeUnits})</Label
    >
    <Input
        type="number"
        min="0"
        step="any"
        id="rma-bbb-transport"
        bind:value={model.rma_bbb_transport}
    />
</div>

<div class="grid gap-2">
    <Label for="rma-deg">Degradation Rate (1/{timeUnits})</Label>
    <Input
        type="number"
        min="0"
        step="any"
        id="rma-deg"
        bind:value={model.rma_deg}
    />
</div>

{#if model.freq}
    <div class="grid gap-2">
        <Label for="freq">Freq (1/{timeUnits})</Label>
        <Input
            type="number"
            min="0"
            step="any"
            id="freq"
            bind:value={model.freq}
        />
    </div>
{/if}
