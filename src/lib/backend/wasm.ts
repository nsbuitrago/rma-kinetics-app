import type { SimulationResult } from "$lib/models.svelte.js";

// Dynamic import type for the WASM module
type WasmModule = typeof import("./wasm-pkg/rma_kinetics_wasm.js");

let wasmModule: WasmModule | null = null;
let initPromise: Promise<void> | null = null;

async function ensureInit(): Promise<WasmModule> {
  if (wasmModule) {
    return wasmModule;
  }

  if (!initPromise) {
    initPromise = (async () => {
      const module = await import("./wasm-pkg/rma_kinetics_wasm.js");
      await module.default();
      wasmModule = module;
    })();
  }

  await initPromise;
  return wasmModule!;
}

export async function simulateConstitutive(
  model: unknown,
  initState: unknown,
  t0: number,
  tf: number,
  dt: number,
): Promise<SimulationResult<{ brain_rma: number; plasma_rma: number }>> {
  const wasm = await ensureInit();
  try {
    console.log("WASM simulateConstitutive called with:", {
      model,
      initState,
      t0,
      tf,
      dt,
    });
    const result = wasm.simulate_constitutive_model(
      model,
      initState,
      t0,
      tf,
      dt,
    );
    console.log("WASM result:", result);
    return result;
  } catch (error) {
    console.error("WASM simulation error:", error);
    throw error;
  }
}

export async function simulateOscillating(
  model: unknown,
  initState: unknown,
  t0: number,
  tf: number,
  dt: number,
  noiseLevel: number,
): Promise<SimulationResult<{ brain_rma: number; plasma_rma: number }>> {
  const wasm = await ensureInit();
  return wasm.simulate_oscillating_model(
    model,
    initState,
    t0,
    tf,
    dt,
    noiseLevel,
  );
}

export async function simulateTetoff(
  model: unknown,
  initState: unknown,
  t0: number,
  tf: number,
  dt: number,
): Promise<
  SimulationResult<{
    brain_rma: number;
    plasma_rma: number;
    tta: number;
    plasma_dox: number;
    brain_dox: number;
  }>
> {
  const wasm = await ensureInit();
  return wasm.simulate_tetoff_model(model, initState, t0, tf, dt);
}

export async function simulateChemogenetic(
  model: unknown,
  initState: unknown,
  t0: number,
  tf: number,
  dt: number,
): Promise<
  SimulationResult<{
    brain_rma: number;
    plasma_rma: number;
    tta: number;
    plasma_dox: number;
    brain_dox: number;
    dreadd: number;
    peritoneal_cno: number;
    plasma_cno: number;
    brain_cno: number;
    plasma_clz: number;
    brain_clz: number;
  }>
> {
  const wasm = await ensureInit();
  return wasm.simulate_chemogenetic_model(model, initState, t0, tf, dt);
}
