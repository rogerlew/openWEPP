# Execute Canonical Hourly Lane D Routing-Coefficient Projection Authority

Scope: local repository science-contract/kernel authority task; flat-file
reads/edits only; no external connectivity required.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001/package.md`
sequentially through disposition.

Required reading:

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001/package.md`
- `docs/work-packages/20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001/artifacts/required-reading-map.md`
- `docs/ROADMAP.md` `## Watershed Runtime Performance Queue`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- `docs/work-packages/20260708-laned-router-conditional-default-activation-001/package.md`
- `docs/work-packages/20260708-plant-file-native-lanuse-routing-doc-001/package.md`

Conditional:

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md`
- `docs/specifications/science-contracts/index.md`
- `/workdir/wepp-forest_260430_baseline/src/frcfac.for`
- `/workdir/wepp-forest_260430_baseline/src/param.for`
- `/workdir/wepp-forest_260430_baseline/src/bigout.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`

Required-reading budget: `442,168` bytes, `WARN`; map:
`artifacts/required-reading-map.md`.

Files:

- `docs/work-packages/20260708-laned-router-canonical-hourly-laned-routing-coeff-projection-authority-001/**`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/wepp-input-files/specs/plant-file.spec.md`
- Conditional contract files only as justified in `package.md`.

Task: decide the routing-coefficient projection authority and canonical hourly
Lane D production-path policy end-to-end.

Constraints: contract-first sequencing; canonical `SC-*` authority; baseline
provenance from `/workdir/wepp-forest_260430_baseline` at
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; typed guards; no silent defaults;
no canonicalize-and-proceed for domain violations.

No surrogate physics: do not invent unbounded production coefficients. A
projection may be accepted only if contract-ratified, source-mapped,
bounded, deterministic, and fidelity-adequate under a predeclared envelope.
Missing authority is a hold-for-authority boundary.

Canonical path posture: evaluate hourly water balance plus Lane D active
routing as the production path for both single-OFE and MOFE. Retain non-hourly,
DC01-only, and non-Lane-D paths only for validation, comparator, rollback, and
regression diagnosis unless contract authority says otherwise.

Subagent requirement: REQUIRED for read-only review and verification if
available. This prompt explicitly authorizes subagent spawning/delegation to
review, verification, and comparator-design roles for authority checking;
outputs: package-local `artifacts/review-*.md`,
`artifacts/verification-*.md`, and optional
`artifacts/comparator-design-*.md`; write access: bounded to package artifacts.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts, contract/spec authority if closed,
disposition, and implementation handoff.
