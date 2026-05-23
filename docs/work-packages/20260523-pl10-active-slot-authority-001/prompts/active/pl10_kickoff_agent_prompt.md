# PL10 Kickoff Agent Prompt

You are executing `20260523-pl10-active-slot-authority-001`.

Objectives:
1. Remove first-slot/crop placeholder authority from PL growth/decomposition
   transition dispatch paths.
2. Implement deterministic day-aware active slot/crop resolution per OFE.
3. Add typed failure behavior for invalid/ambiguous active-slot conditions.
4. Deliver integration evidence for multi-slot and rotation-boundary cases.

Constraints:
- Preserve ordering invariants and existing typed guard posture.
- Do not implement PL11+ event payload expansion or process kinetics.
- Maintain typed-seam non-regression posture per ARCH15/ARCH21 evidence.

Required outputs are listed in `package.md` Deliverables.
