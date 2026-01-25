<script lang="ts">
    import * as Card from "$lib/components/ui/card/index.js";

    import { TetoffModel, TetoffState } from "$lib/models.svelte";

    import SimulationParams from "$lib/components/simulation-params.svelte";
    import RmaRates from "$lib/components/rma-rates.svelte";
    import SubmitButton from "$lib/components/submit-button.svelte";
    import InitState from "$lib/components/init-state.svelte";
    import TtaParams from "$lib/components/tta-params.svelte";
    import DoxDose from "$lib/components/dox-dose.svelte";

    let { solution = $bindable(), summary = $bindable(), errorMessage = $bindable(), runSimulation = $bindable() } = $props();

    // simulation config
    let simulationParams = $state({
        timeUnits: "hr" as "hr" | "min" | "s",
        concentrationUnits: "nM" as "nM" | "µM",
        t0: 0,
        tf: 504,
        dt: 1,
    });

    // model
    let model = $state<TetoffModel>(new TetoffModel());
    let initTta = $derived(model.tta_prod / model.tta_deg);
    let initState = $derived(new TetoffState(initTta));

    let initCondDialogOpen = $state<boolean>(false);
    let ttaDialogOpen = $state<boolean>(false);
    let doxDialogOpen = $state<boolean>(false);

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
        <Card.Title>TetOff RMA</Card.Title>
        <Card.Description class="grid gap-2">
            <p>Inducible RMA expression using the TetOff system.</p>
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

            <TtaParams
                bind:model
                bind:simulationParams
                bind:dialogOpen={ttaDialogOpen}
            />

            <DoxDose
                bind:model
                bind:simulationParams
                bind:dialogOpen={doxDialogOpen}
            />

            <InitState
                bind:initState
                bind:simulationParams
                bind:dialogOpen={initCondDialogOpen}
            />

            <SubmitButton />
        </form>
    </Card.Content>
</Card.Root>
