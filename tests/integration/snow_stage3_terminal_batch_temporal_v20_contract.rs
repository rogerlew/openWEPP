use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

#[test]
fn coordinated_v20_v10_v138_v5_define_the_complete_authority_surface() {
    let snow = read("docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md");
    let lse = read("docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md");
    let freeze = read("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    let time = read("docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md");
    let inventory = read("docs/work-packages/20260821-snow-stage3-v11-covered-consumer-runner-closure-001/artifacts/terminal-residual-state-inventory-v1.md");

    for required in [
        "candidate_contract_version: 20",
        "S(x1)-S(x0) - I_prescribed(H) - Q_endpoint",
        "BE: Q_endpoint = h F(t1,x_BE)",
        "CN: Q_endpoint = h/2 [F(t0,x0)+F(t1,x_CN)]",
        "Only the CN candidate may be installed",
        "INV-SNOWENERGY-056",
        "INV-SNOWENERGY-057",
        "INV-SNOWENERGY-058",
        "INV-SNOWENERGY-059",
        "INV-SNOWENERGY-060",
        "OBL-SNOWENERGY-P-015",
        "OBL-SNOWENERGY-C-023",
        "0.600000001",
        "1.199999999",
        "zero\nconstitutive calls below `600 ms`",
    ] {
        assert!(snow.contains(required), "SnowEnergy v20 missing {required}");
    }

    for required in [
        "candidate_contract_version: 10",
        "CoveredLseEndpointFluxSetV1",
        "never converted to endpoint\nrates",
        "LSEB-E-046",
        "INV-LANDSURFACEENERGY-130",
        "INV-LANDSURFACEENERGY-131",
        "INV-LANDSURFACEENERGY-132",
        "INV-LANDSURFACEENERGY-133",
        "599999999/600000000/600000001 ns",
    ] {
        assert!(lse.contains(required), "LSE v10 missing {required}");
    }

    for required in [
        "candidate_contract_version: 138",
        "CoveredTerminalBatchTrialRequestV2",
        "Caller-supplied event\nticks or ending hints are prohibited",
        "advances shared\nvegetation, LSE, surface-liquid, hydrology, BGC and soil-thermal owners exactly\nonce",
        "SNOWFREEZE-E-TERMINAL-ACTIVESET-002",
        "covered-terminal-batch-request-v2",
        "INV-SNOWFREEZE-106",
        "INV-SNOWFREEZE-107",
        "INV-SNOWFREEZE-108",
        "INV-SNOWFREEZE-109",
        "INV-SNOWFREEZE-110",
    ] {
        assert!(freeze.contains(required), "SnowFreeze v138 missing {required}");
    }

    for required in [
        "candidate_contract_version: 5",
        "CoveredTerminalBatchTrialResultV2",
        "CoveredTerminalGroupTopologySetV2",
        "CoveredTerminalZeroPrefixReceiptV1",
        "covered-terminal-zero-prefix-receipt-v1",
        "provider_call_count:u32=0",
        "physical_mutation_count:u32=0",
        "event_posture:u8=0",
        "ERR-CT-027",
        "ERR-CT-028",
        "ERR-CT-029",
        "INV-COUPLEDTIME-027",
        "INV-COUPLEDTIME-028",
        "INV-COUPLEDTIME-029",
        "INV-COUPLEDTIME-030",
        "INV-COUPLEDTIME-031",
    ] {
        assert!(time.contains(required), "CoupledTime v5 missing {required}");
    }

    for required in [
        "seven owner payloads remain opaque exact custody objects",
        "Prescribed totals versus endpoint fluxes",
        "does not return the coarse/fine",
        "No component value is fabricated",
    ] {
        assert!(inventory.contains(required), "inventory missing {required}");
    }
}

#[test]
fn successor_preserves_floor_tolerances_and_scope_holds() {
    let snow = read("docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md");
    let lse = read("docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md");
    let freeze = read("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    let time = read("docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md");

    assert!(snow.contains("a_mass=1e-9 kg m^-2"));
    assert!(snow.contains("a_energy=1e-6 J m^-2"));
    assert!(snow.contains("relative `1e-8`"));
    assert!(lse.contains("dt>=600000000 ns"));
    assert!(freeze.contains("preserving v137"));
    assert!(time.contains("preserving reviewed version 4"));
    assert!(snow.contains("authorizes no Rust implementation"));
    assert!(freeze.contains("admits no production wiring"));
    assert!(time.contains("admits no implementation"));
}
