# Hillslope Mechanical Winter-Overhead Sweep (sub-5× WP-1)

Status: IN PROGRESS

Package id: `20260701-hillperf-mechanical-winter-overhead-001`

## Objective

Execute the identity-preserving lane of
`docs/backlog/20260701-hillslope-sub5x-performance-assessment.md` — findings
F2, F3, F5, F6, F7, F8 — as one consolidated package. Every change in this
package must leave the H2637 protected outputs byte-identical; there is no
science or contract decision anywhere in scope. The contract-adjudicated
finding (F1, frost single-solve) is deliberately excluded and follows as its
own package.

Performance context: H2637 direct endpoint measured 70.16/72.63 s vs legacy
9.63/9.67 s (7.40×) on 2026-07-01; the `<=5x` budget is 45.6 s against the
9.12 s spec anchor. This package alone is projected to land ~52–56 s; F1
carries the remainder.

## Execution model (operator-ratified 2026-07-01)

- Claude implements end-to-end (explicit operator direction, breaking the
  default Codex-authors-code split; precedent FARPOINT01 F-B).
- Codex performs the independent package review before close.
- Work happens on the `worktree-hillperf-sub5x` worktree branch because
  watershed perf packages are concurrently executing on `main`; merge to
  `main` after review, with one identity re-run on the merged tree.
- Timing discipline on the shared host: per-finding endpoint times are
  recorded as *indicative single-rep* values; the binding 3-rep timing
  evidence is captured in a quiet window (watershed packages idle) at package
  end. Identity gates and the test suite are load-insensitive and run freely.

## Authority

- `docs/backlog/20260701-hillslope-sub5x-performance-assessment.md` (evidence
  and finding definitions; Ran-class profile of 2026-07-01).
- `docs/architecture/array-native-runtime-specification.md` §6 (endpoint
  authority; stop criteria — no new cache layer without deleting a boundary).
- ADR-0003 (protected-output identity as the regression gate for
  behavior-preserving change).

## Scope

In scope (write set):

- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/**`
  (typed-boundary guards, winter coupling) — F2, F5, F7.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**` — F6, F8.
- `crates/openwepp-runner/src/hillslope/direct_publication/**` — F3, F6.
- `crates/openwepp-kernel-contract` symbol-construction surface only if F2
  requires a deferred-construction helper there.
- Focused tests for changed helpers; package artifacts.

Out of scope: F1/F4 (contract-gated), any physics or operand-order change,
any new lookup/cache layer over a runtime boundary, watershed code.

## Findings and per-finding acceptance

Common gate for every finding: `cargo nextest run --workspace` green and the
H2637 identity gate (below) clean; one commit per finding; a finding whose
verification pre-step fails is **converted to a documented non-viable entry**
in the closure artifact and the package continues (convert, don't hold).

| # | Finding | Change shape | Pre-step |
|---|---|---|---|
| F7 | `diagnostic_count_to_f64` does `to_string().parse()` | `value as f64` (exact for counts < 2^53) | — |
| F2 | Guard `BoundarySymbol` Strings built on the success path | defer symbol construction to the failure branch (closure/late construction); error strings byte-identical | — |
| F6 | Trace events + erosion inputs cloned before their disabled-checks; shadow projections cloned where a move suffices | construct-behind-the-gate; move-not-clone | — |
| F3 | 24-h winter hourly forcing built twice per (lane, day) builder-side | build once, share | prove the two `DirectWinterHourlyContext` argument sets are field-identical |
| F5 | `fit_legacy_tmpcft_curve` re-derived per solve | memoize per (lane, day) | prove the curve inputs are per-(lane, day)-stable |
| F8 | 14 discarded phase views + audit atomics per OFE-day | compute counts arithmetically; **manifest counter values must be unchanged** | — |

## H2637 identity gate

Reference hashes (frozen 2026-07-01 from the profiled baseline binary,
hillslope source = this branch's base commit):

```
18c7ddcd8b5b4205876e47e82eaa3931d56db0b98d37f96d5dcebb50b7f85c2e  H2637.hbp
32977b750cf399c98687910b1ff612d5d11c7b1688c77b7eaeb83fbc99559549  H2637.loss.json
f4de3e5c2224556e6c913d6ca12d807415da56a07b182d4e3238fec1879a6e22  H2637.pass.parquet
26d4b9415820e6da2e16869f2f926a8b5ddd39c565dfff612a0551477b7e09f6  H2637.wat.parquet
19dc44f2e8ae462037cf468413253ce0b1e5a4ecf08da441a3daf2c7dfb04142  H2637.plot.parquet
```

Gate: build the modified binary, run the H2637 fixture (WB05A replay inputs,
all five outputs), `sha256sum -c` against the reference. The manifest is
compared as a field diff with an allowlist of `source_commit` only; any other
manifest field change fails the gate. Entry gate for the package: the clean
worktree binary must reproduce these hashes before the first finding lands.

## Exit gates

1. All findings landed or dispositioned non-viable; one commit each.
2. `cargo fmt --check`, `cargo clippy --workspace --all-targets`
   (warnings denied), `cargo nextest run --workspace`, `cargo deny check`
   green.
3. H2637 identity clean on the final branch state.
4. Quiet-window 3-rep H2637 timing + fresh `perf` profile recorded in
   artifacts; backlog assessment updated with measured deltas.
5. Codex review dispatched; findings dispositioned; merged to `main` with one
   post-merge identity re-run.

## Stop conditions

Stop and surface (do not force): an identity break a finding cannot avoid
after one honest attempt (skip the finding, record it, continue — stop the
package only if the *entry* gate itself cannot be reproduced); a required
manifest counter change outside F8's value-preserving design; any change that
would require touching physics operand order.
