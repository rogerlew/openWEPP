# Kernel-Profile Compliance Checklist

Status: queued

Evidence mode: not-run

Static:
- Queued artifact for checking HPHYS0247 against
  `docs/specifications/science-contracts/kernel-process-contract-profile.md`.

Ran:
- Not run.

Checklist:
- [ ] Canonical `SC-*` authority is amended before production code edits.
- [ ] Contract-derived tests are added before production code edits.
- [ ] Pre-implementation contract gate is recorded.
- [ ] Pinned legacy provenance is cited for equations, constants, guards, and
  invariants.
- [ ] Runtime aliases preserve legacy WEPP symbol continuity.
- [ ] Typed guards reject missing, non-finite, or out-of-domain required
  surfaces.
- [ ] No heuristic/proxy process-physics substitutions are introduced.
- [ ] Disposition remains `HOLD` if review, verification, or profile
  requirements are incomplete.
