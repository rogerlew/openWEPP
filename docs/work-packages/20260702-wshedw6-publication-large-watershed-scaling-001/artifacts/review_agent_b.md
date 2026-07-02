# Review Agent B

Status: `completed-with-findings`

Evidence mode: `Static:` QA review plus `Ran:` `cargo fmt --check`,
`git diff --check`, fixture manifest validation, and scaling JSON inspection.

Reviewer: `rust_qa_reviewer`

## Findings

| Severity | Finding | Disposition |
| --- | --- | --- |
| High | Final closure gate table was still queued/not-run while other artifacts claimed progress. | Accepted. `gate-results.md` is updated only after post-fix command execution completes. |
| High | Required review/verification artifacts were placeholders while `review-disposition.md` claimed passed. | Accepted. Review artifacts now contain the reviewer findings and dispositions; verification artifacts are updated after final gates. |
| Medium | Conservation reconstruction relied too much on produced-output self-consistency. | Accepted and fixed. `conservation-reconstruction.md` now records an independent Python audit that parsed every committed `pN.source.run` and `pN.slp` to reconstruct source geometry area before comparing produced area, `Runoff`, `Q`, and `runvol`. |

## Non-Blocking Debt

- Line-count artifact was stale after test helper extraction; updated current
  counts.
- The output crate now depends on the orchestrator for
  `WatershedPublicationFrame`. A future split can extract a shared publication
  DTO if writer/orchestrator coupling grows.
