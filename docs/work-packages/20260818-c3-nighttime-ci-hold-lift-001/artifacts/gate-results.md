# Gate Results

## Exact-commit review and terminal verification

Commit `646e95b40` received an independent Rust review and two independent
terminal-verifier PASS verdicts. Verifier A ran focused Rust parity 1/1, V10
authority/regeneration 3/3 (Nextest `233f34d6-945a-4ad2-9858-e9e80a1d05bc`),
strict LSE Clippy, and exact diff checks. Verifier B ran in a clean detached
copy: focused parity 1/1, authority/regeneration 3/3 (Nextest
`d7c54cfa-1cf7-4051-984f-e255baac7578`), and exact diff checks. Both found no
blocking issue and confirmed actual five-vector production-path execution.

Disposition: `PASS` for the frozen-vector parity increment. The 2,924-line
`solver.rs` remains a nonblocking WARN with retained split intent.

Status: `V10/LSE-V2 IMPLEMENTATION PASS / RESTART AND CHILD-4 PENDING`

Resumed direct Rust-to-frozen parity increment:

- actual Rust V10 leaf-gas path against all five committed frozen PAR vectors:
  PASS, 1/1;
- V10 authority integration target (including independent Python
  regeneration): PASS, 3/3, Nextest run
  `4d959066-609e-47d7-90e8-68129a799be5`;
- detailed evidence: `artifacts/rust-frozen-vector-parity.md`;
- exact-current independent Rust review: PASS with no findings at
  `646e95b40`; terminal verification remains pending.

No completion gate is claimed before the complete 48-interval provider day,
persisted restart, Child 4, and campaign gates pass.

Baseline implementation gates apply to committed tree
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

Current nonpositive-assimilation increment on starting commit
`e5ddc63e7e904eaccff6ab09ebdfd992a40c705d` plus the present diff:

- affected three-crate quick Nextest: PASS, 1040/1040, run
  `f98652fe-c05f-4a47-b287-b60d9bcb9342`;
- LSE library: PASS, 60/60;
- V10 authority target: PASS, 3/3;
- complete 48-interval zero-radiation provider day: PASS, 1/1;
- complete 48-interval realistic positive-radiation provider day: PASS, 1/1,
  including interval 8 respiration-dominated positive low light;
- affected three-crate all-target Clippy with warnings denied in Nix: PASS;
- authority-suite anti-evasion: PASS;
- AUTH11 required-suite obligation guards: PASS, 3/3, run
  `ae2c8733-df69-4562-9239-cd2ea013cb45`;
- `cargo fmt --all -- --check`: PASS;
- `git diff --check`: PASS.

Exact current bindings:

- SC-VEGETATION-001 V14: `3a9f0a373259934e35472d90c07d6a54062b1407e576cd0b57dfbf66d12db174`;
- V10 definition: `0c42b025b6f9282d85afd5c8819ec9cc60d66a2b79ac6d5922bfdcc8026dd182`;
- SC-LANDSURFACEENERGY-001 V4: `9b6b12864e74bef5ef73eb56346c2527eb259e26bc73170b1347d1f27968b551`;
- LSE-V2 definition: `67d1681bf47c2b8b87d6195433209990b4021b7896bc50df973ac9246bfd6c19`.

The canonical midnight interval and complete 48-interval zero-radiation
provider day execute successfully. Interval 15 initially rejected in the
uncapped potential pass; the frozen scaled matrix is rank 28 and its null
direction is the lower occupancy wet-surface temperature backed by only
`6.776263578034403e-21 kg m^-2` of store. V2 now uses the contract-bound
inactive anchor only when that store-cap-active rate is below the canonical
water tolerance. The complete-day regression and an interval-15 rollback
poison pass.

A realistic positive-radiation day now accepts its positive-PAR low-light
interval through the explicit respiration-dominated branch. Gross assimilation
remains positive, net assimilation remains nonpositive, conductance remains
`g0`, and hydraulics use `beta_hyd=1` with `Egas=q1`. No light threshold was
introduced. Persisted restart, Child 4, and campaign closure are not claimed
by this increment.

Rust review remediation after the first HOLD includes: finite differences
restricted to uncapped active zero-PAR solves; iteration-zero acceptance sealed
behind transaction FullSupply classification; partial ground supply returned
to the ordinary fixed-final path; request-batch digest validation at
finalization; private potential identity/candidate/batch fields with explicit
phase-to-batch lineage poisons; removal of the mixed-receipt V1 projection;
and an exact V10/LSE-V2 vegetation-receipt join with a poison.

Changed-file line governance remains below the hard stop. WARN-sized files are
`solver.rs` (2,900 lines), `transaction.rs` (2,418), and
`v9_real_consumer_shadow.rs` (2,238). Transaction tests were split into
`transaction_tests.rs`; the remaining large modules retain follow-on split
intent and are not allowed to cross 3,000 lines.
