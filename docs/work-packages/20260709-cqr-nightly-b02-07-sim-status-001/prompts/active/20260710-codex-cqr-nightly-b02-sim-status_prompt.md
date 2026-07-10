# CQR Nightly Batch 02 Target 07 Kickoff

Scope: local behavior-preserving CQR inside `/home/workdir/openWEPP`. Execution
mode: package-end-to-end. Read applicable guidance, CQR/ADR docs, target, and
tests; reading map `artifacts/required-reading-map.md`, budget `~50 KiB`, `OK`.

Characterize all public status mappings before private decomposition. Preserve
strings, classification/severity, typed status guard, error text, and API.
Delegate heavy metrics/workspace gates and use bounded review/verification.

Subagent requirement: REQUIRED: use a comparator suite runner for heavy
coverage/CRAP and workspace fmt/clippy/full-nextest/deny. Review and
verification agents are read-only unless explicitly given a bounded target path.

Autonomy: execute through disposition unless hard-blocked.
