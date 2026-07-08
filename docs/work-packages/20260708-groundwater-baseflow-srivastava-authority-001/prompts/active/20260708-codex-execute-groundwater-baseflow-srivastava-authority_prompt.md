# Execute Srivastava Groundwater/Baseflow Authority

Scope: local repository science-contract work; flat-file reads/edits only; no
external connectivity required.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260708-groundwater-baseflow-srivastava-authority-001/package.md`
sequentially through disposition.

Required reading:

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/work-packages/20260708-groundwater-baseflow-srivastava-authority-001/package.md`
- `docs/work-packages/20260708-groundwater-baseflow-srivastava-authority-001/artifacts/required-reading-map.md`
- `references/annotated_bibliography.md` entries R-21, R-22, R-22A, and R-70
- `docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md`
- `docs/specifications/wepp-input-files/specs/gwcoeff.spec.md`

Authority sources:

- `/workdir/wepp-forest/references/Srivastava_Diss2013_14.pdf`
- `references/copyrighted/Srivastava2013.pdf`
- `references/copyrighted/Srivastava2017_ToASABE_wepp_streamflow.pdf`
- `references/copyrighted/dun2009.pdf`
- `/workdir/wepp-forest_260430_baseline/src/main.for`
- `/workdir/wepp-forest_260430_baseline/src/contin.for`
- `/workdir/wepp-forest_260430_baseline/src/wshpas.for`
- `/workdir/wepp-forest_260430_baseline/src/wshdrv.for`
- `/workdir/wepp-forest_260430_baseline/src/wshchr.for`
- `/workdir/wepp-forest_260430_baseline/src/wshcqi.for`
- `/workdir/wepp-forest_260430_baseline/src/watbalprint.for`

Conditional:

- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md`

Required-reading budget: `8,920,167` bytes,
`REQUIRES-JUSTIFICATION`; map:
`artifacts/required-reading-map.md`. Justification: this is a contract-first
authority package requiring primary/companion PDFs plus baseline Fortran code;
use targeted PDF extraction and do not copy long copyrighted passages.

Files:

- `docs/work-packages/20260708-groundwater-baseflow-srivastava-authority-001/**`
- `docs/specifications/science-contracts/contracts/SC-GWBASEFLOW-001.md` or
  justified alternate contract target.
- `docs/specifications/science-contracts/index.md`
- Cross-contract files only as declared in `package.md`.

Task: execute the M-T2A objective end-to-end. Establish canonical
groundwater/baseflow process authority from Srivastava literature plus
`/workdir/wepp-forest_260430_baseline` code authority, then hand off M-T2B.

Constraints: contract-first sequencing; canonical `SC-*` authority; baseline
provenance from `/workdir/wepp-forest_260430_baseline` at
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; typed guards; no silent defaults;
no canonicalize-and-proceed for domain violations.

No surrogate physics: production code must implement actual contract-backed or
baseline-authoritative physics in later implementation packages. Surrogate,
provisional, proxy, empirical stand-in, or heuristic production formulas are
forbidden. Missing authority is a hold-for-authority boundary.

Terminology guard: keep `latqcc` lateral subsurface export,
groundwater-reservoir baseflow, deep seepage, and `chan.inp` `cbase` separate.

Subagent requirement: REQUIRED for read-only review and verification if
available. This prompt explicitly authorizes subagent spawning/delegation to
review and verification roles for authority/code-map checking; outputs:
package-local `artifacts/review-*.md` and `artifacts/verification-*.md`; write
access: bounded to package artifacts.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts, contract authority if closed, disposition,
and M-T2B worker handoff.
