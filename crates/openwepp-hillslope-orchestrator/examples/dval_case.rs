//! MOFEFID-D7 D-val CLI dumper: runs a validation case via
//! `ofe_routing::dval` and prints the outlet hydrograph as CSV (`t_s,q_m2s`).
//! Summary to stderr. Usage: `cargo run --example dval_case -- <case> [ko] [ks_mm_h]`.
use openwepp_hillslope_orchestrator::ofe_routing::dval;

fn main() {
    let case: u32 = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(4);
    let ko: f64 = std::env::args()
        .nth(2)
        .and_then(|s| s.parse().ok())
        .unwrap_or(500.0);
    let ks: Option<f64> = std::env::args().nth(3).and_then(|s| s.parse().ok());
    let run = match case {
        4 => dval::run_iwagaki(ko),
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
        "case={case} ko={ko} peak={:.6e} t_peak={:.2} maxCr={:.3} RC={:?}",
        run.peak_m2_s, run.time_to_peak_s, run.max_courant, run.runoff_coefficient
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
