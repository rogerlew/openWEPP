# CQR05 Disposition

Evidence: Static + Ran.

Status: complete-with-warnings.

Completed:

- Behavior-preserving EROD14 Wave-2 private-helper extraction.
- Public API parity.
- Target maximum CRAP reduced from `587.5911363349628` to `23.0`.
- Target `too_many_lines` suppression removed.
- Focused tests, workspace tests, clippy, fmt, and dependency/license gate all
  passed.

Warnings:

- Target-file coverage improved but remains below science-tier thresholds:
  line `79.59183673469387%`, region `82.99737072743207%`.

Follow-on candidate:

- A future module-test-enhancement package can target unexercised EROD14 guard
  and rare reproportioning branches to close the ADR-0021 coverage threshold.
