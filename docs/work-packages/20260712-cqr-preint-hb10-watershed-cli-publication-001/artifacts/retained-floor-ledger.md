# HB-10 Retained-Floor Ledger

Evidence class: **Ran + Static**

Authority: archived `hb10-final.lcov` and `hb10-final-crap.json`. `Regions` is
the exact covered/total fraction underlying cargo-crap coverage. Reviewer A
and reviewer B independently accept every row below as
`R-LOW-COMPLEXITY-PRODUCTION`: CRAP is at most 30, uncovered behavior is
fail-closed validation or already bounded orchestration, and named direct plus
real-CLI evidence prevents a producer-only disposition.

| Function | Regions | Cov. | CC | CRAP | Uncovered branch summary | Named evidence | Review A / B |
| --- | ---: | ---: | ---: | ---: | --- | --- | --- |
| `parse_watershed_runfile_hillslopes` | 85/127 | 66.929% | 17 | 27.453 | Additional generated/reuse path error-priority alternatives. | CLI `wshedw2_watershed_cli_requires_explicit_reuse_mode`, `wshedw2_watershed_cli_rejects_ambiguous_reuse_block_with_run_file`, generated-mode success. | accept / accept |
| `validate_manifest_per_ofe_wb13_publication_counts` | 33/65 | 50.769% | 11 | 25.438 | Remaining malformed count/day combinations; no accepted arithmetic output. | Unit `validator_families_are_characterized`; CLI `watershed_cli_mofe05_rejects_multiofe_manifest_count_mismatch`, `watershed_cli_mf_accepts_valid_per_ofe_publication_metadata`. | accept / accept |
| `build_topology_from_watershed_structure` | 62/83 | 74.699% | 19 | 24.847 | Additional invalid contributor-kind/lookup combinations. | CLI `wshedw5_public_cli_uses_typed_network_and_publication_frames`, `watershed_cli_accepts_explicit_zero_impoundment_file_when_structure_has_none`, W11D baseline vector. | accept / accept |
| `run_watershed_plan` | 100/141 | 70.922% | 16 | 22.294 | Parser/network construction failure alternatives; successful execution and publication are direct. | Full 29-test `watershed_cli_behavior_contract`; CLI `wshedimpl14_baseline_authoritative_cli_lane_replays_baseline_ebe_signature`, `wshedw7r_p102_sediment_active_fixture_publishes_nonzero_sediment_and_jobs_identity`. | accept / accept |
| `manifest_mofe_hourly_carry_header` | 8/15 | 53.333% | 9 | 17.232 | Remaining malformed policy/active/count combinations. | Unit `validator_families_are_characterized`; CLI MOFE05 rejection and valid multi-OFE vectors. | accept / accept |
| `resolve_configured_watershed_sidecars` | 23/37 | 62.162% | 10 | 15.417 | Missing configured `chaninp`/`gwcoeff`/`tcr` path priorities and default lookup alternatives. | CLI `watershed_cli_legacy_discovery_matches_hillslope_unknown_sidecar_behavior`, generated relative-path vectors; unit `groundwater_authority_boundaries_are_characterized`. | accept / accept |
| `validate_manifest_per_ofe_wb13_publication_keys` | 6/11 | 54.545% | 7 | 11.602 | Remaining malformed monotonic and first/last-key combinations. | Unit `validator_families_are_characterized`; CLI `watershed_cli_mf_accepts_valid_per_ofe_publication_metadata`. | accept / accept |
| `validate_manifest_mofe_hourly_carry_metadata` | 30/41 | 73.171% | 9 | 10.564 | Narrow absent/inactive/single-OFE dispatch alternatives. | Unit `validator_families_are_characterized`; CLI MOFE05 missing/shape/count/valid vectors. | accept / accept |
| `validate_manifest_per_ofe_wb13_publication_policies` | 1/2 | 50.000% | 6 | 10.500 | Additional wrong storage/state policy values; all fail closed. | Unit `validator_families_are_characterized`; CLI valid M-F and malformed MOFE vectors. | accept / accept |
| `validate_watershed_runfile_document` | 11/24 | 45.833% | 5 | 8.973 | Schema/name/unit/empty-block first-error alternatives. | CLI missing/disallowed applicability and full valid-runfile vectors; unit `validator_families_are_characterized`. | accept / accept |
| `validate_watershed_runfile_applicability` | 25/43 | 58.140% | 6 | 8.641 | Other missing/disallowed selector positions with identical typed failure family. | CLI `watershed_cli_rejects_missing_applicability_selector_block`, `watershed_cli_rejects_disallowed_perennial_stream_selector`; unit `validator_families_are_characterized`. | accept / accept |
| `validate_manifest_mofe_hourly_carry_required_arrays` | 8/11 | 72.727% | 5 | 5.507 | Additional non-string/missing array-family positions. | Unit `validator_families_are_characterized`; CLI MOFE05 shape rejection and valid multi-OFE vector. | accept / accept |
| `validate_manifest_per_ofe_identity_statuses` | 13/23 | 56.522% | 4 | 5.315 | Other missing/wrong identity-status pointers. | Unit `validator_families_are_characterized`; CLI valid M-F vector. Strict residual numerics are separately above 75%. | accept / accept |
| `resolve_structure_contributor_local_id` | 13/22 | 59.091% | 3 | 3.616 | Wrong-kind success lookup alternative; zero and unresolved paths are direct. | Unit `simple_boundaries_are_characterized`; CLI typed-topology and zero-impoundment vectors. | accept / accept |
| `default_hillslope_binary` | 8/13 | 61.538% | 3 | 3.512 | Environment/current-executable failure alternatives. | CLI generated-mode serial/parallel vectors and explicit-binary failure vectors. | accept / accept |
| `validate_manifest_mofe_hourly_carry_inactive_single_ofe` | 13/21 | 61.905% | 3 | 3.498 | Nonempty versus missing inactive-array variants. | Unit `validator_families_are_characterized`; CLI single- and multi-OFE manifest vectors. | accept / accept |
| `validate_manifest_publication_policy` | 19/26 | 73.077% | 3 | 3.176 | Remaining unsupported policy spelling; missing, invalid and both accepted policies are bound. | Unit `validator_families_are_characterized`; CLI MOFE05 malformed/valid and M-F valid vectors. | accept / accept |
| `validate_manifest_schema` | 17/23 | 73.913% | 3 | 3.160 | Additional wrong schema value; missing/wrong/accepted behavior is direct. | Unit `validator_families_are_characterized`; CLI MOFE05 malformed and valid vectors. | accept / accept |

The strict numeric/authority rows are not exceptions: area source/native,
publication area, identity residual, carry totals, CRFRAC entry/mapping, and
groundwater authority all exceed 75% in the same terminal evidence set.
`print_help` is literal glue and is not an eligible production row.
