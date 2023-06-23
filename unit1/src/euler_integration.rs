use crate::constants::*;

pub fn euler_integration(dv_dt: impl Fn(f64) -> f64, mut v_prev: f64) -> f64 {
    let mut t_prev = T_INIT;

    for _ in 1..=STEPS {
        let m = dv_dt(v_prev);
        let v_next = v_prev + DELTA_T * m;
        let t_next = t_prev + DELTA_T;
        t_prev = t_next;
        v_prev = v_next;
    }

    v_prev
}
