# Kickoff

Execute SNOWFROST-FIDELITY-D end to end. Publish modeled snow depth as a WAT
diagnostic from `snow.runtime_depth_m`, consume it in the observed harness, and
rerun A classification across all five pilot sites.

Do not change snow/frost physics, constants, Qwet, SFCC/frozen-K models,
heat-flow equations, or thresholds. If closure cannot be reached without those
edits, stop at the package HOLD boundary with concrete evidence.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only publication-review, science-review, and
verification subagents for WAT publication lineage review, snow-control
classification legitimacy review, source-scan review, and final evidence
review. Expected outputs are compact findings summarized into
`artifacts/review-disposition.md` and `artifacts/verification.md`; subagents
may not edit files.
