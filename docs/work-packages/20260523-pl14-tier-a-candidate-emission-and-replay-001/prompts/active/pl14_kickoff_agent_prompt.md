# PL14 Kickoff Agent Prompt

You are executing `20260523-pl14-tier-a-candidate-emission-and-replay-001` for
the monolithic openWEPP scientific hydrology/erosion model.

Objectives:
1. Execute strict Tier-A comparator replay using direct openWEPP candidate
   output versus pinned legacy baseline.
2. Implement required canonical PL14 contract amendments for replay authority
   and guard behavior.
3. Implement contract-derived PL14 tests and run pre-implementation gate
   evidence before production replay/harness code edits.
4. Persist comparator JSON artifacts, command traces, and provenance hashes for
   reproducibility.

Mandatory sequencing constraints:
- Do not modify replay/harness production code until:
  1. contract amendments are implemented, and
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/clamping for comparator failures,
  provenance gaps, or missing required replay artifacts.

Required outputs are listed in `package.md` Deliverables.
