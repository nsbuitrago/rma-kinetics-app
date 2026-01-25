<script lang="ts">
    import * as Card from "$lib/components/ui/card/index.js";

    import { ChemogeneticModel, ChemogeneticState } from "$lib/models.svelte";

    import SimulationParams from "$lib/components/simulation-params.svelte";
    import RmaRates from "$lib/components/rma-rates.svelte";
    import SubmitButton from "$lib/components/submit-button.svelte";
    import InitState from "$lib/components/init-state.svelte";
    import TtaParams from "$lib/components/tta-params.svelte";
    import DoxDose from "$lib/components/dox-dose.svelte";
    import DreaddParams from "$lib/components/dreadd-params.svelte";
    import CnoDose from "$lib/components/cno-dose.svelte";

    let { solution = $bindable(), summary = $bindable(), errorMessage = $bindable(), runSimulation = $bindable() } = $props();

    // simulation config
    let simulationParams = $state({
        timeUnits: "hr" as "hr" | "min" | "s",
        concentrationUnits: "nM" as "nM" | "µM",
        t0: 0,
        tf: 96,
        dt: 1,
    });

    // model
    let model = $state<ChemogeneticModel>(new ChemogeneticModel());
    let initDreadd = $derived(model.dreadd_prod / model.dreadd_deg);
    let initState = $derived(new ChemogeneticState(initDreadd));

    // dialog states
    let initCondDialogOpen = $state<boolean>(false);
    let ttaDialogOpen = $state<boolean>(false);
    let dqDialogOpen = $state<boolean>(false);
    let doxDialogOpen = $state<boolean>(false);
    let cnoDialogOpen = $state<boolean>(false);

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
        <Card.Title>Chemogenetic RMA</Card.Title>
        <Card.Description class="grid gap-2">
            <p>
                RMA expression induced by neuronal activity using the activatory
                designed receptor hM3Dq and optional doxycycline inhibition with
                Tet-Off gating.
            </p>
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

            <DreaddParams
                bind:model
                bind:simulationParams
                bind:dialogOpen={dqDialogOpen}
            />

            <div class="flex justify-evenly">
                <DoxDose
                    bind:model
                    bind:simulationParams
                    bind:dialogOpen={doxDialogOpen}
                />

                <CnoDose
                    bind:model
                    bind:simulationParams
                    bind:dialogOpen={cnoDialogOpen}
                />
            </div>

            <InitState
                bind:initState
                bind:simulationParams
                bind:dialogOpen={initCondDialogOpen}
            />

            <SubmitButton />
        </form>
    </Card.Content>
</Card.Root>
