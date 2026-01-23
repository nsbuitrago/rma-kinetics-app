import { invoke, isTauri } from "@tauri-apps/api/core";
export const isTauriEnv: boolean = isTauri();

export class ConstitutiveModel {
  prod = $state<number>(0.2);
  bbbTransport = $state<number>(0.6);
  deg = $state<number>(0.007);

  /**
   * Serialize this model to a plain object for Tauri/Rust serialization.
   * Rust expects snake_case field names based on the `rename_all = "snake_case"` attribute.
   */
  toJSON(): { prod: number; bbb_transport: number; deg: number } {
    return {
      prod: this.prod,
      bbb_transport: this.bbbTransport,
      deg: this.deg,
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
  ) {
    if (isTauriEnv) {
      let solution = await invoke("simulate_constitutive_model", {
        model: this.toJSON(),
        init_state: init_state.toJSON(),
        t0,
        tf,
        dt,
      });

      return solution;
    } else {
      console.log("running in the browser. Use wasm");
    }
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
  dox_pk_model = $state<DoxModel>(new DoxModel());
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
    dox_pk_model: DoxModel;
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
      dox_pk_model: this.dox_pk_model,
      dox_tta_kd: this.dox_tta_kd,
    };
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
   */
  toJSON(): {
    dose: number;
    time: [number, number];
  } {
    return {
      dose: this.dose,
      time: this.time,
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
  plasma_vd = $state<number>(0.021);
  schedule = $state<AccessPeriod[]>([]);
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
    schedule: AccessPeriod[];
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
      schedule: this.schedule,
      dose_concentration: this.dose_concentration,
    };
  }
}

export class TetoffState {
  brain_rma = $state<number>(0);
  plasma_rma = $state<number>(0);
  tta = $state<number>(0);
  plasma_dox = $state<number>(0);
  brain_dox = $state<number>(0);

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

export class OscillatingModel {
  prod = $state<number>(0.2);
  bbbTransport = $state<number>(0.6);
  deg = $state<number>(0.007);
  freq = $state<number>(0.0138);

  /**
   * Serialize this model to a plain object for Tauri/Rust serialization.
   * Rust expects snake_case field names based on the `rename_all = "snake_case"` attribute.
   */
  toJSON(): { prod: number; bbb_transport: number; deg: number; freq: number } {
    return {
      prod: this.prod,
      bbb_transport: this.bbbTransport,
      deg: this.deg,
      freq: this.freq,
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
  ) {
    if (isTauriEnv) {
      let solution = await invoke("simulate_oscillating_model", {
        model: this.toJSON(),
        init_state: init_state.toJSON(),
        t0,
        tf,
        dt,
      });

      return solution;
    } else {
      console.log("running in the browser. Use wasm");
    }
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
