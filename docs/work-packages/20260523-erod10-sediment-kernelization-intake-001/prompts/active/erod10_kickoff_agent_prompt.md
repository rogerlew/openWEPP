# EROD10 Kickoff Agent Prompt

You are executing
`20260523-erod10-sediment-kernelization-intake-001` for the monolithic
openWEPP scientific hydrology/erosion model.

Objectives:
1. Convert `KERNEL-GAP-010` deferral into an executable sediment-kernelization
   intake and phase plan.
2. Publish explicit dependency graph and wave ownership/gates for follow-on
   implementation packages.
3. Publish contract-authority mapping for erosion-lane follow-ons.
4. Preserve kernel governance posture (procedure/profile + truthfulness labels).

Constraints:
- This package is intake/planning scoped; do not implement production erosion
  kernel code in EROD10.
- Enforce kernel profile consistency using
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.
- Use truthfulness posture (`Static:` vs `Ran:`) in all artifacts.
- Correctness over completion: unresolved high-severity authority/ownership
  ambiguity remains `HOLD`.

Required outputs are listed in `package.md` Deliverables.
