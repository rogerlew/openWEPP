# WS10 Kickoff Agent Prompt

You are executing
`20260523-ws10-channel-impoundment-production-kernels-001` for the monolithic
openWEPP scientific hydrology/erosion model.

Objectives:
1. Implement production watershed channel/impoundment kernel behavior under
   typed boundary integration.
2. Implement required canonical WS10 contract amendments and contract-derived
   tests.
3. Produce routing/impoundment contract evidence and production
   `WatershedKernel` path evidence.
4. Preserve ARCH15/ARCH21 typed-seam non-regression posture.

Mandatory sequencing constraints:
- Do not modify production WS10 kernel code until:
  1. contract amendments are implemented, and
  2. contract-derived tests are implemented, and
  3. pre-implementation contract-gate evidence is recorded.
- Contract tests must be authored from canonical SC authority, not from current
  implementation behavior.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Do not introduce silent defaults/clamping for routing/impoundment boundary
  domain violations, missing required symbols, or non-finite values.

Required outputs are listed in `package.md` Deliverables.
