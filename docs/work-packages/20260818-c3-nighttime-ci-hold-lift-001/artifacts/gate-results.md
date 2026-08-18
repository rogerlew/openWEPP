# Gate Results

Status: `IMPLEMENTATION PASS / COMPLETE-DAY HOLD`

No completion gate is claimed before the complete 48-interval provider day,
persisted restart, Child 4, and campaign gates pass.

Current implementation gates apply to committed tree
`40fa0c484881e560e2904ade51e3754b17436fb3`:

- affected three-crate quick Nextest: PASS, 1036/1036, run
  `1ccaaad9-e47e-4d4b-92c0-986aeefc4583`;
- V10 authority target: PASS, 3/3;
- historical vegetation authority target: PASS, 27/27;
- authority-suite anti-evasion: PASS;
- AUTH11 required-suite obligation guards: PASS, 3/3;
- affected three-crate all-target Clippy with warnings denied under the pinned
  Nix shell (`cargo 1.95.0`, `clippy 0.1.95`): PASS;
- `cargo fmt --all -- --check`: PASS.
- `git diff --check`: PASS.

Current bounded numerical increment on that commit plus the present diff:

- affected three-crate quick Nextest: PASS, 1039/1039, run
  `1141c94f-8f5a-4f6b-ae76-babf4a882cdd`;
- V10 authority target: PASS, 3/3, run
  `56006e1f-5cf0-40b1-8db6-d3ac6a82eee3`;
- authority-suite anti-evasion: PASS;
- AUTH11 required-suite obligation guards: PASS, 3/3, run
  `fd01c6a5-4f43-4af2-bfdf-a60942873a7b`;
- affected LSE/orchestrator all-target Clippy with warnings denied in Nix:
  PASS;
- `cargo fmt --all -- --check`: PASS;
- `git diff --check`: PASS.

Exact current bindings:

- SC-VEGETATION-001 V14: `2432870da936959b1a16f4afc87540a4e1802bd9eccfccf615776daf6e262495`;
- V10 definition: `9133d41fa67c5a8e3d50690cbdc24ebe06b8647fcdd569c0f1646618fb9fa947`;
- SC-LANDSURFACEENERGY-001 V4: `ef405aa0fe9ea15ea95568637fa3ac9ca3d626eeadcc1d56e064b6d051d4c061`;
- LSE-V2 definition: `86eb2f5bdb7db494c80e90d1395799aa85c1ffcca3ba6698ca2503cba902d99f`.

The canonical midnight interval and complete 48-interval zero-radiation
provider day execute successfully. Interval 15 initially rejected in the
uncapped potential pass; the frozen scaled matrix is rank 28 and its null
direction is the lower occupancy wet-surface temperature backed by only
`6.776263578034403e-21 kg m^-2` of store. V2 now uses the contract-bound
inactive anchor only when that store-cap-active rate is below the canonical
water tolerance. The complete-day regression and an interval-15 rollback
poison pass.

A realistic positive-radiation day remains HOLD at its first low-light
positive-PAR interval with historical `ConstitutiveDomain("ci_bracket")`.
That branch is outside the newly declared exact-zero-PAR V10 correction, so
persisted restart, Child 4, and campaign closure have not started and no PASS
is claimed for them.

Rust review remediation after the first HOLD includes: finite differences
restricted to uncapped active zero-PAR solves; iteration-zero acceptance sealed
behind transaction FullSupply classification; partial ground supply returned
to the ordinary fixed-final path; request-batch digest validation at
finalization; private potential identity/candidate/batch fields with explicit
phase-to-batch lineage poisons; removal of the mixed-receipt V1 projection;
and an exact V10/LSE-V2 vegetation-receipt join with a poison.

Changed-file line governance remains below the hard stop. WARN-sized files are
`solver.rs` (2,798 lines), `transaction.rs` (2,382), and
`v9_real_consumer_shadow.rs` (2,243). Transaction tests were split into
`transaction_tests.rs`; the remaining large modules retain follow-on split
intent and are not allowed to cross 3,000 lines.
