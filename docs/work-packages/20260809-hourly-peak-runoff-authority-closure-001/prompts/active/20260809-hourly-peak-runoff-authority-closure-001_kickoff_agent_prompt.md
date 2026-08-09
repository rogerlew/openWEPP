Scope: local repository science-contract/kernel defect-closure task; flat-file
reads/edits and local execution only; no external systems are required.

Execution mode: package-end-to-end.

Phase plan: execute all phases in `package.md` sequentially through disposition.

Required reading:

- Core: `AGENTS.md`, `docs/work-packages/AGENTS.md`,
  `docs/specifications/science-contracts/AGENTS.md`, `crates/AGENTS.md`,
  `tests/AGENTS.md`, this package, and
  `docs/backlog/20260807-canopy-peak-runoff-discontinuity.md`.
- Conditional: `docs/standards/kernel-work-package-preparation.md`,
  `docs/standards/testing-and-gate-strategy.md`, and
  `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md` before
  contract/test/production edits.
- On-demand: affected consumer contracts, runner/output specifications, and
  Topanga mutation-census manifests named in `artifacts/required-reading-map.md`.

Required-reading budget: `467459` local bytes for the initial authority set,
`REQUIRES-JUSTIFICATION`; the large canonical WATBAL contract cannot be reduced
to a sidecar because it owns WB16 and the shared hourly return/runoff lineage.
Map: `artifacts/required-reading-map.md`.

Task: close `PEAK-HOURLY-001`, `PEAK-RETURN-002`, and `PEAK-UNITS-003`; make
the existing modeled 24-bin runoff series authoritative for maximum hourly mean
peak, preserve hourly surface return, convert to volumetric flow once at public
output, and execute the complete frozen Topanga small-mutation design with
openWEPP.

Constraints: contract-first sequencing; typed guards; no silent defaults; no
legacy reproduction/parity target; no invented subhourly physics; no parameter
tuning; real downstream consumers must read the corrected path.

Conservation/output acceptance: record operand lineage, separate plausible
aliases, independently reconstruct hourly volume and public peak, test area
scaling, and align metadata/schema. Self-consistency alone cannot close.

Subagent requirement: REQUIRED. This prompt explicitly authorizes subagent
spawning/delegation to bounded source investigators, two hydrology/science
reviewers, `rust_code_reviewer`, `rust_qa_reviewer`, two terminal verifiers, and
`comparator_suite_runner` for the full cohort and heavy closure gates. Outputs:
compact findings or named package artifacts/logs; write access: read-only except
for explicitly named review/verification artifacts and generated logs.

Autonomy: execute end-to-end without user intervention unless hard-blocked.
