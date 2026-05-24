# PL15R Kickoff Agent Prompt

You are executing `20260523-pl15r-tier-a-delta-recloseout-and-hold-lift-rerun-001`
for the monolithic openWEPP scientific hydrology/erosion model.

Objectives:
1. Classify residual Tier-A deltas from PL14R rerun evidence and issue refreshed
   PL08 hold-lift verdict (`lift` or `retain hold`).
2. Implement required canonical PL15R contract/spec amendments for decision
   authority and governance guard behavior.
3. Implement contract-derived PL15R tests and run pre-implementation gate
   evidence before production closeout logic or decision-surface code edits.
4. Record explicit risk-acceptance reference outcome when unresolved Tier-A
   blockers remain.

Mandatory sequencing constraints:
- Do not modify production closeout logic or decision-surface code until:
  1. contract amendments are implemented, and
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent down-classification of Tier-A blockers or implicit
  risk-acceptance; all exceptions must have explicit approval references.

Required outputs are listed in `package.md` Deliverables.
