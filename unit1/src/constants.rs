pub const T_INIT: f64 = 0.0;
pub const T_MAX: f64 = 200.0;
pub const DELTA_T: f64 = 0.01;
pub const T_STEP: f64 = 0.1;

pub const TAU_MEM: f64 = 20.0;
pub const E_LEAK: f64 = -60.0;
pub const V_RESET: f64 = -70.0;
pub const V_THRESH: f64 = -50.0;
pub const R_M: f64 = 10.0;
pub const I_EXT: f64 = 2.0;

pub const STEPS: usize = ((T_STEP / DELTA_T) + 0.5) as usize;
pub const VEC_CAPACITY: usize = (((T_MAX - T_INIT) / T_STEP) + 0.5) as usize;

pub const OUTPUT_PATH: &str = "output.txt";
