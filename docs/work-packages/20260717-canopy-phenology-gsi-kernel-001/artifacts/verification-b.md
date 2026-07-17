# Terminal Verification B

Evidence class: `Static`, `Ran`, and retained `Ran`

Disposition: `HOLD`

Verification subject: fully dispositioned current working tree based on frozen
base `45d49090214b4702d11a04aafe5d5ccade7ba440`. Verifier B read the initial
Review B and producer disposition but did not read Verifier A's output. The
expensive workspace coverage workflow was not rerun.

## Checks performed

- Inspected `artifacts/review-b.md`, `disposition.md`, `focused-gates.md`, and
  `heavy-gates.md` against terminal-current source.
- Inspected the final `SC-PLANT-001` GSI invariants, Guard Map, Symbol Alias Map,
  cold-start/chronology policy, vectors, integration hold, and revision record.
- Inspected the complete `openwepp-plant-phenology` public API, state mutation,
  restoration, errors, and 12 tests.
- Confirmed source search and reverse dependency inspection find no production
  consumer of the crate; integration remains outside this package.
- Ran `cargo fmt --check`, strict package Clippy, package Nextest quick profile
  (12/12, run `0898e15e-33b5-4088-91c8-7cc9bd689e91`), the roadmap/backlog/
  contract/package Markdown lints, and `git diff --check`; all passed.
- Reconfirmed line counts of 913 for the Rust source and 854 for the contract.
- Recomputed the current Rust source SHA-256 as
  `94d79dd78324a2e546bda0b753ed953b03353eda0e3588ad29748f4dd5c72b4d`;
  it exactly matches the entry in
  `target/adjudicated-crap/source-manifest-final.json`.

## Review B closure audit

| Finding | Verification | Evidence |
| --- | --- | --- |
| `B-01` traceability | `CLOSED` | `SC-PLANT-001.md:473-477` supplies guards for `INV-PLANT-028..032`; lines 531-541 map forcing, parameters, indicators, result, FIFO, and newest date to the Rust API. |
| `B-02` warm-up authority | `CLOSED` | `INV-PLANT-029` now separates direct and inferred evidence; `SC-PLANT-001.md:777-789` explicitly identifies available-real-sample cold start and restart chronology as openWEPP policy rather than published Jolly behavior. |
| `B-03` chronology/restart | `PARTIAL / HOLD` | `GsiDate`, anchored `GsiState`, consecutive admission, typed failures, and rollover behavior are implemented coherently, but the required successful restoration vector is absent; see `VB-01`. |
| `B-04` contract vectors | `CLOSED` for the original product/FIFO finding | `lib.rs:543-564` exercises three nontrivial indicators and a reconstructed product. Lines 607-643 independently assert heterogeneous 20-sample, 21-sample, and post-eviction means. |
| `B-05` backlog truthfulness | `CLOSED` | The fixed-date replacement is assigned to Increment 3 at the backlog's lines 102-117; lines 461-488 separate resolved process choices from remaining integration questions. |
| `B-06` terminal evidence | `CLOSED` for the source measured | Focused and heavy artifacts exist. Retained full Nextest reports 2,084/2,084 and deny/workspace Clippy PASS. CRAP reports raw 2, adjudicated 2, actionable 0, and touched actionable 0. Current production source matches the retained source manifest. |

## Finding

### VB-01 — High — Exact successful restart is claimed but not tested

`SC-PLANT-001.md:803-804` requires history/date-anchor restoration vectors, and
the Review B disposition at `artifacts/disposition.md:15` says chronology,
restart, and tests close `B-03`. `artifacts/implementation.md:15-19` likewise
claims an exact restored-state test. However, every `GsiState::try_from_history`
call in `crates/openwepp-plant-phenology/src/lib.rs:718-762` is a rejection
vector: history too long, out-of-range history, or missing/extraneous anchor.
The replay test at lines 816-839 starts two empty states and does not exercise
restoration. No test successfully restores a valid ordered FIFO plus newest date
and proves that the next consecutive admission produces the same output and
state as uninterrupted execution.

Action: add a positive public-API restoration/continuation vector. Advance an
original state through heterogeneous consecutive forcing, reconstruct a second
state from `history()` and `last_date()`, assert exact state equality, advance
both with the same next calendar day, and assert bit-identical results and final
state. Refresh any gate and identity evidence made stale by that change before
terminal verification repeats.

## Retained CRAP identity verification

The hashes recorded in `heavy-gates.md` match the corresponding retained
artifacts:

- CRAP input JSON (`target/adjudicated-crap/workspace-crap.json`):
  `629bae489352c326b8dbccced737fce23a89c4d6858a72f28b0f6ebfbe48f07b`;
- LCOV: `f34b9ebd4cf4f2e58aa50218240945658c032b44f1c3a5f17306cf0633790cd4`;
- production source manifest:
  `08fb25e6150ee9a1d1bf4d3b1acb9e9f16003308cf8257f453a88c79b6b656f3`;
- adjudication registry:
  `10b19679e382ebacd6b2d20ee02144c461e01b1ac958731d07dd6585acb7d67f`.

The gate report is eligible, fresh, and `PASS` for its measured source. The
remaining hold is the unclosed contract-test obligation, not a CRAP or scope
failure.

## Conclusion

Equation fidelity, inferred warm-up disclosure, chronology implementation,
scope containment, product/FIFO vectors, planning truthfulness, and retained
terminal/CRAP evidence pass. `PASS-PROCESS-KERNEL` remains blocked until the
successful anchored-restoration vector is present and terminal evidence is
current for that final source.
