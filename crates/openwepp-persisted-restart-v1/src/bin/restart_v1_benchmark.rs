use std::time::{Duration, Instant};

use openwepp_persisted_restart_v1::{
    DirectV10RestartHost, ExpectedRestartStaticContext, admit_checkpoint_v1,
    restart_authority_in_progress_checkpoint_fixture, to_canonical_bytes,
};

fn median(mut samples: Vec<Duration>) -> Duration {
    samples.sort_unstable();
    samples[samples.len() / 2]
}

fn main() {
    let (fixture, checkpoint, run, topology) = restart_authority_in_progress_checkpoint_fixture(24);
    let context = ExpectedRestartStaticContext {
        run_identity_sha256: &run,
        topology_sha256: &topology,
        vegetation_configuration: fixture
            .owners
            .runtime
            .shadow
            .restart_authority_vegetation_configuration(),
        vegetation_owner_id: fixture
            .owners
            .runtime
            .shadow
            .restart_authority_vegetation_owner_id(),
        soil_thermal_owner_id: &fixture
            .owners
            .runtime
            .shadow
            .restart_authority_soil_thermal()
            .owner_id,
        soil_thermal_configuration_sha256: &fixture
            .owners
            .runtime
            .shadow
            .restart_authority_soil_thermal()
            .configuration_sha256,
        lse_configuration: fixture
            .owners
            .runtime
            .shadow
            .restart_authority_lse_configuration(),
        surface_liquid_configuration: fixture
            .owners
            .runtime
            .shadow
            .restart_authority_surface_configuration(),
        gsi_configuration: fixture.owners.runtime.shadow.gsi_owner_configuration(),
        forcing_static_configuration: fixture
            .owners
            .runtime
            .shadow
            .provider_static_configuration(),
        phase_plan: &fixture
            .owners
            .runtime
            .shadow
            .restart_authority_hydrology_frame()
            .phase_plan,
        phase_plan_sha256: &fixture.owners.phase_plan_sha256,
        day_inputs: &fixture.owners.day_inputs,
        day_input_digests: &fixture.owners.day_input_digests,
    };
    let bytes = to_canonical_bytes(&checkpoint).unwrap();
    let mut serialization = Vec::new();
    let mut admission = Vec::new();
    let mut construction = Vec::new();
    let mut continuation = Vec::new();
    let mut finish = Vec::new();
    let mut abort = Vec::new();
    for _ in 0..20 {
        let start = Instant::now();
        let emitted = to_canonical_bytes(&checkpoint).unwrap();
        serialization.push(start.elapsed());

        let start = Instant::now();
        let restored = admit_checkpoint_v1(&emitted, &context).unwrap();
        admission.push(start.elapsed());

        let start = Instant::now();
        let mut host = DirectV10RestartHost::from_isolated(restored, &context).unwrap();
        construction.push(start.elapsed());

        let start = Instant::now();
        host.advance_to(48).unwrap();
        continuation.push(start.elapsed());

        let start = Instant::now();
        let _finished = host.finish().unwrap();
        finish.push(start.elapsed());

        let restored = admit_checkpoint_v1(&emitted, &context).unwrap();
        let host = DirectV10RestartHost::from_isolated(restored, &context).unwrap();
        let start = Instant::now();
        let _ = to_canonical_bytes(host.abort_to_day_beginning().unwrap()).unwrap();
        abort.push(start.elapsed());
    }
    println!("samples=20 checkpoint_bytes={}", bytes.len());
    for (name, values) in [
        ("interval24_serialize", serialization),
        ("interval24_admit", admission),
        ("interval24_isolated_restore", construction),
        ("remaining_24_intervals", continuation),
        ("finish", finish),
        ("abort_canonical_bytes", abort),
    ] {
        let maximum = values.iter().copied().max().unwrap();
        println!(
            "{name} median_us={} max_us={}",
            median(values).as_micros(),
            maximum.as_micros()
        );
    }
}
