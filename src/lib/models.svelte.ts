import * as backend from "$lib/backend/index.js";
export const isTauriEnv: boolean = backend.isTauriEnv;

/**
 * Generic solution type returned from Rust simulations.
 * Contains time points and corresponding state values.
 */
export interface SimulationSolution<T> {
  t: number[];
  y: T[];
}

/**
 * Summary data returned from Rust simulations.
 */
export interface SummaryData {
  species: string;
  max_concentration: number;
  tmax: number;
}

/**
 * Type alias for simulation results - a tuple of solution and summary data.
 */
export type SimulationResult<T> = [SimulationSolution<T>, SummaryData[]];

export class ConstitutiveModel {
  rma_prod = $state<number>(0.2);
  rma_bbb_transport = $state<number>(0.6);
  rma_deg = $state<number>(0.007);

  /**
   * Serialize this model to a plain object for Tauri/Rust serialization.
   * Rust expects snake_case field names based on the `rename_all = "snake_case"` attribute.
   */
  toJSON(): { prod: number; bbb_transport: number; deg: number } {
    return {
      prod: this.rma_prod,
      bbb_transport: this.rma_bbb_transport,
      deg: this.rma_deg,
    };
  }

  /**
   * Request constitutive simulation
   * @param init_state
   * @param t0
   * @param tf
   * @param dt
   */
  async simulate(
    init_state: ConstitutiveState,
    t0: number,
    tf: number,
    dt: number,
  ): Promise<SimulationResult<{ brain_rma: number; plasma_rma: number }>> {
    return backend.simulateConstitutive(
      this.toJSON(),
      init_state.toJSON(),
      t0,
      tf,
      dt,
    );
  }
}

export class ConstitutiveState {
  brain_rma = $state<number>(0);
  plasma_rma = $state<number>(0);

  reset() {
    this.brain_rma = 0;
    this.plasma_rma = 0;
  }

  /**
   * Serialize this state to a plain object for Tauri/Rust serialization.
   * Rust expects snake_case field names based on the `rename_all = "snake_case"` attribute.
   */
  toJSON(): { brain_rma: number; plasma_rma: number } {
    return {
      brain_rma: this.brain_rma,
      plasma_rma: this.plasma_rma,
    };
  }
}

export class TetoffModel {
  rma_prod = $state<number>(0.2);
  leaky_rma_prod = $state<number>(0.002);
  rma_bbb_transport = $state<number>(0.6);
  rma_deg = $state<number>(0.007);
  tta_prod = $state<number>(10);
  tta_deg = $state<number>(1);
  tta_kd = $state<number>(10);
  tta_cooperativity = $state<number>(2);
  dox_pk_model = $state<DoxModel>(
    new DoxModel([new AccessPeriod(40, [0, 96])]),
  );
  dox_tta_kd = $state<number>(10);

  /**
   * Serialize this state to a plain object for Tauri/Rust serialization.
   * Rust expects snake_case field names based on the `rename_all = "snake_case"` attribute.
   */
  toJSON(): {
    rma_prod: number;
    leaky_rma_prod: number;
    rma_bbb_transport: number;
    rma_deg: number;
    tta_prod: number;
    tta_deg: number;
    tta_kd: number;
    tta_cooperativity: number;
    dox_pk_model: ReturnType<DoxModel["toJSON"]>;
    dox_tta_kd: number;
  } {
    return {
      rma_prod: this.rma_prod,
      leaky_rma_prod: this.leaky_rma_prod,
      rma_bbb_transport: this.rma_bbb_transport,
      rma_deg: this.rma_deg,
      tta_prod: this.tta_prod,
      tta_deg: this.tta_deg,
      tta_kd: this.tta_kd,
      tta_cooperativity: this.tta_cooperativity,
      dox_pk_model: this.dox_pk_model.toJSON(),
      dox_tta_kd: this.dox_tta_kd,
    };
  }

  /**
   * Request TetOff simulation
   * @param initState
   * @param t0
   * @param tf
   * @param dt
   */
  async simulate(
    initState: TetoffState,
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
    return backend.simulateTetoff(
      this.toJSON(),
      initState.toJSON(),
      t0,
      tf,
      dt,
    );
  }
}

export class AccessPeriod {
  dose: number;
  time: [number, number];

  /**
   * Construct a new access period with the given dose and time interval.
   * @param dose
   * @param time
   */
  constructor(dose: number, time: [number, number]) {
    this.dose = $state<number>(dose);
    this.time = $state<[number, number]>(time);
  }

