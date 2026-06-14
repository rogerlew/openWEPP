# Watershed Staged Increment Plan — Dispatch Artifact

Status: active - W-B executed-hold; W-C queued
Author: Claude Code, 2026-06-13
Template: FDHP01 `d3-staged-increment-plan.md` / MOFE01
`mofe-staged-increment-plan.md` (proven; agent memory
`staged-increment-port-template`).

## Universal rules (every increment)

- **Conservation is acceptance**: the totalwatsed3 identity + watershed-level
  water balance must close at the established floor on routed output. Identity
  checks use **independent operands** — no 0==0 / self-built / alias closure
  (the M-E4-REDO/M-I lesson, hard-won this series). A residual that is exactly
  0.0 is the tautology smell; genuine closure is nonzero-at-noise.
- **Hillslope inputs are settled**: MOFE01 HBP shards are read-only inputs;
  no per-OFE physics re-opened. Single-OFE/MOFE anchors stay unchanged by any
  watershed-side increment.
- **Comparator posture**: legacy is a flag (ADR-0017). Acceptance is
  conservation, not legacy-match.
- Contract-first per increment; red tests before production edits; commit each
  increment `executed-hold` until acceptance; truthful evidence labels;
  `Static:`/`Ran:`.
- Subagent requirement per package.md §4a (comparator_suite_runner REQUIRED
  for heavy runs).
- Cross-repo boundary: no wepppy production edits without explicit scope; name
  cross-repo needs as follow-ons.
- Read the lines, don't infer from symbol tables (the Dh lesson).

## Increment W-A — characterization + watershed routing scope (no production edits)

Ground everything in measured reality before any code (the M-A lesson):

1. **Watershed CLI current behavior**: run `openwepp-cli-watershed` on
   arboreal-dendrite with the closed MOFE01 hillslope HBP shards. Record where
   it fails/succeeds, what it routes (channel network from `chan.inp`), what
   watershed output it produces (or doesn't), and the full failure chain.
2. **The `jpond=0` impoundment finding**: read
   `watershed_impoundment.rs` — does the parser reject `jpond=0` (no
   impoundments) as `IMP-E-004`, and is that a **parser defect** (zero ponds
   is valid; should yield an empty impoundment set) or genuinely invalid
   input? Confirm against legacy impoundment-file handling. Cite lines.
   Record in `impoundment-no-pond-finding.md`.
3. **Routing + output seams**: map the channel-routing path
   (`openwepp-watershed-orchestrator`), the watershed-output schema
   (`openwepp-watershed-output`, the interchange contract), and what
   **totalwatsed3** (`wepppy/wepp/interchange/totalwatsed3.py`) expects as
   input — the exact columns/schema the audit consumes. Read the lines.
4. **Scope artifact** (`watershed-routing-scope.md`): legacy watershed-routing
   authority map (channel routing, pass-file consumption, watershed WB
   aggregation), openWEPP seam mapping, the totalwatsed3 input contract, the
   conservation identity (watershed-total = Σ hillslope contributions routed,
   closing against external inputs), red-test definitions, and the
   implementation increment breakdown + sizing.

- Gates: no production edits; evidence + scope artifact with current-tree
  file:line citations; the `jpond=0` finding classified (defect vs invalid)
  with evidence; the totalwatsed3 input contract documented.

W-A execution result (2026-06-13):

- Ran current watershed CLI on arboreal-dendrite with closed M-I HBP shards.
  It failed before `chan.inp`, HBP parsing, dispatch, or output writing:
  `CLIWAT-E-010` wrapping `IMP-E-004` on `pw0.imp` line 2, `jpond=0`;
  output file count was `0`.
- Classified `jpond=0` as a parser defect on valid no-impoundment input.
- Mapped the routing, output writer, and totalwatsed3 audit seams.
- W-B is the next implementation increment.

W-B execution result (2026-06-14):

- Amended the watershed runfile contract to pin explicit no-impoundment
  semantics: `inputs.pw0_imp` remains required in schema v1, but supported
  `.imp` files with `jpond=0` are valid typed empty impoundment sets when
  `pw0_str` declares zero impoundments.
- Added red/green parser and watershed CLI tests for explicit `jpond=0`.
- Implemented count reconciliation before the bare `jpond >= 1` guard, so
  `expected_structural_count=Some(0)` admits an empty item set, positive
  structural counts fail as `IMP-E-007`, and bare parses still fail as
  `IMP-E-004`.
- Re-ran the arboreal-dendrite watershed CLI. It now proceeds past
  `CLIWAT-E-010` and reaches the next hard stop:
  `CLIWAT-E-020` / `WKERNEL-WS10-CHANNEL-E-003`, with zero output files.
- W-C is the next implementation increment.

## Subsequent increments (refined by W-A; provisional)

- **W-B — impoundment no-pond handling**: executed-hold. Gate met:
  arboreal-dendrite watershed CLI proceeds past the impoundment parse, and
  existing impoundment fixtures are non-regressed.
**W-B execution result (2026-06-14, `ea95d372`) — accepted (Claude):** `jpond=0` accepted only for `expected_structural_count==Some(0)`; the structure-vs-file mismatch is preserved as a distinct typed error `IMP-E-007` (CountMismatch), `IMP-E-004` retained without structure context — guard preserved, not loosened. The absent-`.imp` case got an explicit decision (pw0_imp still required in schema v1, pinned in the runfile contract) — stricter than legacy (which never reads it) but deliberate and pinned. Red-first. The arboreal-dendrite CLI now clears `CLIWAT-E-010`.

**W-C now starts at the next hard stop:** `CLIWAT-E-020` / `WKERNEL-WS10-CHANNEL-E-003` (a channel-node *DomainViolation* in the WS10 watershed kernel; `kernel/types.rs:154`), 0 output files. W-C's first task is to characterize this exactly as W-A/jpond did: is it (a) valid channel input rejected (guard too strict — confirm legacy runs the same channel state), (b) a genuine domain violation (real bad routed channel state), or (c) missing routing data the channel kernel needs from the hillslope shards? Read the lines + compare to legacy `chnrt`/channel routing; do not assume defect.

- **W-C — watershed routing + output**: route the hillslope shards over the
  channel network to a watershed-level routed parquet (interchange schema).
  Gate: watershed water balance conserves against the closed hillslope inputs
  (independent-operand identity at noise); output schema matches the
  interchange contract.
- **W-D — totalwatsed3 end-to-end audit + closure**: run totalwatsed3 on the
  routed output (wepppy `.venv`; cross-repo validation). Gate: the
  totalwatsed3 identity closes at the established floor — the WBVAL06/6a
  deferral resolved. Closing obligations: ROADMAP item 1 removed, README
  execution log, handoff naming the next mechanism + cross-repo/sediment
  follow-ons.

(W-A sizing may split W-C/W-D further; each increment behind a conservation
hard stop.)

## Dispatch instructions

Each Codex dispatch: *"Execute increment <W-A|W-B|W-C|W-D> of
`docs/work-packages/20260613-wshed01-watershed-routed-outputs-totalwatsed3-closure-001/artifacts/watershed-staged-increment-plan.md`
end-to-end."* Required reading order: this plan; `package.md`;
`watershed-routing-scope.md` (once it exists); the MOFE01 + FDHP01 staged
plans (the failure modes these rules encode — tautology, clone, hollow
closure). An increment that cannot meet its gates backs out with evidence,
localized to its seam.
