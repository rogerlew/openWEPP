//! D10B: dump the Iwagaki Case-4 oracle headline metrics (both
//! constructions) for the package evidence record.

use openwepp_hillslope_orchestrator::ofe_routing::iwagaki_oracle::{
    OracleConfig, run_oracle, run_upwind_reference,
};

fn main() {
    let cfg = OracleConfig::iwagaki_case4();
    for cells in [2000usize, 4000, 8000, 16_000] {
        let r = run_upwind_reference(&cfg, cells);
        println!(
            "{{\"oracle\": \"upwind\", \"cells\": {}, \"peak_m2_s\": {:.8}, \"t_peak_s\": {:.4}, \"rise_10_90_s\": {:.4}, \"mass_residual_rel\": {:.3e}}}",
            cells,
            r.peak_m2_s,
            r.time_to_peak_s,
            r.rise_10_90_s.unwrap_or(f64::NAN),
            r.mass_residual_rel
        );
    }
    for (cells, sample_dt, max_dt) in [
        (120usize, 0.25, 0.125),
        (240, 0.125, 0.0625),
        (480, 0.0625, 0.03125),
        (960, 0.03125, 0.015_625),
    ] {
        let run = openwepp_hillslope_orchestrator::ofe_routing::dval::run_iwagaki_manning(
            cells, sample_dt, max_dt,
        )
        .expect("manning case4 runs");
        let rise = openwepp_hillslope_orchestrator::ofe_routing::dval::sampled_rise_time_10_90(
            &run.hydrograph,
        )
        .unwrap_or(f64::NAN);
        println!(
            "{{\"solver\": \"manning\", \"cells\": {}, \"peak_m2_s\": {:.8}, \"t_peak_s\": {:.4}, \"rise_10_90_s\": {:.4}, \"max_courant\": {:.4}, \"tv_increase_m2_s\": {:.3e}}}",
            cells,
            run.peak_m2_s,
            run.time_to_peak_s,
            rise,
            run.max_courant,
            run.diagnostic_max_homogeneous_tv_increase_m2_s
                .unwrap_or(f64::NAN)
        );
    }
    let moc = run_oracle(&cfg);
    println!(
        "{{\"oracle\": \"characteristics_fan\", \"particles\": {}, \"peak_m2_s\": {:.8}, \"t_peak_s\": {:.4}, \"rise_10_90_s\": {:.4}, \"mass_residual_rel\": {:.3e}, \"shocks_tracked\": {}}}",
        cfg.particles,
        moc.peak_m2_s,
        moc.time_to_peak_s,
        moc.rise_10_90_s.unwrap_or(f64::NAN),
        moc.mass_residual_rel,
        moc.shocks_tracked
    );
}
