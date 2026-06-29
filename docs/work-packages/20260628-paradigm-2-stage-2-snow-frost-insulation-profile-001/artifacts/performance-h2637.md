# H2637 Performance Evidence

Status: `NOT-PROMOTION-DECISIVE`

ADR-0025 requires H2637 performance evidence for the opt-in candidate.

The package did not reach a promotion decision because the primary frost
observation-corpus gate failed first:

- bulk handoff: `3` robust fails / `49` score;
- layered resistance: `3` robust fails / `49` score;
- primary robust improvements: `0`.

Ran performance-relevant real corpus work:

- Command: `.venv/bin/python tools/snowfreeze_observed/paradigm2_stage2_insulation_profile.py --mode frost --hill-binary target/debug/openwepp-cli-hill`
- Scope: five frost-observation sites x two model arms.
- Elapsed: `319.1788842184469 s`.

Disposition: no ADR-0025 activation/performance claim is made. H2637 promotion
evidence remains required if a later Stage 2 variant improves the frost-primary
rubric.
