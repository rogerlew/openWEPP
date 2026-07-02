//! MOFEFID-D7 D-val CLI dumper: runs a validation case via
//! `ofe_routing::dval` and prints the outlet hydrograph as CSV (`t_s,q_m2s`).
//! Summary to stderr. Usage:
//! `cargo run --example dval_case -- <case> [ko] [ks_mm_h]`
//! or for Case 4 diagnostics:
//! `cargo run --example dval_case -- 4 <ko> [cells] [sample_dt_s] [max_dt_s]`.
use openwepp_hillslope_orchestrator::ofe_routing::dval;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let case: u32 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(4);
    let ko: f64 = args.get(2).and_then(|s| s.parse().ok()).unwrap_or(500.0);
    let third_arg = args.get(3);
    let ks: Option<f64> = third_arg.and_then(|s| s.parse().ok());
    let run = match case {
        4 => {
            let cells = third_arg.and_then(|s| s.parse().ok()).unwrap_or(120);
            let sample_dt = args.get(4).and_then(|s| s.parse().ok()).unwrap_or(1.0);
            let max_dt = args.get(5).and_then(|s| s.parse().ok()).unwrap_or(0.5);
            dval::run_iwagaki_with_options(ko, cells, sample_dt, max_dt)
        }
        1 => dval::run_rain_case(&apply_ks(dval::case1_bare(), ks)),
        2 => dval::run_rain_case(&apply_ks(dval::case2_isolated(), ks)),
        3 => dval::run_rain_case(&apply_ks(dval::case3_vegetation(), ks)),
        _ => {
            eprintln!("unknown case {case}");
            std::process::exit(2);
        }
    }
    .expect("dval run");
    eprintln!(
        "case={case} ko={ko} sampled_peak={:.6e} sampled_t_peak={:.2} substep_peak={:?} substep_t_peak={:?} maxCr={:.3} RC={:?}",
        run.peak_m2_s,
        run.time_to_peak_s,
        run.diagnostic_substep_peak_m2_s,
        run.diagnostic_substep_time_to_peak_s,
        run.max_courant,
        run.runoff_coefficient
    );
    println!("t_s,q_m2s");
    for s in &run.hydrograph {
        println!("{:.3},{:.8e}", s.time_s, s.q_m2_s);
    }
}

fn apply_ks(mut c: dval::RainCase, ks: Option<f64>) -> dval::RainCase {
    if let Some(k) = ks {
        c.ks_mm_h = k;
    }
    c
}
