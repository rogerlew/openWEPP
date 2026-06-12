# Codex Review — Frost-Heave Frozen-Fringe Backlog

Date: 2026-06-12

Review target:
`docs/backlog/20260612-frost-heave-frozen-fringe-impedance-formulation.md`
from commit `68bfc810`.

Evidence labels:

- Static: inspected pinned baseline source under
  `/workdir/wepp-forest_260430_baseline/src`, openWEPP committed source at
  `68bfc810`, FDHP01 staged-plan/evidence artifacts, backlog README
  conventions, and local agent-memory storage metadata.
- Ran: `rg` source searches, `jq`/CSV metric reads, independent arithmetic
  recomputation with `.venv/bin/python`, local memory-store `sqlite3` lookup.
- Skipped: builds/tests, per docs-only review request.

## Findings

| ID | Disposition | Finding | Action |
| --- | --- | --- | --- |
| FHB-001 | accepted | Several legacy line citations in the backlog note were stale or imprecise. `frzng.for:404-407` does not contain the `saxfun` call/assignment, and `saxfun.for:63-69,76-82` does not cover both `-150.0` fallbacks. The underlying dead-gate claim is still correct. | Corrected the backlog to cite `frzng.for:396-403`, `saxfun.for:123-124`, and fallback ranges `saxfun.for:72-79,88-93`; tightened the `/2.0` citation to `frzng.for:405-414`. |
| FHB-002 | accepted | The explosion arithmetic is directionally correct, but the prose overstated the order comparison against `~mm/day` rates and used `3.34e8` rather than the source constant `3.35e8`. Independent recomputation gives uncapped flux `0.000323333 m/s = 1.164 m/hour`; the supply cap `2.25 mm/hour` gives `209.375 W/m2`, about `52x` a `4 W/m2` heat flux and `35..105x` across `6..2 W/m2`. | Corrected the backlog arithmetic wording to `1.16 m/hour`, `3.35e8`, `209 W/m2`, and "tens of thousands of times above mm/day-scale" rates. |
| FHB-003 | accepted | Agent-memory labels cited in the backlog are not durable local references in this checkout. `/home/roger/.codex/memories_1.sqlite` contains no `stage1_outputs` rows, so the names cannot be verified here. | Demoted memory references in the backlog to non-authoritative author-cited context and kept ADR-0017 / durable artifacts as authority. Follow-up only if those memories should be promoted into repo docs. |
| FHB-004 | rejected | Concern that the legacy `qwet` dead-gate claim might be unsupported. | Rejected. Static inspection confirms `frzng.for:393-394` sets active `frzftp = 0.0`; `saxfun.for:123-124` returns negative potentials; `frzng.for:410` therefore cannot activate for normal negative `wtpm`. The frozen-below guard, supply cap, and `watdst` call path also match the cited source structure. |
| FHB-005 | rejected | Concern that openWEPP already ports `qwet` or that the F5 `qdry` fallback claim conflicts with the staged plan. | Rejected for commit `68bfc810`. `rg '\bqwet\b|qwater' crates` returns no matches. `git show 68bfc810:.../coupling.rs` shows `lower_front_heat_w_m2` at lines `1070-1082` using `FROST_RUNTIME_UNFROZEN_CONDUCTIVITY_FALLBACK_W_M_K` directly, matching `d3-staged-increment-plan.md:618-639`. |
| FHB-006 | rejected | Concern that the Dd/F5/De sequencing or promotion posture changes the roadmap substance. | Rejected. The backlog remains concept/backlog, hard-gated behind FDHP01 D3/De certification and F4 snow disposition, defaulted behind MOFE unless a thaw-season erodibility sizing gate promotes it. This is consistent with `d3-staged-increment-plan.md:599-662`, the Dd evidence artifact, and backlog README conventions. |
| FHB-007 | deferred | The backlog's Dun 2010/Morris validation rationale remains an inference because the paper text was not checked in this review. | No backlog correction needed; the note already labels this as inference and carries an explicit open verification item in the roadmap. |

## Source Checks

- Legacy `frzng.for`: verified qwet guard `373-377`, Watanabe/front-potential
  comments and `frzftp` gate `385-394`, `saxfun` call/assignment `396-403`,
  activation and Darcy flux `410-414`, supply cap `419-425`, `qwet = lhfh2o *
  qwater` at `437`, and `watdst(qwater, flfzt, 1)` path `624-630`.
- Legacy `saxfun.for`: verified `varwtp` definition at `34`, invalid-layer
  fallback `72-79`, invalid-porosity fallback `88-93`, and normal
  negative-potential assignment `123-124`.
- Legacy `watdst.for`: verified mode flags `20-25`, including mode `1` for
  around-frozen-front redistribution.
- Legacy `cwint.inc`: verified heave/frost symbols `50-64`, including
  `amtfrz(mxplan)` at `63`.
- FDHP01 evidence: verified Dd summary metrics from
  `fdhp01_increment_dd_execution_summary_20260612.json` and staged-plan F5/De
  block at `d3-staged-increment-plan.md:599-662`.
- Internal consistency: backlog README conventions are met; the note remains a
  concept-stage item with constraints, equations, validation gates, and open
  questions.

## Verification

- Ran: `wctl doc-lint --path docs` — pass, `1220` files validated, `0`
  errors, `0` warnings.
- Ran: `git diff --check` — pass.
