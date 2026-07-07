# Worker Handoff

Status: READY. Evidence mode: Static.

ADR-0037 removal is complete on main after final gates. The hybrid implicit
stepper is abandoned, archived, and stripped; do not continue optimizing or
promoting it from main.

## Next Package

Scaffold a separate Tier-2 mesh-resolution re-scope package. The surviving
question is not hybrid promotion; it is a Δx-target mesh policy for the
plain Lane D active router.

Expected scope:

- Treat H2637 as a synthetic stress case, not a fleet-general timing proof.
- Do not revive hybrid code or `SC-OFEROUTE-002`.
- Build Δx-anchored oracle/self-convergence rungs for the active plain path.
- Price mesh policy against real selected-cohort members and the synthetic
  stress case separately.
- Preserve the ADR-0037 rule that any hybrid revival starts from the archive
  branch under a new contract, not by reverting the removal.

Suggested package id:

`20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001`
