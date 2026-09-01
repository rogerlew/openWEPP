# R105 rolling exact transition reset implementation and validation

Status: `IMPLEMENTED_FOCUSED_GREEN_CANONICAL_R106_PENDING`

Implemented:

- `phase_consistent_coupled_active_set_transition_window_v1` retains the
  unchanged exact V33 reset predicate.
- An exact reset dispatches without altering the retained root or branch-entry
  evidence.
- A nonexact reset never dispatches. It promotes the current already-validated
  interface to the next root anchor and clears branch-entry state, so the next
  complete window is evaluated against the fresh anchor rather than the stale
  first interface.
- V34 stable-monotone eligibility, the shared 96-evaluation budget, V35 receipt
  stabilization, V38 finalization-equivalent map, V39 custody, authentic-only
  replay/finalization, rollback, and publication guards are unchanged.

Ran:

- Contract-first source-bound expected red: run
  `a7d1aba9-65d2-41a9-b878-031544e42ad9`, 1 failed only for the two absent
  rolling-window behavior obligations.
- Focused reset/window tests: run
  `0d67cc51-2e05-4beb-8594-e4b6af6afc86`, 4 passed.
- Corrective V33 source-bound obligation: run
  `bd69e86c-9d8f-410b-baa3-1b6ee166a840`, 1 passed.
- `cargo check -p openwepp-hillslope-orchestrator --all-targets`: passed.
- `cargo fmt --all -- --check`: passed.
- Owned-path `git diff --check`: passed.
- Bounded diagnostic scan: no `DFF_V*`, `eprintln!`, or `dbg!` remains in the
  rolling-window production/test write set.

Canonical r106 remains root-owned and pending.
