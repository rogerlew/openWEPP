# CQR17 Implementation and Test Evidence

Status: closed.

Static: implementation changed only the CQR17 target helper family in
`hydrology_phase_erod19.rs`.

Implementation summary:

- Added private `Erod19XcritInputs` and `Erod19XcritResult` structs.
- Replaced the monolithic `erod19_xcrit_classification` decision tree with
  private helpers for linear, rising, curved, curved-root, and root-selection
  cases.
- Removed the target function's
  `#[allow(clippy::similar_names, clippy::too_many_lines)]` suppression.
- Preserved the existing public crate-visible function signature and tuple
  output.

Static: focused characterization added in
`crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/hydrology.rs`:

```text
cqr17_erod19_xcrit_classification_preserves_branch_vectors
```

The characterization covers ten branch vectors:

- linear increasing critical point inside segment
- linear decreasing critical point inside segment
- convex rising all above critical shear
- convex rising all below critical shear
- convex rising crosses critical shear
- curved segment remains above critical shear
- curved segment has no real critical crossing
- curved segment crosses from below to above critical shear
- curved segment crosses from above to below critical shear
- curved segment has two critical crossings

Ran: focused characterization before production refactor: exit code `0`.

Ran: focused characterization after production refactor: exit code `0`.

Ran: early touched-crate clippy:

```text
cargo clippy -p openwepp-hillslope-orchestrator --all-targets -- -D warnings
```

Result: exit code `0`.
