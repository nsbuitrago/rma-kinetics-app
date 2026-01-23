import { invoke } from "@tauri-apps/api/core";
import { ConstitutiveState, isTauriEnv } from "./models.svelte";

interface Solution {
  t: number[];
  y: ConstitutiveState[];
}

enum SpeciesType {
  plasmaRMA = "plasmaRMA",
  brainRMA = "brainRMA",
  tTA = "tTA",
  dox = "dox",
  hM3Dq = "hM3Dq",
  CNO = "CNO",
  CLZ = "CLZ",
}

export function getPlasmaRma(solution: Solution): number[] {
  return solution.y.map((state) => state.plasma_rma);
}

export function getBrainRma(solution: Solution): number[] {
  return solution.y.map((state) => state.brain_rma);
}

export function getSpecies(solution: Solution, speciesType: string): number[] {
  switch (speciesType) {
    case "plasmaRMA":
      return getPlasmaRma(solution);
    case "brainRMA":
      return getBrainRma(solution);
    default:
      throw new Error(`Unknown species type: ${speciesType}`);
  }
}

export interface SummaryStat {
  speciesType: SpeciesType;
  max_concentration: number;
  tmax: number;
}

export async function get_summary_stats(
  solution: Solution,
  speciesType: SpeciesType,
): Promise<SummaryStat[]> {
  let summary_stats: SummaryStat[];
  if (isTauriEnv) {
    summary_stats = await invoke("get_summary_stat", {
      solution,
      speciesType,
    });
  } else {
    // TODO: wasm implementation
    summary_stats = [];
  }

  return summary_stats;
}
