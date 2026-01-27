import { isTauri } from "@tauri-apps/api/core";

export const isTauriEnv = isTauri();

// Lazy-loaded backend implementations
let backendModule:
  | typeof import("./tauri.js")
  | typeof import("./wasm.js")
  | null = null;

async function getBackend() {
  if (backendModule) {
    return backendModule;
  }

  if (isTauriEnv) {
    backendModule = await import("./tauri.js");
  } else {
    backendModule = await import("./wasm.js");
  }

  return backendModule;
}

export async function simulateConstitutive(
  model: unknown,
  initState: unknown,
  t0: number,
  tf: number,
  dt: number,
) {
  const backend = await getBackend();
  return backend.simulateConstitutive(model, initState, t0, tf, dt);
}

export async function simulateOscillating(
  model: unknown,
  initState: unknown,
  t0: number,
  tf: number,
  dt: number,
  noiseLevel: number,
) {
  const backend = await getBackend();
  return backend.simulateOscillating(model, initState, t0, tf, dt, noiseLevel);
}

export async function simulateTetoff(
  model: unknown,
  initState: unknown,
  t0: number,
  tf: number,
  dt: number,
) {
  const backend = await getBackend();
  return backend.simulateTetoff(model, initState, t0, tf, dt);
}

export async function simulateChemogenetic(
  model: unknown,
  initState: unknown,
  t0: number,
  tf: number,
  dt: number,
) {
  const backend = await getBackend();
  return backend.simulateChemogenetic(model, initState, t0, tf, dt);
}
