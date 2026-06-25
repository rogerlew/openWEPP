# Review A: Harness and Route Policy

Evidence mode: Static + Ran.

Reviewer: local Codex review pass A.

Scope:

- `tools/snowfreeze_observed/pysnobal_compare.py`;
- G1 PySnobal site-sane and failed-lane artifacts;
- package correction envelope.

Findings:

1. accepted: the initial reuse run failed because `openwepp_snow.csv` used WAT
   simulation-calendar year/month/day values such as `0001-02-29`. The fix maps
   WAT `sim_day_index` to the actual climate dates already used by the PySnobal
   forcing, and adds a Rust unit test for the leap-day case.

2. accepted: strict all-lanes gating must remain available because it is useful
   for sensitivity-lane sanity. The harness keeps `all-lanes` as the default and
   requires explicit `--route-policy site-sane` to route around failed
   non-selected sensitivity lanes.

3. accepted: output reuse could hide stale or corrupt PySnobal outputs if it
   simply trusted existing CSVs. The implementation revalidates finite,
   nonnegative SWE/depth, positive snow response, and 700 kg/m3 density ceiling
   before using reused output in a summary.

4. rejected: make `site-sane` the default route. Rationale: that would weaken
   G0's strict sanity posture and could hide sensitivity-lane failures in future
   all-lane runs. Keeping strict default plus explicit `site-sane` is safer.

Disposition: no undispositioned findings.
