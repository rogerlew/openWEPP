# Gate Results

Status: `IMPLEMENTATION PASS / COMPLETE-DAY HOLD`

No completion gate is claimed before the complete 48-interval provider day,
persisted restart, Child 4, and campaign gates pass.

Current implementation gates on HEAD `d1f3094c22bbb2057bf1c8e0925574492be40d84`
plus the reviewed working-tree diff:

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

Exact current bindings:

- SC-VEGETATION-001 V14: `2432870da936959b1a16f4afc87540a4e1802bd9eccfccf615776daf6e262495`;
- V10 definition: `9133d41fa67c5a8e3d50690cbdc24ebe06b8647fcdd569c0f1646618fb9fa947`;
- SC-LANDSURFACEENERGY-001 V4: `d4de0a89d922ac7fb2e945dc57066421081ee0e038a2cb88d4a7ac80ae686c5b`;
- LSE-V2 definition: `409af8eb3577a9c1028f6b73a9be79ffa874e56efda70e30a105667448c7e812`.

The canonical midnight provider interval executes successfully. A diagnostic
attempt to execute the full provider-derived day rejects at interval 15 in the
uncapped potential pass with `NumericalSingular` (pivot
`2.740142887211839e-6`, matrix infinity norm `38567280187.35466`). This is a
load-bearing HOLD; the failed diagnostic edit was reverted and no PASS is
claimed for a complete day.

Rust review remediation after the first HOLD includes: finite differences
restricted to uncapped active zero-PAR solves; iteration-zero acceptance sealed
behind transaction FullSupply classification; partial ground supply returned
to the ordinary fixed-final path; request-batch digest validation at
finalization; private potential identity/candidate/batch fields with explicit
phase-to-batch lineage poisons; removal of the mixed-receipt V1 projection;
and an exact V10/LSE-V2 vegetation-receipt join with a poison.

Changed-file line governance remains below the hard stop. WARN-sized files are
`solver.rs` (2,777 lines), `transaction.rs` (2,382), and
`v9_real_consumer_shadow.rs` (2,145). Transaction tests were split into
`transaction_tests.rs`; the remaining large modules retain follow-on split
intent and are not allowed to cross 3,000 lines.
