<script lang="ts">
    import * as Card from "$lib/components/ui/card/index.js";
    import SimulationParams from "$lib/components/simulation-params.svelte";
    import RmaRates from "$lib/components/rma-rates.svelte";
    import InitState from "$lib/components/init-state.svelte";
    import SubmitButton from "$lib/components/submit-button.svelte";
    import { OscillatingModel, OscillatingState } from "$lib/models.svelte";

    let { solution = $bindable(), summary = $bindable(), errorMessage = $bindable(), runSimulation = $bindable() } = $props();

    // simulation config
    let simulationParams = $state({
        timeUnits: "hr",
        concentrationUnits: "nM",
        t0: 0,
        tf: 504,
        dt: 1,
    });

    // model
    let model = $state<OscillatingModel>(new OscillatingModel());
    let initState = $state<OscillatingState>(new OscillatingState());
    let initCondDialogOpen = $state<boolean>(false);

    async function run_simulation() {
        try {
            [solution, summary] = await model.simulate(
                initState,
                simulationParams.t0,
                simulationParams.tf,
                simulationParams.dt,
            );
            errorMessage = null;
        } catch (error) {
            errorMessage = error instanceof Error ? error.message : String(error);
        }
    }

    // Expose the run_simulation function to parent
    runSimulation = run_simulation;
</script>

<Card.Root>
    <Card.Header>
        <Card.Title>Oscillating RMA</Card.Title>
        <Card.Description class="grid gap-2">
            <p>RMA expression driven by an oscillating force function.</p>
        </Card.Description>
    </Card.Header>
    <Card.Content>
        <form class="flex flex-col gap-3" onsubmit={run_simulation}>
            <SimulationParams bind:params={simulationParams} />
            <RmaRates
                bind:concentrationUnits={simulationParams.concentrationUnits}
                bind:timeUnits={simulationParams.timeUnits}
                bind:model
            />
            <InitState
                bind:simulationParams
                bind:initState
                bind:dialogOpen={initCondDialogOpen}
            />
            <SubmitButton />
        </form>
    </Card.Content>
</Card.Root>
