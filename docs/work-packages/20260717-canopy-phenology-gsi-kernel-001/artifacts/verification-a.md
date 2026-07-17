# Terminal Verification A

**Evidence class:** Static and Ran
**Disposition:** HOLD

The terminal source preserves the Jolly GSI and FAO-56 equations, typed domain
failures, signed-latitude and polar behavior, deterministic arithmetic, strict
calendar admission, exact 21-value FIFO mechanics, and the downstream
integration hold. The current crate is still isolated from production
consumers, so the maximum supportable claim remains
`PASS-PROCESS-KERNEL`. No canopy, biomass, litter, hydrology, snow, erosion, or
empirical-validation claim is supported.

Two terminal evidence defects remain.

## Findings

### High — Required first-admission and successful restart vectors are absent

The strengthened FIFO test at
`crates/openwepp-plant-phenology/src/lib.rs:608` verifies the 20-sample mean,
the 21-sample mean, and uniquely identifiable oldest-value eviction. It no
longer asserts the first admission, although
`docs/specifications/science-contracts/contracts/SC-PLANT-001.md:796` requires
that vector.

The contract also requires history/date-anchor restoration vectors at
`SC-PLANT-001.md:803`. The tests at
`crates/openwepp-plant-phenology/src/lib.rs:718` through line 763 exercise only
failed restorations. No test successfully restores a heterogeneous FIFO plus
newest date, verifies the restored accessors, admits the next consecutive day,
and compares the result with uninterrupted execution. Consequently,
`artifacts/implementation.md:15` overstates the evidence when it says the 12
tests cover “exact restored state,” and the disposition does not fully close
the original chronology/restart finding.

**Required disposition:** Add a first-admission assertion and a positive
restart-equivalence vector. The latter should restore ordered heterogeneous
history and its newest date, admit the next calendar day, and compare history,
date, sample count, and output bits with an uninterrupted state.

### Moderate — The terminal CRAP JSON hash is mislabeled

`artifacts/heavy-gates.md:26` labels
`629bae489352c326b8dbccced737fce23a89c4d6858a72f28b0f6ebfbe48f07b`
as the “report JSON” SHA-256. The retained target artifacts show that this is
the hash of `target/adjudicated-crap/workspace-crap.json`. The actual
`target/adjudicated-crap/adjudicated-crap-report.json` SHA-256 is
`70556fc8c4eb1777175bf349be5873af02872a90e59d37368c888f49eaa09a5a`.
The gate result and counts are valid, but the named evidence identity is not.

**Required disposition:** Relabel `629b…` as the workspace CRAP JSON hash, or
record the actual adjudicated-report hash under the existing label.

## Closed Initial Findings

- `SC-PLANT-001` front matter now consistently identifies revision 21 and
  2026-07-17.
- The three-nontrivial-indicator product, independently reconstructed 20/21/
  eviction means, and ordinary-latitude FAO-56 anchor are present and pass.
- Guard-map and symbol-alias traceability covers `INV-PLANT-028..032` and the
  public GSI state and equation surfaces.
- The available-real-sample cold start is explicitly identified as an
  openWEPP inference rather than published Jolly initialization law.
- Repeated, skipped, and reversed dates fail before state mutation; common and
  leap-year rollovers pass.
- The planning record retains canopy/litter integration outside this package.

## Evidence Checks

Ran on terminal-current source:

| Check | Result |
| --- | --- |
| `cargo fmt --check` | PASS |
| strict package Clippy | PASS |
| package Nextest, quick profile | PASS, 12/12, run `8fb32a2c-7efa-448a-a9db-8b5034e49ba6` |
| contract and work-package Markdown lint | PASS, 13 files, 0 warnings |
| `git diff --check` | PASS |

Static retained-evidence checks:

- Current `crates/openwepp-plant-phenology/src/lib.rs` SHA-256 is
  `94d79dd78324a2e546bda0b753ed953b03353eda0e3588ad29748f4dd5c72b4d`,
  matching `target/adjudicated-crap/source-manifest-final.json`.
- The fresh adjudicated CRAP report is closure-eligible and passes with 9,746
  production entries, two raw rows, two valid adjudications, zero actionable
  rows, and zero touched-file actionable rows.
- The LCOV, source-manifest, and adjudication-registry hashes recorded in
  `heavy-gates.md` match the retained target artifacts.
- Source search found no production consumer outside the new crate. The source
  remains 913 lines and `SC-PLANT-001` remains 854 lines.

The long full-workspace coverage run was not repeated because its retained
source manifest matches the terminal production source and its CRAP result is
internally valid; only the human-facing JSON hash label is incorrect.
