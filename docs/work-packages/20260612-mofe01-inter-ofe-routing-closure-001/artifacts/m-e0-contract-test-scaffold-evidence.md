# M-E0 contract/test scaffold evidence

Status: executed-hold

Evidence mode: Ran + Static

## Scope

M-E0 executed the M-D contract-first scaffold:

- amend `SC-RUNOFFPART-001`, `SC-WATBAL-001`, and `SC-SYSTEM-001`,
- add contract-derived tests for the per-OFE dynamic-state architecture,
- prove the positive authority test passes,
- prove the current aggregate architecture fails structural red tests for the
  state collection, transfer payloads, and publication-policy manifest gate,
- stop before production implementation.

No runtime comparison was run. M-E0 changed no production runtime behavior, and
the user explicitly instructed comparisons without the comparator subagent only
where comparisons are needed. The M-E0 evidence boundary is the red contract
test, not a candidate-vs-legacy output comparison.

## Contract authority installed

`SC-RUNOFFPART-001`:

- version 43, `last_reviewed: 2026-06-13`,
- `INV-RUNOFFPART-029`,
- `MOFE01 M-E0 Per-OFE Runoff Lane-State Addendum`,
- binding exposure row
  `MOFE01-M-E0-PER-OFE-RUNOFF-LANE-STATE-ADDENDUM`.

`SC-WATBAL-001`:

- version 155, `last_reviewed: 2026-06-13`,
- `INV-WATBAL-097`,
- `MOFE01 M-E0 Per-OFE Dynamic Water-Balance State Addendum`,
- binding exposure row
  `MOFE01-M-E0-PER-OFE-DYNAMIC-WATER-BALANCE-STATE-ADDENDUM`.

`SC-SYSTEM-001`:

- version 79, `last_reviewed: 2026-06-13`,
- `INV-SYSTEM-030`,
- `MOFE01 M-E0 Per-OFE Dynamic-State Publication Policy Addendum`,
- binding exposure row
  `MOFE01-M-E0-PER-OFE-DYNAMIC-STATE-PUBLICATION-POLICY-ADDENDUM`.

Registry:

- `docs/specifications/science-contracts/index.md` updated to
  `Last updated: 2026-06-13`,
- review dates for `SC-RUNOFFPART-001`, `SC-WATBAL-001`, and
  `SC-SYSTEM-001` updated to `2026-06-13`.

## Tests installed

Added:

- `tests/integration/mofe01_per_ofe_state_contract.rs`
- `Cargo.toml` target `mofe01_per_ofe_state_contract`

Updated:

- `tests/integration/mofe01_inter_ofe_route_contract.rs`

The M-B authority smoke test no longer freezes the registry at the old M-B
review date. The M-E0 authority test also avoids pinning future-sensitive
global version/date values; it checks invariant/addendum/changelog authority and
exact registry row structure instead.

## Ran

| Command | Result | Evidence |
| --- | --- | --- |
| `cargo fmt --check` | PASS | Final post-edit format gate passed. |
| `cargo test --test mofe01_per_ofe_state_contract mofe01_me0_contract_authority_is_present -- --nocapture` | PASS | Positive M-E0 authority test passed. |
| `cargo test --test mofe01_inter_ofe_route_contract -- --nocapture` | PASS | Adjacent M-B authority smoke test passed after date-brittleness fix. |
| `cargo test --test mofe01_per_ofe_state_contract -- --nocapture` | FAIL | Expected red gate: 1 authority test passed; 3 structural red tests failed for missing per-OFE state collection, transfer payloads, and publication-policy manifest gate. |
| `cargo clippy --workspace --all-targets -- -D warnings` | PASS | Compiled all targets without executing the red test. |
| `cargo deny check` | PASS | `advisories ok, bans ok, licenses ok, sources ok`. |
| `markdown-doc lint --path docs/work-packages/20260612-mofe01-inter-ofe-routing-closure-001 --path docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md --path docs/specifications/science-contracts/contracts/SC-WATBAL-001.md --path docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md --path docs/specifications/science-contracts/index.md --format plain` | PASS | Final post-evidence run: 35 files validated, 0 errors, 0 warnings. |

## Final verification

Final Verification Agent A
(`019ebf2a-5e69-7952-a268-947caf803dad`) returned PASS after rerunning the
focused authority test, full expected-red M-E0 target, fmt, clippy, deny, and
scoped docs lint. The verifier found no blocking issues and confirmed the
package remains `active; M-E0 executed-hold`.

Final Verification Agent B
(`019ebf2a-5f28-7832-a780-d9a11ace1001`) returned PASS after read-only
governance checks. The verifier confirmed no production `crates/` edits, no
comparator use, allowed gate-result taxonomy only, separately recorded
clippy/deny gates, structural red-test quality, and explicit non-green
mergeability posture.

## Gate disposition

M-E0 is not a full Rust closure increment. The failing target is required and
intentional. Full `cargo test --workspace` is therefore blocked until M-E1 adds
the real per-OFE state collection or an explicitly contracted equivalent.

M-E1 must make all three structural red gates pass without weakening their
assertions or synthesizing per-OFE records from aggregate WB13/WAT rows. This
executed-hold increment is intentionally not a green/mergeable closure state.

## Claude review addendum (2026-06-13) — endorsed; two tracked items

Evidence mode: Ran (contract + test inspection) + Static.

**Endorsed.** M-E0 is a correct contract-first red scaffold:

- Contract amendments present across `SC-RUNOFFPART-001`, `SC-WATBAL-001`
  (`INV-WATBAL-097`), `SC-SYSTEM-001`, and the index.
- The per-element / transfer / hillslope-total identity *equations* are
  pinned (`INV-WATBAL-096`, from M-B, now referenced by the M-E0 structural
  invariant `INV-WATBAL-097` that makes them measurable): per-element
  `local_liquid + UpStrmQ + SubRIn = infiltration + Q_partition +
  Δdepression_storage + ε`; transfer at hourly resolution
  (`ui_SCrunf→ui_SUrunf`, `UpStrmQ=Σui_SUrunf`).
- Red contract test installed and failing as intended; disposition honest
  (executed-hold, red-by-design); gates classified per the non-deferral rule.

Two items to carry forward (neither blocks M-E1):

1. **Numeric tolerance/units not yet pinned.** `INV-WATBAL-096` says
   "residuals beyond tolerance hard-fail" but no concrete noise-floor number
   or unit basis (mm) is stated. M-E4 (where the identities first become
   measurable) must pin the exact tolerance — the FDHP01-era grade
   (`~1e-11`/`1e-13` mm) is the precedent — or its red test has no
   pass/fail line. Pin it in `SC-WATBAL-001` at E4, not later.
2. **Red tests are structural (type-existence), not behavioral.** The E0
   gates assert `PerOfeDailyWaterBalanceCollection`/`TransferInput`/
   `TransferOutput` exist — correct for a scaffold, but they go green the
   moment the types exist, regardless of whether the identities close. The
   **behavioral** identity tests (does `INV-WATBAL-096`'s equation close to
   the pinned tolerance on 2-OFE and 5-OFE fixtures) must land at M-E4 per
   the M-D breakdown and are the real acceptance — the structural tests must
   not be mistaken for it.

**Intentional-red-workspace note:** `cargo test --workspace` is now red by
design (3 structural gates) until M-E1+. This is the correct contract-first
state. A future agent must satisfy these by *building the surfaces*, never by
weakening the assertions — greening the workspace by relaxing the red gates
is a non-deferral violation.
