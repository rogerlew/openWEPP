# Gate Results

Status: `execution and reconstruction PASS / scientific disposition HOLD`.

Evidence mode: `Ran`.

| Gate | Result | Evidence |
| --- | --- | --- |
| Exact result-blind v3 admission | `PASS` | Independent science/Rust/consumer `PASS/PASS/PASS` at `5ebfc5135`. |
| Four-site 12-lane execution | `PASS` | Release binary completed control, same-state, and sequential lanes at all sites. |
| Retained replay verification | `PASS` | `--verify-existing`; `143/143` retained artifacts. |
| Reconstruction and endpoint closure | `PASS` | All primitive, total, mass, cold, continuity, join, and support guards. |
| Protected production outputs | `PASS` | HBP/PASS/WAT exact at all four sites; CoE authority unchanged. |
| Frozen scientific classifier | `PASS` | Emitted `PREDECESSOR_NOT_REPRODUCED` and projection difference without overclaim. |
| Assurance source adoption | `PASS` | Typed transaction `31798778...`; DRAFT preserved, no authority invalidated. |
| Critical terminal validation | `PENDING` | Exact closure-head heavy gate to be appended. |
| Dual terminal verification | `PENDING` | Exact closure-head independent verification to be appended. |

Scientific reconciliation remains `HOLD` because the exact predecessor bridge
fails. This does not invalidate the package's completed operator-mechanics
characterization.

The first critical closure attempt at exact clean `597aebd0e` stopped before
expensive profiles because workspace warnings-denied Clippy found a 115-line
contract-test function above its 100-line limit. The assertions were split into
two named helpers without changing content. Workspace Clippy, focused
contract/observability `13/13`, consumer `51/51`, formatting, and diff hygiene
then passed locally. A renewed full critical run is required; the failed attempt
is not counted as closure evidence.

The second attempt at exact clean `50175c6f3` passed formatting, affected and
workspace Clippy, doctests, consumer `51/51`, meteorology `25/25`, orchestrator
Stage 3 `9/9`, runner Stage 3 `11/11`, contract/observability `13/13`, Binding
Exposure `11/11`, and assurance validation/plan. It then stopped on the
composite assurance export guard because that historical guard counted the
later governed `usersum/assurance/review-drafts/**` lane as public. The package
prospectively admitted a narrow guard correction. The guard now excludes only
that named nonpublic subtree while retaining the sole-public-README,
zero-document, vendoring-disabled, validation, check, and transition-preflight
requirements. Direct guard execution and deterministic review-draft drift
checking pass. A renewed complete critical run remains required.
