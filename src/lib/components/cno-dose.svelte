<script lang="ts">
    import * as Dialog from "$lib/components/ui/dialog/index.js";
    import * as Table from "$lib/components/ui/table/index.js";
    import { Button, buttonVariants } from "$lib/components/ui/button/index.js";
    import { Label } from "$lib/components/ui/label/index.js";
    import { Input } from "$lib/components/ui/input/index.js";
    import { Trash } from "@lucide/svelte";
    import { CnoDose } from "$lib/models.svelte";

    let {
        model = $bindable(),
        simulationParams = $bindable(),
        dialogOpen = $bindable(),
    } = $props();

    let cnoDose = $state<number>(0.03);
    let cnoT0 = $state<number>(96);
    let cnoView = $state<"main" | "pk">("main");

    function pushCnoDose(dose: CnoDose) {
        model.cno_pk_model.doses.push(dose);
    }

    function deleteCnoDose(index: number) {
        model.cno_pk_model.doses.splice(index, 1);
    }
</script>

<Dialog.Root bind:open={dialogOpen} onOpenChange={() => (cnoView = "main")}>
    <Dialog.Trigger class={buttonVariants({ variant: "outline" })}>
        CNO Dose
    </Dialog.Trigger>
    <Dialog.Content class="overflow-hidden p-0">
        <div
            class="flex transition-transform duration-300 ease-in-out"
            style="transform: translateX({cnoView === 'main' ? '0%' : '-100%'})"
        >
            <!-- Main View -->
            <div
                class="w-full shrink-0 flex flex-col gap-3 p-6 overflow-hidden"
            >
                <div class="grid grid-cols-2 gap-4">
                    <div class="grid gap-2">
                        <Label for="cno-dose">Dose (mg)</Label>
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="cno-dose"
                            bind:value={cnoDose}
                        />
                    </div>
                    <div class="grid gap-2">
                        <Label for="cno-t0">
                            Injection Time ({simulationParams.timeUnits})
                        </Label>
                        <Input
                            type="number"
                            min="0"
                            step="any"
                            id="cno-t0"
                            bind:value={cnoT0}
                        />
                    </div>
                </div>
                <Button
                    onclick={() => pushCnoDose(new CnoDose(cnoDose, cnoT0))}
                >
                    Add new CNO dose
                </Button>
                <Button
                    variant="outline"
                    onclick={() => (cnoView = "pk")}
                    class="hover:cursor-pointer"
                >
                    See More Parameters
                </Button>
                <div class="max-h-48 overflow-y-auto">
                    <Table.Root>
                        <Table.Header>
                            <Table.Row>
                                <Table.Head class="font-bold"
                                    >Dose (mg)</Table.Head
                                >
                                <Table.Head class="font-bold"
                                    >Injection Time</Table.Head
                                >
                            </Table.Row>
                        </Table.Header>
                        <Table.Body class="scroll-auto">
                            {#each model.cno_pk_model.doses as dose, index}
                                <Table.Row>
                                    <Table.Cell>{dose.mg}</Table.Cell>
                                    <Table.Cell>{dose.time}</Table.Cell>
                                    <Table.Cell>
                                        <Button
                                            variant="ghost"
                                            class="text-destructive"
                                            onclick={() => deleteCnoDose(index)}
                                        >
                                            <Trash />
                                        </Button>
                                    </Table.Cell>
                                </Table.Row>
                            {/each}
                        </Table.Body>
                    </Table.Root>
                </div>
            </div>
            <!-- PK Parameters View -->
            <div
                class="w-full shrink-0 flex flex-col gap-3 p-6 overflow-hidden"
            >
                <h3 class="font-semibold">PK Parameters</h3>
                <div class="max-h-96 overflow-y-scroll flex flex-col gap-3">
                    <div class="grid grid-cols-2 gap-4">
                        <div class="grid gap-2">
                            <Label for="cno-abs">
                                CNO Absorption Rate (1/{simulationParams.timeUnits})
                            </Label>
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="cno-abs"
                                bind:value={model.cno_pk_model.cno_absorption}
                            />
                        </div>
                        <div class="grid gap-2">
                            <Label for="cno-el">
                                CNO Elimination Rate (1/{simulationParams.timeUnits})
                            </Label>
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="cno-el"
                                bind:value={model.cno_pk_model.cno_elimination}
                            />
                        </div>
                    </div>
                    <div class="grid grid-cols-2 gap-4">
                        <div class="grid gap-2">
                            <Label for="clz-el">
                                CLZ Elimination Rate (1/{simulationParams.timeUnits})
                            </Label>
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="clz-el"
                                bind:value={model.cno_pk_model.clz_elimination}
                            />
                        </div>
                    </div>
                    <div class="grid grid-cols-2 gap-4">
                        <div class="grid gap-2">
                            <Label for="cno-rev-met">
                                CNO Reverse Metabolism Rate (1/{simulationParams.timeUnits})
                            </Label>
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="cno-rev-met"
                                bind:value={
                                    model.cno_pk_model.cno_reverse_metabolism
                                }
                            />
                        </div>
                        <div class="grid gap-2">
                            <Label for="clz-met">
                                CLZ Metabolism Rate (1/{simulationParams.timeUnits})
                            </Label>
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="clz-met"
                                bind:value={model.cno_pk_model.clz_metabolism}
                            />
                        </div>
                    </div>
                    <div class="grid grid-cols-2 gap-4">
                        <div class="grid gap-2">
                            <Label for="cno-brain-trans">
                                CNO Brain Transport Rate (1/{simulationParams.timeUnits})
                            </Label>
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="cno-brain-trans"
                                bind:value={
                                    model.cno_pk_model.cno_brain_transport
                                }
                            />
                        </div>
                        <div class="grid gap-2">
                            <Label for="cno-plasma-trans">
                                CNO Plasma Transport Rate (1/{simulationParams.timeUnits})
                            </Label>
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="cno-plasma-trans"
                                bind:value={
                                    model.cno_pk_model.cno_plasma_transport
                                }
                            />
                        </div>
                    </div>
                    <div class="grid grid-cols-2 gap-4">
                        <div class="grid gap-2">
                            <Label for="clz-brain-trans">
                                CLZ Brain Transport Rate (1/{simulationParams.timeUnits})
                            </Label>
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="clz-brain-trans"
                                bind:value={
                                    model.cno_pk_model.clz_brain_transport
                                }
                            />
                        </div>
                        <div class="grid gap-2">
                            <Label for="clz-plasma-trans">
                                CLZ Plasma Transport Rate (1/{simulationParams.timeUnits})
                            </Label>
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="clz-plasma-trans"
                                bind:value={
                                    model.cno_pk_model.clz_plasma_transport
                                }
                            />
                        </div>
                    </div>
                    <div class="grid grid-cols-2 gap-4">
                        <div class="grid gap-2">
                            <Label for="cno-plasma-vd">CNO Plasma Vd (L)</Label>
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="cno-plasma-vd"
                                bind:value={model.cno_pk_model.cno_plasma_vd}
                            />
                        </div>
                        <div class="grid gap-2">
                            <Label for="cno-brain-vd">CNO Brain Vd (L)</Label>
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="cno-brain-vd"
                                bind:value={model.cno_pk_model.cno_brain_vd}
                            />
                        </div>
                    </div>
                    <div class="grid grid-cols-2 gap-4">
                        <div class="grid gap-2">
                            <Label for="clz-plasma-vd">CLZ Plasma Vd (L)</Label>
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="clz-plasma-vd"
                                bind:value={model.cno_pk_model.clz_plasma_vd}
                            />
                        </div>
                        <div class="grid gap-2">
                            <Label for="clz-brain-vd">CLZ Brain Vd (L)</Label>
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="clz-brain-vd"
                                bind:value={model.cno_pk_model.clz_brain_vd}
                            />
                        </div>
                    </div>
                    <div class="grid grid-cols-2 gap-4">
                        <div class="grid gap-2">
                            <Label for="cno-ec50">
                                CNO EC50 ({simulationParams.concentrationUnits})
                            </Label>
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="cno-ec50"
                                bind:value={model.cno_ec50}
                            />
                        </div>
                        <div class="grid gap-2">
                            <Label for="clz-ec50">
                                CLZ EC50 ({simulationParams.concentrationUnits})
                            </Label>
                            <Input
                                type="number"
                                min="0"
                                step="any"
                                id="clz-ec50"
                                bind:value={model.clz_ec50}
                            />
                        </div>
                    </div>
                    <div class="grid grid-cols-2 gap-4">
                        <div class="grid gap-2">
                            <Label for="cno-coop"
                                >CNO-DREADD Hill Coefficient</Label
                            >
                            <Input
                                type="number"
                                min="1"
                                step="any"
                                id="cno-coop"
                                bind:value={model.cno_cooperativity}
                            />
                        </div>
                        <div class="grid gap-2">
                            <Label for="clz-coop"
                                >CLZ-DREADD Hill Coefficient</Label
                            >
                            <Input
                                type="number"
                                min="1"
                                step="any"
                                id="clz-coop"
                                bind:value={model.clz_cooperativity}
                            />
                        </div>
                    </div>
                </div>
                <div class="flex justify-between">
                    <Button
                        variant="outline"
                        onclick={() => (cnoView = "main")}
                        class="hover:cursor-pointer w-fit"
                    >
                        ← Back
                    </Button>
                </div>
            </div>
        </div>
    </Dialog.Content>
</Dialog.Root>
