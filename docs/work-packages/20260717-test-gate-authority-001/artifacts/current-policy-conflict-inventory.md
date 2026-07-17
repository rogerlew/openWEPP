# Current Policy Conflict Inventory

Evidence class: `Static`

## Finding

No single document previously governed test selection and gate timing. The
binding surfaces evolved independently and now encode incompatible lifecycle
assumptions.

| Surface | Current rule | Conflict or cost |
| --- | --- | --- |
| `AGENTS.md` validation gates | Full workspace Clippy, Nextest, cargo-deny, comparators, and fresh CRAP before kernel completion | Makes an isolated package pay release-scale cost |
| `docs/work-packages/AGENTS.md` | Full closure loop and fresh full-workspace CRAP for every implementation package | Overrides proportional local-CI guidance |
| `docs/standards/local-ci-gate-selection.md` | Match gate cost to risk; avoid reflexive full reruns | Cannot waive the stronger package rules |
| ADR-0021 Decision 5 | Coverage thresholds are not a global per-PR gate | Decision 8 nevertheless makes global CRAP coverage an every-package gate |
| `.github/workflows/release-gates.yml` | Runs the release-gate workflow on pull request, push, weekly schedule, and manual dispatch | Ordinary CI and release qualification share one wrapper |
| `run_release_candidate_gates.sh` | Runs full Nextest, then the adjudicated CRAP runner | The CRAP runner launches another workspace test execution under coverage |
| Assurance lifecycle | Every material software-realization change requires impact disposition | No campaign queue distinguishes historical report validity from new-head currency |

## Retained Timing Evidence

The final canopy-phenology package recorded full Nextest at 2,085 tests and
559.630 seconds. The retained local-CI baseline recorded 1,333 tests with
1,125.187 aggregate test seconds before later suite growth. Several unrelated
snowbench tests individually require roughly one to two minutes. Fresh CRAP
then cleans coverage artifacts and executes the workspace again.

The problem is therefore architectural rather than merely scheduler tuning:
the same broad evidence is acquired at package, CI, campaign, and release
boundaries without distinguishing their purposes.

## Authority Resolution

ADR-0039 and `docs/standards/testing-and-gate-strategy.md` now own the lifecycle
question. Existing documents retain authority over their specialized subjects:

- science contracts define correct process behavior;
- the correctness-authority model ranks evidence and external suites;
- the Rust standard defines test obligations and authoring mechanics;
- ADR-0021 defines coverage/CRAP quality thresholds and exception taxonomy;
- assurance governance defines scientific review and publication authority.

The implementation follow-up must replace duplicated gate-frequency text with
short pointers to the new standard.
