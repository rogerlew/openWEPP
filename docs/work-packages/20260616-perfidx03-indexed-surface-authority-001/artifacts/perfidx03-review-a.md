# PERFIDX03 Review A

Status: HOLD 2026-06-17
Evidence mode: **Static** + **Ran**

This is a primary-agent local review artifact, not an independently delegated
subagent review.

## Findings

- **Accepted blocker:** the production authority flip cannot close. The attempted
  active indexed-authority path exported the sparse authority back to full
  `BTreeMap` surfaces for the existing kernel seam, causing OFE5 to regress from
  about `27s` to about `38s`.
- **Accepted mitigation:** production activation is disabled in the final tree.
  `refresh_indexed_writeback_authority` preserves an already-active authority but
  does not implicitly turn it on.
- **Accepted precondition fix:** the reachable registry now covers frost terminal
  fine layers and irrigation depletion/fixed-date sidecars. The final five-case
  audit has zero unknowns.
- **Residual required gate:** full H2637 both-UI plus OFE1-OFE5 anchor
  bit-identity was not run after the speed gate failed. That is correct for a
  HOLD, but it prevents PASS.

## Review Result

HOLD is the correct disposition. Do not mark PERFIDX03 complete until the kernel
seam can avoid the full map export cost while preserving output identity.
