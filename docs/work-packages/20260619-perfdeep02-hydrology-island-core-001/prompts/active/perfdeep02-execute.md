# PERFDEEP02 Execute

Execute `PERFDEEP02` under ADR-0025. Start by closing the carried
PERFDEEP01 real-surface frame round-trip gate. Then migrate only real
hydrology compute work onto the frame; do not reclassify writeback-only or
materialization-only changes as Stage-1 completion.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only review and verification subagents for package
artifacts, code diffs, and gate evidence. Expected outputs are review and
verification artifact drafts under this package. Write access is bounded to
this package's `artifacts/` directory.
