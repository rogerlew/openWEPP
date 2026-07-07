# SC-OFEROUTE-002 Verification Agent B

Lane: Agent B verification of review-response fixes in
`artifacts/science-contracts/SC-OFEROUTE-002/disposition.md`.

Evidence:

- Static: read root `AGENTS.md` from the prompt, `docs/work-packages/AGENTS.md`,
  `docs/specifications/science-contracts/AGENTS.md`,
  `docs/specifications/science-contract-spec.md`, package `package.md`,
  `disposition.md`, `review_agent_b.md`, `SC-OFEROUTE-002.md`,
  `SC-OFEROUTE-001.md`, `docs/specifications/science-contracts/index.md`, and
  current implementation/test surfaces in `implicit_recession.rs`, `cascade.rs`,
  `kinematic_wave.rs`, `profile.rs`, and runner selector/profile plumbing.
- Ran: scoped documentation/contract gates listed below.

Verdict: **NO-GO** for approval lift.

All Medium Agent-B fixes are verified, and B-L2 is verified. One Low Agent-B
fix remains incomplete: the guard-map row for `INV-OFEHYB-006` still cites a
module/family shorthand instead of actual retained test function names.

## Findings

### Low: B-L1 is still only partially fixed for the deficit-carry guard-map row

`SC-OFEROUTE-002`'s guard map now names concrete tests for most rows, but the
`INV-OFEHYB-006` row still says "`rev30_deficit_carry_tests` functions" rather
than naming the retained test functions individually
(`docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md:309`).
That is the same class of family-label anchoring Agent B asked to replace with
actual retained test names.

The actual retained tests exist and are statically confirmed:

- `absorb_deficit_exact_total_and_non_negative`
  (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:1073`)
- `dispose_terminal_carry_material_deficit_fails_closed`
  (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:1099`)
- `dispose_terminal_carry_subnoise_absorbs_backward_exactly`
  (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:1116`)
- `dispose_terminal_carry_all_dry_subnoise_drop_is_bounded`
  (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs:1150`)
- `bin_recorder_returns_material_terminal_deficit_exactly`
  (`crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs:1388`)

Required amendment: replace the module shorthand in the `INV-OFEHYB-006`
guard-map row with the actual function names above, or explicitly justify why
this row is exempt from B-L1's actual-test-name requirement.

## Verified Fixes

**B-M1: verified.** The C-L1 bounded all-dry/insufficient-gross exception is
threaded through the required outputs, Algorithm 5.4/5.5, `INV-OFEHYB-006`,
`OBL-OFEHYB-P-001`, and `OBL-OFEHYB-C-001`
(`SC-OFEROUTE-002.md:104`, `SC-OFEROUTE-002.md:249`,
`SC-OFEROUTE-002.md:255`, `SC-OFEROUTE-002.md:295`,
`SC-OFEROUTE-002.md:319`, `SC-OFEROUTE-002.md:327`). The current
implementation matches the bounded disposition: material deficits fail closed,
sub-noise deficits absorb backward when possible, and all-dry sub-noise carry is
dropped without publishing negative bins (`cascade.rs:373`, `cascade.rs:387`,
`cascade.rs:393`, `cascade.rs:396`, `cascade.rs:402`).

**B-M2: verified.** The transactionality wording now matches the current
low-level API. `SC-OFEROUTE-002` says no `Ok` returns an unvalidated pair, but
the low-level buffers are undefined on typed failure and callers must fail the
window closed (`SC-OFEROUTE-002.md:195`, `SC-OFEROUTE-002.md:198`,
`SC-OFEROUTE-002.md:320`). That matches `implicit_step_with_discharges`, which
mutates `depth_m` / optional discharge buffers during the march before the
residual guard and returns `ImplicitSolveNonConvergence` on guard failure
(`implicit_recession.rs:164`, `implicit_recession.rs:165`,
`implicit_recession.rs:194`).

**B-M3: verified for scoped lifecycle surfaces.** `SC-OFEROUTE-002` front matter
uses `status: draft` / `maturity: draft`; the body header uses `Status: draft`
and `Maturity: draft`; the registry row uses `draft` / `draft`
(`SC-OFEROUTE-002.md:4`, `SC-OFEROUTE-002.md:5`,
`SC-OFEROUTE-002.md:23`, `SC-OFEROUTE-002.md:24`,
`docs/specifications/science-contracts/index.md:56`). The experimental
subsystem posture remains in the body and BEI instead of lifecycle fields
(`SC-OFEROUTE-002.md:29`, `SC-OFEROUTE-002.md:392`).

**B-L2: verified.** `SC-OFEROUTE-001` front matter and the registry row both
carry `last_reviewed: 2026-07-07`
(`SC-OFEROUTE-001.md:17`, `docs/specifications/science-contracts/index.md:55`).

## Non-Blocking Notes

- `SC-OFEROUTE-001` rev-32 revision-history prose still records
  `SC-OFEROUTE-002` as "status draft / maturity experimental"
  (`SC-OFEROUTE-001.md:483`). I am not counting this as B-M3's scoped blocker
  because B-M3 named front matter/body/registry lifecycle surfaces, all of
  which are corrected. It is stale canonical-contract prose and should be
  cleaned with the final approval-lift edit.
- The package objective still contains the original `Maturity: experimental`
  planning wording (`package.md:37`). This is package-local historical scope
  text, not a lifecycle field, but final cleanup could avoid future confusion.

## Gates Run

- `markdown-doc lint --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md --path docs/specifications/science-contracts/index.md --path docs/work-packages/20260707-laned-router-hybrid-contract-authority-001`:
  PASS, 11 files, 0 errors, 0 warnings.
- `python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`:
  PASS-DEFERRED, 4 binding exposure rows, 4 science-review-follow-on rows.
- `python tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`:
  PASS-DEFERRED, 7 binding exposure rows, 6 science-review-follow-on rows.
- `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-002.md`:
  PASS.
- `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`:
  PASS.
- `git diff --check`: PASS.
- `rg` static test-name lookup over `implicit_recession.rs`, `cascade.rs`,
  `kinematic_wave.rs`, and `d10b_reconciliation_tests.rs`: PASS, retained test
  names found.

No contract or production code was modified.
