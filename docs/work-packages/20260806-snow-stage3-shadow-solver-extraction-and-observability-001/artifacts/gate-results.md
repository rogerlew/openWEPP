# Gate Results

Status: PASS through exact-head heavy validation; terminal verification queued.

Evidence mode: Ran plus queued. TESTGATE was not used.

- Strict Binding Exposure: PASS, 10 rows.
- Science-contract unit compliance: PASS, no findings.
- Focused v128/predecessor contract pair: PASS, `10/10`.
- Focused implementation and independent review groups: PASS; see
  `implementation-test-evidence.md` and the three reviewer artifacts.
- Assurance source re-adoption: PASS at generation `221f8e51`; repeat check is
  unchanged, all three reports validate DRAFT, and public count is zero.
- Full exact-head formatting, Clippy, doctest, quick, frost, full, assurance,
  dependency, and cleanliness evidence: PASS at clean immutable commit
  `56f85c3a0bfbcea275de40db90b7e06ea14b34ad`.

The first heavy candidate `76578b41` passed format and then stopped on three
workspace-Clippy findings in `snow_surface_eb03_runtime.rs`: `float_cmp`,
`needless_pass_by_value`, and `match_wildcard_for_single_variants`. The
test-only remediation uses exact `to_bits()` comparison, a borrowed helper
argument, and an explicit `Kernel` match arm. Workspace all-target Clippy then
passes, and the affected runtime/contract group passes `36/36`. A new clean
candidate receives the complete heavy sequence; no later gate is inferred from
the stopped run.

The second heavy candidate `f9f219e8` passed all pre-profile gates and then
stopped at quick-profile test `production_runtime_sources_only_wire_stage0_flux_primitives_through_stage3_opt_in`
after `1,303` passes. The historical source guard allowed only the old Stage 3
monolith, so it rejected the behavior-preserving extracted solver/evaluation
modules and internal trace/parity consumers. The guard now enumerates those
exact authorized Stage 3 paths; it does not broaden by directory or token.
Frost and full were not inferred from the stopped run.

Final heavy evidence at `56f85c3a`:

| Gate | Result | Exact evidence |
| --- | --- | --- |
| Format | PASS | `cargo fmt --all -- --check`, 2.63 s |
| Workspace Clippy | PASS | `cargo clippy --workspace --all-targets -- -D warnings`, 2.47 s |
| Doctests | PASS | `cargo test --workspace --doc`, 20 crates / 0 cases, 6.20 s |
| Focused contracts | PASS | `10/10`, 0.051 s Nextest |
| Real schema-v5 consumer | PASS | `2/2`, 0.036 s Nextest |
| WAT/HBP/PASS byte parity | PASS | `1/1`, 0.208 s Nextest |
| Assurance | PASS | three DRAFT, zero public, source root `b33a109d`; 7.84 s |
| Dependency policy | PASS | advisories, bans, licenses, sources; 1.05 s |
| Quick | PASS | `2,212/2,212`, 57 slow, 40 profile-skipped; 2,273.085 s Nextest |
| Frost | PASS | `360/360`, 1 slow, 1,946 profile-skipped; 531.838 s Nextest |
| Full | PASS | `2,261/2,261`, 34 slow, 33 profile-skipped; 2,270.298 s Nextest |

The corrected Stage 0 guard passed in quick and full. All timing records bind
the same clean SHA with `dirty:false`. TESTGATE was not used. Evidence root:
`target/local-ci-history/snow-stage3-shadow-observability-56f85c3a/`. The only
warning was a non-failing unmatched `MIT-0` allowance in `cargo deny`.
