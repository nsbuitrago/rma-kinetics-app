import { invoke } from "@tauri-apps/api/core";
import type { SimulationResult } from "$lib/models.svelte.js";

export async function simulateConstitutive(
  model: unknown,
  initState: unknown,
  t0: number,
  tf: number,
  dt: number,
): Promise<SimulationResult<{ brain_rma: number; plasma_rma: number }>> {
  return invoke("simulate_constitutive_model", {
    model,
    init_state: initState,
    t0,
    tf,
    dt,
  });
}

export async function simulateOscillating(
  model: unknown,
  initState: unknown,
  t0: number,
  tf: number,
  dt: number,
  noiseLevel: number,
): Promise<SimulationResult<{ brain_rma: number; plasma_rma: number }>> {
  return invoke("simulate_oscillating_model", {
    model,
    init_state: initState,
    t0,
    tf,
    dt,
    noise_level: noiseLevel,
  });
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
  return invoke("simulate_tetoff_model", {
    model,
    init_state: initState,
    t0,
    tf,
    dt,
  });
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
  return invoke("simulate_chemogenetic_model", {
    model,
    init_state: initState,
    t0,
    tf,
    dt,
  });
}
