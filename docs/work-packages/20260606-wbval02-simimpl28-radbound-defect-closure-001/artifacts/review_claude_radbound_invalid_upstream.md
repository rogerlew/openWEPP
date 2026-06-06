# Claude Code Review — WBVAL02 Radiation-Bound Invalid-Upstream Closure

Reviewer: Claude Code
Date (UTC): 2026-06-06
Evidence mode: **Static** — read the package, the attribution/validation ledgers,
the disposition, `SC-CLIMATE-001`'s radiation REFs, and verified the production
change landed via `git show --stat 57eed35` plus `grep` of the forcing module. I
did **not** re-run `openwepp-cli-hill` or the test suite; the `cargo` results and
the six-hillslope reruns are Codex's, attributed.

Verdict: **APPROVE — an exemplary first DC-ExecPlan.** The "invalid upstream
input" verdict is genuinely proven, not a punt; the package conformed to the
ADR-0018 shape; and it landed a real contract-first change without loosening the
guard or compensating downstream.

---

## F1 — Conformance to the DC-ExecPlan shape (positive)

This is what ADR-0018 intended on its first outing. The package declared a
Correction Authority Envelope, ran the seven-gate bar, reproduced the failure,
named the mechanism, checked authority, and closed at a **declared branch
boundary** ("if the source `.cli`/CLIGEN daily radiation record is physically
invalid … close with typed upstream invalid-input evidence and a new defect
target"). The handoff is **defect-shaped** — it names `WBVAL04-CLIMATE-RADLY-
RAMAX-INPUT-BOUNDARY` with all fields, not "inspect the next radiation variable."
No grind: it closed in one pass. This is the anti-pattern's opposite.

## F2 — The science is sound; one load-bearing assumption named

The six hillslopes share one DRIGGS climate file whose row `1990-02-18`
(DOY 49) carries `radly = 486 Ly d⁻¹`, while the baseline `sunmap` horizontal
daily potential `r3 = 453.07 Ly d⁻¹` (ratio 1.073). Surface solar radiation above
the horizontal potential is physically impossible, so the input is genuinely
invalid — this is a known CLIGEN station-parameter data-quality artifact, not an
openWEPP synthesis defect.

The verdict rests on one load-bearing fact, which I checked against
`SC-CLIMATE-001` REF-CLIMATE-WF-RADLY-RADMJ: `r3`/`rpoth` is the **astronomical
(geometry-derived) horizontal potential** from `sunmap`, independent of the input
`radly`. Because that is a top-of-atmosphere-class ceiling on a horizontal
surface, `radly > r3` is strictly impossible and the rejection is airtight. (If
`r3` were instead a transmissivity-attenuated *clear-sky surface* estimate, a 7%
exceedance could be physically real and the rejection would be over-strict — it
is not, but naming it makes the verdict's basis explicit.)

## F3 — The production change is real and correctly scoped (verified)

Despite the misleading `docs:` commit subject, `57eed35` landed a genuine
contract-first change: `06_simimpl28_hourly_forcing.rs` now enforces
`0 <= radly <= sunmap horizontal daily potential (rpoth/r3)`, `08_tests.rs` adds
the red/green coverage, and `SC-CLIMATE-001` is amended to require fail-closed at
the source symbol `radly`. This **moves the fail-closed point earlier and more
precisely** — from a geometry-dependent hourly bound to the invalid daily source
— without clipping, scaling, or canonicalizing the impossible value, and without
any downstream snow/ET/percolation/WAT compensation. That is the correct outcome:
the six runs *should* fail; making them pass would require inventing radiation.

## F4 — Commit message understates the change (minor)

`57eed35` is subject-lined `docs:` but changes three non-doc files
(`06_simimpl28_hourly_forcing.rs`, `08_tests.rs`, `SC-CLIMATE-001.md`). A reader
scanning `git log` would mistake a contract+kernel+test change for documentation.
Recommend conventional-commit accuracy on landed DC-ExecPlan corrections so the
production-edit history stays legible.

## F5 — Loose end surfaced (ours, not WBVAL02's)

The disposition honestly records that `cargo test --workspace` fails outside
WBVAL02 "at an ADR0017 decisions README assertion." That is a regression from the
**ADR-0018/0017 `decisions/README.md` edits made during the doc work** (the 0018
row and the 0017 "operationalized by 0018" status cell broke a test pinning the
decisions index). It is correctly out of WBVAL02's envelope, but it is now a red
workspace gate that needs an owner — a small defect target to update the
decisions-index assertion to the post-ADR-0018 state.

---

## Disposition boundary

Findings + evidence per the review model. F1–F3 are confirmations with no
requested change. F4 is a commit-hygiene note. F5 is an out-of-envelope loose end
needing a separate owner. The `WBVAL04` handoff is well-formed; see my WBVAL03
review for why WBVAL04 is now on the critical path for all of rung-1 on this run,
and for the one sharpening its acceptance needs.
