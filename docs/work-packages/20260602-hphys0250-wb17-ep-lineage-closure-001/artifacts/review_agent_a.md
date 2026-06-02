# Review Agent A

Status: complete

Evidence mode: static

Static:

- Reviewer: Rust code-review agent (`Dewey`).
- Scope: HPHYS0250 Ep lineage contract/code review.
- Ran: no suites rerun by reviewer; read-only review against reported evidence.

Findings:

## High

- None.

## Medium

1. Duplicated PL active-slot logic can diverge between runner sentinel
   preservation and scheduler slot resolution.
   - Disposition: accepted as residual/continuation risk, not changed in this
     package. The runner check exists only to decide whether to preserve or
     remove the activation sentinel before scheduler ownership starts; current
     zero-date perennial behavior is covered by runner and scheduler tests.
     Next package should centralize or cross-check this seam if PL scope expands
     beyond single-OFE primary-slot execution.
2. Established-perennial initial assimilation publishes `rtd = rdmax` without a
   management-projection soil-depth cap.
   - Disposition: accepted as residual/continuation risk. Management projection
     does not own soil-depth surfaces; the growth transition caps `rtd` by
     `min(rdmax, solthk)` once the merged runtime surface is available before
     ET/root uptake. HPHYS0250 records this as an initial-state/projection seam
     to revisit when baseline-authoritative initial-state migration expands
     beyond H1-style activation.

## Low

- Package artifacts initially still showed queued/not-run state.
  - Disposition: fixed. Package status, artifact README, disposition, gate
    evidence, metrics, and review artifacts were updated to `HOLD` / static+ran
    evidence.

Residual risk:

- No blocking implementation correctness issue found for WB15 near-zero
  canonicalization, zero-date perennial activation, final WB13 flux-preferred
  `Ep`, or growth/decomposition writeback.
- Keep HPHYS0250 `HOLD`; continuation should focus on `swu.for` uptake/stress
  magnitude and the PL active-window/initial-root-depth seams above.
