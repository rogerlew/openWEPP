//! Steady-state probe: case1 config, booked outflow rate vs sampled q.
use openwepp_hillslope_orchestrator::ofe_routing::kinematic_wave::{
    CellParameters, Forcing, KinematicWaveMesh, KinematicWaveSolver,
};

fn main() {
    let v = 60.0 / 3.6e6;
    for cells in [30usize, 60, 120, 240] {
        let mesh = KinematicWaveMesh::uniform(7.5, cells, CellParameters::bare(0.09, 500.0));
        let mut solver = KinematicWaveSolver::new(mesh);
        let excess = |_i: usize, _t: f64| v;
        let inflow = |_t: f64| 0.0;
        let intensity = |_t: f64| v;
        let forcing = Forcing {
            rainfall_excess_m_s: &excess,
            upstream_inflow_m2_s: &inflow,
            rainfall_intensity_m_s: &intensity,
        };
        let res = solver.run(&forcing, 3600.0, 10.0, 2.0).expect("run");
        let q_sampled = res.hydrograph.last().unwrap().outlet_unit_discharge_m2_s;
        // booked outflow rate over the last 600 s from the bins
        let bins = &res.outlet_bin_outflow_m2;
        let tail: f64 = bins[bins.len() - 60..].iter().sum::<f64>() / 600.0;
        println!(
            "cells={cells} q_sampled={q_sampled:.8e} q_booked_tail={tail:.8e} vL={:.8e} sampled/vL={:.4} booked/vL={:.4}",
            v * 7.5,
            q_sampled / (v * 7.5),
            tail / (v * 7.5)
        );
    }
}
