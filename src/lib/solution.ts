import { invoke } from "@tauri-apps/api/core";
import { isTauriEnv } from "./models.svelte";
import * as backend from "$lib/backend/index.js";

/**
 * Generic solution state - all possible fields from any model.
 * Fields not present for a given model will be undefined.
 */
interface SolutionState {
  brain_rma: number;
  plasma_rma: number;
  tta?: number;
  plasma_dox?: number;
  brain_dox?: number;
  dreadd?: number;
  peritoneal_cno?: number;
  plasma_cno?: number;
  brain_cno?: number;
  plasma_clz?: number;
  brain_clz?: number;
}

interface Solution {
  t: number[];
  y: SolutionState[];
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

/**
 * Gets the tTA concentration time series.
 * @param solution
 * @returns
 */
export function getTta(solution: Solution): number[] {
  return solution.y.map((state) => state.tta ?? 0);
}

/**
 * Gets the brain doxycycline concentration time series.
 * @param solution
 * @returns
 */
export function getDox(solution: Solution): number[] {
  return solution.y.map((state) => state.brain_dox ?? 0);
}

/**
 * Gets the DREADD (hM3Dq) concentration time series.
 * @param solution
 * @returns
 */
export function getDreadd(solution: Solution): number[] {
  return solution.y.map((state) => state.dreadd ?? 0);
}

/**
 * Gets the brain CNO concentration time series.
 * @param solution
 * @returns
 */
export function getCno(solution: Solution): number[] {
  return solution.y.map((state) => state.brain_cno ?? 0);
}

/**
 * Gets the brain CLZ concentration time series.
 * @param solution
 * @returns
 */
export function getClz(solution: Solution): number[] {
  return solution.y.map((state) => state.brain_clz ?? 0);
}

export function getSpecies(solution: Solution, speciesType: string): number[] {
  switch (speciesType) {
    case "plasmaRMA":
      return getPlasmaRma(solution);
    case "brainRMA":
      return getBrainRma(solution);
    case "tTA":
      return getTta(solution);
    case "dox":
      return getDox(solution);
    case "hM3Dq":
      return getDreadd(solution);
    case "CNO":
      return getCno(solution);
    case "CLZ":
      return getClz(solution);
    default:
      throw new Error(`Unknown species type: ${speciesType}`);
  }
}

export interface SummaryStat {
  speciesType: SpeciesType;
  max_concentration: number;
  tmax: number;
}

export const exportSimulation = async (
  solution: Solution,
  modelType: string,
  format: "csv" | "parquet" | "json" = "csv",
) => {
  if (format === "csv") {
    await backend.csvExport(solution, modelType, format);
  }
};

export const saveImage = async (element: HTMLElement, speciesName: string) => {
  const svgElement = element.querySelector('svg');
  if (!svgElement) {
    throw new Error('No SVG element found in plot area');
  }
  const serializer = new XMLSerializer();
  const svgString = serializer.serializeToString(svgElement);
  await backend.saveImage(svgString, speciesName);
};
