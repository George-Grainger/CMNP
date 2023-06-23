mod constants;
mod euler_integration;
mod float_loop;

use crate::constants::*;
use crate::euler_integration::euler_integration;
use crate::float_loop::float_loop;
use std::fs::File;
use std::io::Write;

fn main() {
    let dv_dt = |v_t_: f64| (E_LEAK - v_t_ + (R_M * I_EXT)) / TAU_MEM;

    let mut x_vals: Vec<f64> = Vec::with_capacity(VEC_CAPACITY);
    let mut y_vals: Vec<f64> = Vec::with_capacity(VEC_CAPACITY);
    let mut v_tt = V_RESET;
    let mut num_spikes: u32 = 0;

    for tt in float_loop(T_INIT, T_MAX, T_STEP) {
        if v_tt != 0.0 {
            let v_next = euler_integration(dv_dt, v_tt);
            v_tt = if v_next >= V_THRESH { 0.0 } else { v_next }
        } else {
            // Maybe caution in case spike is on very last time step?
            num_spikes += 1;
            v_tt = V_RESET;
        }

        x_vals.push(tt);
        y_vals.push(v_tt);
    }

    println!("Number of spikes: {}", num_spikes);
    println!("Saving data to {}", OUTPUT_PATH);
    let mut file = File::create(OUTPUT_PATH).unwrap();
    for (x_val, y_val) in x_vals.iter().zip(y_vals.iter()) {
        writeln!(file, "{} {}", x_val, y_val).expect("Failed to write to file");
    }
    println!("Data saved successfully!");
}
