<script lang="ts">
    import { Label } from "$lib/components/ui/label/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import * as Select from "$lib/components/ui/select/index.js";

    let { params = $bindable() } = $props();
</script>

<h1 class="font-bold">Simulation Parameters</h1>
<div class="flex justify-between">
    <div class="flex gap-2">
        <Label for="time-units">Time Scale</Label>
        <Select.Root
            type="single"
            bind:value={params.timeUnits}
            name="time-units"
        >
            <Select.Trigger>{params.timeUnits}</Select.Trigger>
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
            bind:value={params.concentrationUnits}
            name="concentrationUnits"
        >
            <Select.Trigger>{params.concentrationUnits}</Select.Trigger>
            <Select.Content>
                <Select.Item value="nM" label="nM" />
                <Select.Item value="uM" label="µM" />
            </Select.Content>
        </Select.Root>
    </div>
</div>
<div class="flex flex-row gap-2">
    <div class="grid gap-2">
        <Label for="t0">Start Time ({params.timeUnits})</Label>
        <Input
            type="number"
            min="0"
            step="any"
            id="t0"
            bind:value={params.t0}
        />
    </div>
    <div class="grid gap-2">
        <Label for="tf">Stop Time ({params.timeUnits})</Label>
        <Input
            type="number"
            min="0"
            step="any"
            id="tf"
            bind:value={params.tf}
        />
    </div>
    <div class="grid gap-2">
        <Label for="dt">Step size ({params.timeUnits})</Label>
        <Input type="number" min="1" step="1" id="dt" bind:value={params.dt} />
    </div>
</div>
