use std::fs;

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

fn section<'a>(text: &'a str, start: &str, end: &str) -> &'a str {
    let start_index = text
        .find(start)
        .unwrap_or_else(|| panic!("missing section {start}"));
    let remainder = &text[start_index + start.len()..];
    let end_index = remainder.find(end).unwrap_or(remainder.len());
    &remainder[..end_index]
}

#[test]
fn coordinated_terminal_chronology_successors_bind_the_review_findings() {
    let snow_energy = read("docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md");
    let lse = read("docs/specifications/science-contracts/contracts/SC-LANDSURFACEENERGY-001.md");
    let snow_freeze = read("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    let coupled_time =
        read("docs/specifications/science-contracts/contracts/SC-COUPLEDTIME-001.md");
    let registry = read("docs/specifications/science-contracts/index.md");

    for required in [
        "candidate_contract_version: 19",
        "CoveredTerminalExecutionMode",
        "PersistentReject",
        "DiscoveryProbe",
        "ExactEndpoint { expected_tick }",
        "boundary=None",
        "Raw cloud fraction",
        "raw/scaled/fabricated carrier",
        "Schema-v2\nresolved beginnings",
        "INV-SNOWENERGY-051",
        "INV-SNOWENERGY-052",
        "INV-SNOWENERGY-053",
        "INV-SNOWENERGY-054",
        "OBL-SNOWENERGY-C-024",
        "nonempty,\nindependently reconstructed physical ledger",
    ] {
        assert!(
            snow_energy.contains(required),
            "snow-energy missing {required}"
        );
    }

    for required in [
        "candidate_contract_version: 9",
        "TerminalSnowSoilHeatReceiptV1",
        "No `273.15 K`",
        "INV-LANDSURFACEENERGY-127",
        "INV-LANDSURFACEENERGY-128",
        "INV-LANDSURFACEENERGY-129",
        "dormant installed-owner validation",
    ] {
        assert!(lse.contains(required), "LSE missing {required}");
    }

    for required in [
        "contract_version: 137",
        "OPENWEPP_STAGE3_CANONICAL_SNOW_OWNER_V4",
        "posture tag `0=ProducedUnconsumed`",
        "does not mutate\nsurface liquid, WB14",
        "INV-SNOWFREEZE-103",
        "INV-SNOWFREEZE-104",
        "INV-SNOWFREEZE-105",
        "sole event-ordinal authority",
    ] {
        assert!(
            snow_freeze.contains(required),
            "snow-freeze missing {required}"
        );
    }

    for required in [
        "candidate_contract_version: 4",
        "current search support `[cursor,b)`",
        "current physical-child ordinal",
        "Candidate iteration order and literal ordinal zero",
        "mutation_set` equals the canonical set",
        "INV-COUPLEDTIME-021",
        "INV-COUPLEDTIME-022",
        "INV-COUPLEDTIME-023",
        "INV-COUPLEDTIME-024",
        "INV-COUPLEDTIME-025",
        "ProducedUnconsumed parcel-set digest",
    ] {
        assert!(
            coupled_time.contains(required),
            "coupled-time missing {required}"
        );
    }

    for required in [
        "candidate v19 prospectively binds covered terminal modes",
        "candidate v9 prospectively adds terminal event-integrated snow--soil custody",
        "v137 prospectively adds canonical snow-owner V4 pending parcels",
        "candidate v4 prospectively binds current-search support",
    ] {
        assert!(registry.contains(required), "registry missing {required}");
    }

    let snow_bei = section(&snow_energy, "## Binding Exposure Index", "## Gap");
    let lse_bei = section(&lse, "## Binding Exposure Index", "## Gap Register");
    let freeze_bei = section(&snow_freeze, "## Binding Exposure Index", "## Known Gaps");
    let time_bei = section(&coupled_time, "## Binding Exposure Index", "## Child 2C");
    assert!(snow_bei.contains("SNOWENERGY-V19-COVERED-TERMINAL-CHRONOLOGY"));
    assert!(lse_bei.contains("LSE-V9-TERMINAL-SNOW-SOIL-ENDPOINT"));
    assert!(freeze_bei.contains("SNOWFREEZE-V137-STAGED-TERMINAL-OWNER"));
    assert!(time_bei.contains("BEI-CT-V4-COVERED-TERMINAL-CHAIN"));

    assert!(snow_energy.contains("sum_i(f_i m_i)=m_terminal_liquid"));
    assert!(snow_energy.contains("a_terminal_mass=1e-9 kg m^-2"));
    assert!(snow_energy.contains("a_terminal_energy=1e-6 J m^-2"));
    assert!(
        snow_energy.contains(
            "abs(delta_cumulative_unresolved_liquid-m_terminal_liquid) <=\na_terminal_mass"
        )
    );
    assert!(snow_energy.contains("0 <= Q_terminal_unallocated <=\na_terminal_energy"));
    assert!(snow_energy.contains("SNOWENERGY-E-TERMINAL-ENERGY-001"));
    assert!(
        snow_energy.contains(
            "| Invariant ID | Statement | Authority | Evidence | Guard | Failure posture |"
        )
    );
    assert!(lse.contains("q_ss,k,1+q_ss,k,2"));
    assert!(lse.contains("snow energy includes\n`-Q_ss`"));
    assert!(snow_freeze.contains("OPENWEPP_STAGE3_TERMINAL_PARCEL_V1\\0"));
    assert!(snow_freeze.contains("abs(sum_i(f_i*m_i)-m_terminal_liquid) <= a_terminal_mass"));
    assert!(coupled_time.contains("stage3-v11-terminal-group-preaccept"));
    assert!(coupled_time.contains("stage3-v11-terminal-group-accepted"));
    assert!(!coupled_time.contains("coalesced event-group digest\n-> accepted"));
}

#[test]
fn staged_terminal_authority_keeps_later_work_out_of_scope() {
    let snow_energy = read("docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md");
    let snow_freeze = read("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    for required in [
        "Runner construction, liquid receiver consumption",
        "restart, selectors, activation, CoE retirement, and cutover are not admitted",
        "Runner, receiver consumption, restart\nimplementation, activation, CoE retirement and cutover remain held",
    ] {
        assert!(
            snow_energy.contains(required) || snow_freeze.contains(required),
            "scope hold missing {required}"
        );
    }
}