  /**
   * Serialize this state to a plain object for Tauri/Rust serialization.
   * Rust expects snake_case field names based on the `rename_all = "snake_case"` attribute.
   * The time field is serialized as {start, end} to match Rust's RangeInclusive format.
   */
  toJSON(): {
    dose: number;
    time: { start: number; end: number };
  } {
    return {
      dose: this.dose,
      time: {
        start: this.time[0],
        end: this.time[1],
      },
    };
  }
}

export class DoxModel {
  vehicle_intake = $state<number>(1.875e-4);
  bioavailability = $state<number>(0.9);
  absorption = $state<number>(0.8);
  elimination = $state<number>(0.2);
  brain_transport = $state<number>(0.2);
  plasma_transport = $state<number>(1);
  plasma_vd = $state<number>(0.21);
  schedule: AccessPeriod[];

  /**
   * Construct a new DoxModel with the given schedule.
   * @param schedule - Array of access periods for doxycycline dosing
   */
  constructor(schedule: AccessPeriod[] = []) {
    this.schedule = $state<AccessPeriod[]>(schedule);
  }

  dose_concentration = $derived.by(() => {
    let dose_concentrations = this.schedule.map((period) => {
      return (
        ((period.dose * this.bioavailability * this.vehicle_intake) /
          (444.4 * this.plasma_vd)) *
        1e6
      );
    });

    return dose_concentrations;
  });

  /**
   * Serialize this state to a plain object for Tauri/Rust serialization.
   * Rust expects snake_case field names based on the `rename_all = "snake_case"` attribute.
   */
  toJSON(): {
    vehicle_intake: number;
    bioavailability: number;
    absorption: number;
    elimination: number;
    brain_transport: number;
    plasma_transport: number;
    plasma_vd: number;
    schedule: ReturnType<AccessPeriod["toJSON"]>[];
    dose_concentration: number[];
  } {
    return {
      vehicle_intake: this.vehicle_intake,
      bioavailability: this.bioavailability,
      absorption: this.absorption,
      elimination: this.elimination,
      brain_transport: this.brain_transport,
      plasma_transport: this.plasma_transport,
      plasma_vd: this.plasma_vd,
      schedule: this.schedule.map(period => period.toJSON()),
      dose_concentration: this.dose_concentration,
    };
  }
}

export class TetoffState {
  brain_rma = $state<number>(0);
  plasma_rma = $state<number>(0);
  tta: number;
  plasma_dox = $state<number>(0);
  brain_dox = $state<number>(0);

  /**
   * A constructor for oscillating state which takes an initial tTA dose_concentration
   * that may be computed from a tTA production and degradation rate.
   * @param tta
   */
  constructor(tta: number) {
    this.tta = $state<number>(tta);
  }

  reset() {
    this.brain_rma = 0;
    this.plasma_rma = 0;
    this.tta = 0;
    this.plasma_dox = 0;
    this.brain_dox = 0;
  }

  /**
   * Serialize this state to a plain object for Tauri/Rust serialization.
   * Rust expects snake_case field names based on the `rename_all = "snake_case"` attribute.
   */
  toJSON(): {
    brain_rma: number;
    plasma_rma: number;
    tta: number;
    plasma_dox: number;
    brain_dox: number;
  } {
    return {
      brain_rma: this.brain_rma,
      plasma_rma: this.plasma_rma,
      tta: this.tta,
      plasma_dox: this.plasma_dox,
      brain_dox: this.brain_dox,
    };
  }
}

export class ChemogeneticModel {
  rma_prod = $state<number>(0.428);
  leaky_rma_prod = $state<number>(7.01e-3);
  rma_bbb_transport = $state<number>(0.727);
  rma_deg = $state<number>(5.5e-3);
  tta_prod = $state<number>(12.46);
  leaky_tta_prod = $state<number>(1.22e-1);
  tta_deg = $state<number>(2.81e-2);
  tta_kd = $state<number>(4.19);
  tta_cooperativity = $state<number>(2);
  dox_pk_model = $state<DoxModel>(
    new DoxModel([new AccessPeriod(40, [0, 24])]),
  );
  dox_tta_kd = $state<number>(5.27);
  cno_pk_model = $state<CnoModel>(new CnoModel());
  cno_ec50 = $state<number>(7.94);
  clz_ec50 = $state<number>(4.34);
  cno_cooperativity = $state<number>(1);
  clz_cooperativity = $state<number>(1);
  dreadd_prod = $state<number>(8.05);
  dreadd_deg = $state<number>(1);
  dreadd_ec50 = $state<number>(6.79);
  dreadd_cooperativity = $state<number>(1);

