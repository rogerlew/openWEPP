# Control-Envelope Review A

Evidence class: Static and focused runs.

Initial verdict: FAIL, not HOLD.

- HIGH `TGCC-A-01`: generic control JSON reading followed symlinks. Accepted;
  control bytes now use the confined regular-file reader before strict parsing,
  with a symlink regression.
- HIGH `TGCC-A-02`: validation and publication independently read report bytes.
  Accepted; publication now obtains and atomically writes the exact buffer
  returned by fresh control/digest validation, with a post-validation tamper
  regression.
- MEDIUM `TGCC-A-03`: package line-count evidence omitted the touched 2,000-line
  WARN file. Accepted; package now records the exact 2,611-line count, bounded
  rationale, below-3,000 disposition, and owned follow-on split intent.
- LOW `TGCC-A-03R`: first amended count was stale after finding patches.
  Accepted and corrected from 2,580 to 2,611.

Final verdict: PASS, no remaining finding.

Reviewer A reran the focused artifact regression (1/1 PASS), formatting, direct
adapter SHA checks, and diff hygiene during review. No broad suite ran.
