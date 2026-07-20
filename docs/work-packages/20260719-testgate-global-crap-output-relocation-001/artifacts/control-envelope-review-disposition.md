# Control-Envelope Review Disposition

All findings from both required reviewers are accepted and closed.

The final correction preserves floating-point CRAP metrics while keeping the
control plane integer-only and duplicate-key-safe. Both control and report use
confined regular-file reads; publication revalidates the pair and atomically
writes the exact returned report buffer. Focused regressions prove symlink
rejection, post-validation tamper rejection, float-control rejection,
non-PASS/nonzero rejection, digest binding, and per-attempt cleanup.

The touched 2,611-line executor crosses the 2,000-line WARN but not the 3,000
hard block. Decomposition during this security correction would broaden the
write and review surface; a behavior-preserving artifact-helper extraction is
owned as follow-on intent after package and adversarial acceptance closure.

Final independent verdicts: Reviewer A PASS; Reviewer B PASS. A fresh committed
terminal plan is authorized. The preserved prior plan remains stale and must
not resume.