  /**
   * Serialize this state to a plain object for Tauri/Rust serialization.
   * Rust expects snake_case field names based on the `rename_all = "snake_case"` attribute.
   */
  toJSON(): {
    rma_prod: number;
    leaky_rma_prod: number;
    rma_bbb_transport: number;
    rma_deg: number;
    tta_prod: number;
    leaky_tta_prod: number;
    tta_deg: number;
    tta_kd: number;
    tta_cooperativity: number;
    dox_pk_model: ReturnType<DoxModel["toJSON"]>;
    dox_tta_kd: number;
    cno_pk_model: ReturnType<CnoModel["toJSON"]>;
    cno_ec50: number;
    clz_ec50: number;
    cno_cooperativity: number;
    clz_cooperativity: number;
    dreadd_prod: number;
    dreadd_deg: number;
    dreadd_ec50: number;
    dreadd_cooperativity: number;
  } {
    return {
      rma_prod: this.rma_prod,
      leaky_rma_prod: this.leaky_rma_prod,
      rma_bbb_transport: this.rma_bbb_transport,
      rma_deg: this.rma_deg,
      tta_prod: this.tta_prod,
      leaky_tta_prod: this.leaky_tta_prod,
      tta_deg: this.tta_deg,
      tta_kd: this.tta_kd,
      tta_cooperativity: this.tta_cooperativity,
      dox_pk_model: this.dox_pk_model.toJSON(),
      dox_tta_kd: this.dox_tta_kd,
      cno_pk_model: this.cno_pk_model.toJSON(),
      cno_ec50: this.cno_ec50,
      clz_ec50: this.clz_ec50,
      cno_cooperativity: this.cno_cooperativity,
      clz_cooperativity: this.clz_cooperativity,
      dreadd_prod: this.dreadd_prod,
      dreadd_deg: this.dreadd_deg,
      dreadd_ec50: this.dreadd_ec50,
      dreadd_cooperativity: this.dreadd_cooperativity,
    };
  }

  /**
   * Request Chemogenetic simulation
   * @param initState
   * @param t0
   * @param tf
   * @param dt
   */
  async simulate(
    initState: TetoffState,
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
    return backend.simulateChemogenetic(
      this.toJSON(),
      initState.toJSON(),
      t0,
      tf,
      dt,
    );
  }
}

export class ChemogeneticState {
  brain_rma = $state<number>(0);
  plasma_rma = $state<number>(0);
  tta = $state<number>(0);
  plasma_dox = $state<number>(0);
  brain_dox = $state<number>(0);
  dreadd: number;
  peritoneal_cno = $state<number>(0);
  plasma_cno = $state<number>(0);
  brain_cno = $state<number>(0);
  plasma_clz = $state<number>(0);
  brain_clz = $state<number>(0);

  /**
   * A constructor for chemogenetic state which takes an initial DREADD
   * concentration that may be computed from a tTA production and degradation
   * rate.
   * @param dreadd
   */
  constructor(dreadd: number) {
    this.dreadd = $state<number>(dreadd);
  }

  reset() {
    this.brain_rma = 0;
    this.plasma_rma = 0;
    this.tta = 0;
    this.plasma_dox = 0;
    this.brain_dox = 0;
    this.dreadd = 0;
    this.peritoneal_cno = 0;
    this.plasma_cno = 0;
    this.brain_cno = 0;
    this.plasma_clz = 0;
    this.brain_clz = 0;
  }

  /**
   * Serialize this state to a plain object for Tauri/Rust serialization.
   * Rust expects snake_case field names based on the `rename_all = "snake_case"` attribute.
   */
  toJSON(): {
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
  } {
    return {
      brain_rma: this.brain_rma,
      plasma_rma: this.plasma_rma,
      tta: this.tta,
      plasma_dox: this.plasma_dox,
      brain_dox: this.brain_dox,
      dreadd: this.dreadd,
      peritoneal_cno: this.peritoneal_cno,
      plasma_cno: this.plasma_cno,
      brain_cno: this.brain_cno,
      plasma_clz: this.plasma_clz,
      brain_clz: this.brain_clz,
    };
  }
}

