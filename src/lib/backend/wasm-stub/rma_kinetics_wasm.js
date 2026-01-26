// Stub module for Tauri builds
// This file satisfies the bundler during Tauri builds when the real WASM package
// is not available. The code never executes at runtime because isTauriEnv check
// in index.ts routes to the Tauri backend instead.

export default function init() {
  throw new Error("WASM not available - use Tauri backend");
}

export function simulate_constitutive_model() {
  throw new Error("WASM not available - use Tauri backend");
}

export function simulate_oscillating_model() {
  throw new Error("WASM not available - use Tauri backend");
}

export function simulate_tetoff_model() {
  throw new Error("WASM not available - use Tauri backend");
}

export function simulate_chemogenetic_model() {
  throw new Error("WASM not available - use Tauri backend");
}
