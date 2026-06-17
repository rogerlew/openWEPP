# PERFIDX04 Determinism Evidence

Static:
- The change does not reorder floating-point reductions.
- The scheduler phase order, OFE lane order, and writeback accept/reject/apply order are unchanged.
- Indexed mirrors are non-authoritative read mirrors; the logical `BTreeMap` writeback surface remains the public compatibility and commit surface.

Ran:
- Full anchor identity held across repeated final current runs against the pre-PERFIDX04 baseline.
- Pass Parquet container bytes churned, but logical rows were identical in both `EXCEPT ALL` directions.
- `git diff --check` passed.

Static:
- Remaining nondeterminism is the known pass Parquet container byte churn already accepted by PERFIDX03B; it is not a value-level difference.