export class CnoModel {
  doses = $state<CnoDose[]>([new CnoDose(0.03, 48)]);
  cno_absorption = $state<number>(23.94);
  cno_elimination = $state<number>(5.51e-2);
  cno_reverse_metabolism = $state<number>(1.44);
  clz_metabolism = $state<number>(3e-1);
  clz_elimination = $state<number>(3.94);
  cno_brain_transport = $state<number>(2.33);
  cno_plasma_transport = $state<number>(71.85);
  clz_brain_transport = $state<number>(35.61);
  clz_plasma_transport = $state<number>(34.07);
  cno_plasma_vd = $state<number>(3.99e-2);
  cno_brain_vd = $state<number>(0.21);
  clz_plasma_vd = $state<number>(0.24);
  clz_brain_vd = $state<number>(8.87e-2);

  /**
   * Serialize this state to a plain object for Tauri/Rust serialization.
   * Rust expects snake_case field names based on the `rename_all = "snake_case"` attribute.
   */
  toJSON(): {
    doses: ReturnType<CnoDose["toJSON"]>[];
    cno_absorption: number;
    cno_elimination: number;
    cno_reverse_metabolism: number;
    clz_metabolism: number;
    clz_elimination: number;
    cno_brain_transport: number;
    cno_plasma_transport: number;
    clz_brain_transport: number;
    clz_plasma_transport: number;
    cno_plasma_vd: number;
    cno_brain_vd: number;
    clz_plasma_vd: number;
    clz_brain_vd: number;
  } {
    return {
      doses: this.doses.map(dose => dose.toJSON()),
      cno_absorption: this.cno_absorption,
      cno_elimination: this.cno_elimination,
      cno_reverse_metabolism: this.cno_reverse_metabolism,
      clz_metabolism: this.clz_metabolism,
      clz_elimination: this.clz_elimination,
      cno_brain_transport: this.cno_brain_transport,
      cno_plasma_transport: this.cno_plasma_transport,
      clz_brain_transport: this.clz_brain_transport,
      clz_plasma_transport: this.clz_plasma_transport,
      cno_plasma_vd: this.cno_plasma_vd,
      cno_brain_vd: this.cno_brain_vd,
      clz_plasma_vd: this.clz_plasma_vd,
      clz_brain_vd: this.clz_brain_vd,
    };
  }
}

const CNO_MW = 342.8; // g/mol

export class CnoDose {
  mg: number;
  nmol: number;
  time: number;

  /**
   * Construct a new CNO dose with the given amount in mg and time.
   * @param mg
   * @param time
   */
  constructor(mg: number, time: number) {
    this.mg = $state<number>(mg);
    this.time = $state<number>(time);
    this.nmol = $derived((this.mg / CNO_MW) * 1e6);
  }

  /**
   * Serialize this state to a plain object for Tauri/Rust serialization.
   * Rust expects snake_case field names based on the `rename_all = "snake_case"` attribute.
   */
  toJSON(): {
    mg: number;
    nmol: number;
    time: number;
  } {
    return {
      mg: this.mg,
      nmol: this.nmol,
      time: this.time,
    };
  }
}

export class OscillatingModel {
  rma_prod = $state<number>(0.2);
  rma_bbb_transport = $state<number>(0.6);
  rma_deg = $state<number>(0.007);
  freq = $state<number>(0.0138);

  /**
   * Serialize this model to a plain object for Tauri/Rust serialization.
   * Rust expects snake_case field names based on the `rename_all = "snake_case"` attribute.
   */
  toJSON(): { prod: number; bbb_transport: number; deg: number; freq: number } {
    return {
      prod: this.rma_prod,
      bbb_transport: this.rma_bbb_transport,
      deg: this.rma_deg,
      freq: this.freq,
    };
  }

  /**
   * Request oscillating simulation
   * @param init_state
   * @param t0
   * @param tf
   * @param dt
   */
  async simulate(
    init_state: ConstitutiveState,
    t0: number,
    tf: number,
    dt: number,
  ): Promise<SimulationResult<{ brain_rma: number; plasma_rma: number }>> {
    return backend.simulateOscillating(
      this.toJSON(),
      init_state.toJSON(),
      t0,
      tf,
      dt,
    );
  }
}

export class OscillatingState {
  brain_rma = $state<number>(0);
  plasma_rma = $state<number>(0);

  reset() {
    this.brain_rma = 0;
    this.plasma_rma = 0;
  }

  /**
   * Serialize this state to a plain object for Tauri/Rust serialization.
   * Rust expects snake_case field names based on the `rename_all = "snake_case"` attribute.
   */
  toJSON(): { brain_rma: number; plasma_rma: number } {
    return {
      brain_rma: this.brain_rma,
      plasma_rma: this.plasma_rma,
    };
  }
}
