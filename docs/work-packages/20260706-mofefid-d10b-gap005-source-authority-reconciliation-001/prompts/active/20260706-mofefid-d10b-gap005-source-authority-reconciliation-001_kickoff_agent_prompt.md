# D10B Kickoff Prompt

Scope: local repository science-contract/kernel defect-closure task;
flat-file reads/edits only; no external connectivity required (the D10B-S1
acquisition targets Davis 1984 and Tseng 2010 are already in hand —
operator acquisition 2026-07-06; paths in package.md In-Scope Authority).

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in
`docs/work-packages/20260706-mofefid-d10b-gap005-source-authority-reconciliation-001/package.md`
sequentially through disposition.

Required reading (read before edits):

Core:

- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/codex_exec_plans.md`
- `/home/workdir/openWEPP/docs/defect_closure_execplans.md`
- `/home/workdir/openWEPP/docs/work-packages/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260706-mofefid-d10b-gap005-source-authority-reconciliation-001/package.md`

Conditional:

- `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md`
  before editing `SC-OFEROUTE-001` or kernel authority.
- `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
  before editing `SC-OFEROUTE-001`.
- `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
  before editing `SC-OFEROUTE-001` or kernel authority.
- `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md`
  before editing contract registry or profile-bound status.

On-demand:

- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `/home/workdir/openWEPP/docs/planning/mofe-fidelity-campaign-strategy.md`
  (§6.1) and `/home/workdir/openWEPP/docs/planning/mofe-water-balance-sequencing.md`
  (§3 clean-room provenance).
- D10 artifacts (all of
  `docs/work-packages/20260705-mofefid-d10-shock-numerics-gap005-001/artifacts/`,
  especially `hold-legitimacy-audit.md`, `source-authority-evidence.md`,
  `iwagaki-case4-evidence.md`, `h2637-resolution-evidence.md`,
  `numerics-convergence-evidence.md`).
- `docs/work-packages/20260705-mofefid-d9-dval-disposition-001/artifacts/case4-d10-handoff.md`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs`
- `/home/workdir/openWEPP/tools/dval/compare_dval.py`
- `references/copyrighted/Iwagaki1955_runoff_characteristics_DPRI10.pdf`,
  `references/copyrighted/Papanicolaou2018.md` (+ PDF reference list),
  `references/copyrighted/10.1061@ASCE0733-94291992118@101359.pdf`,
  `references/copyrighted/mingham2001.pdf`,
  `references/copyrighted/Lighthill_Whitham_1955_Kinematic_Waves.pdf`,
  `references/copyrighted/19840021490.pdf` (Davis 1984, ICASE 84-20 /
  NASA CR-172373; `19840021490.md` is a Gemini conversion — spot-check
  any equation against the PDF before citing it as binding authority),
  and `references/copyrighted/Tseng2010_Hydroinformatics.pdf` when source
  provenance is needed. Copyright governance applies; summarize, do not
  vendor raw rows or long source excerpts.

Required-reading budget: `347440` local bytes for core + triggered
contract/kernel conditional pre-edit reading, `OK`; map:
`artifacts/required-reading-map.md`.

Files:

- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/work-packages/20260706-mofefid-d10b-gap005-source-authority-reconciliation-001/**`
- `docs/work-packages/README.md`
- `references/annotated_bibliography.md` + acquired reference files per
  rights governance.
- `docs/planning/mofe-fidelity-campaign-strategy.md` §6.1 row on closure.
- Conditional D-val / Lane D solver files listed in `package.md`.

Task: execute the `GAP-OFEROUTE-005` source-authority reconciliation end to
end per the package's three legs: (A) bind limiter/CFL/dissipation to the
TVD family authority (all four family primaries in hand: Mingham 2001,
Garcia-Navarro 1992, Davis 1984, Tseng 2010 — the latter two from R-63's
own citation chain, acquired 2026-07-06); (B)
re-anchor Case-4 acceptance to the Iwagaki 1955 primary (characteristics
oracle + experimental hydrographs + `n = 0.009` via a named definitional
Manning-to-Darcy mapping) and demote the digitized enhanced-WEPP trace to
an ADR-0017 comparator flag; (C) reclassify the OFE sampled handoff under
the conservation hard gate, build the seam-decomposed ledger on H2637, and
adjudicate/correct the resolution sensitivity. Then land the contract-first
correction if the seven DC gates pass, or close in a legitimate HOLD behind
a boundary narrower than D10's.

Contract timing: S2 may ratify the source-authority structure and
contract-derived obligations before production edits, but final Case-4
tolerances, TVD-family binding claims, and handoff conservation tolerances
must cite the S3/S4 oracle-constructibility and convergence evidence. If that
evidence cannot be produced, leave the final authority held rather than
ratifying an unproven acceptance surface.

Constraints: contract-first sequencing; canonical `SC-*` authority;
CLEAN-ROOM — no Papanicolaou implementation material beyond the published
paper/supplemental; typed guards; no silent defaults; no
canonicalize-and-proceed for domain violations; no production/default
activation; no `k_o` tuning as authority; D11 rev-20/21 production operand
path untouched; no pre-filled expected values in evidence artifacts.

Findings carried in are hypothesis-grade (package.md "Findings Carried
In"): the printed (11c) limiter branch inversion, the handoff aliasing
candidate, the boundary-cell dissipation zeros, and the
behavior-pinned-test question. Adjudicate them with the acquired authority
and the re-anchored harness; do not treat them as prescribed fixes.

DC closure: do not hold while source reading, oracle construction,
contract/test work, or validation remains possible inside the declared
envelope. D10's hold boundary ("primaries do not bind implementation
parity") is consumed by this package's re-anchoring and cannot be re-cited.
If `HOLD` is claimed, record a hold-legitimacy audit naming the (new)
boundary, evidence, considered in-envelope route, and why it cannot close
now.

No surrogate physics: production code must implement contract-backed or
source-authorized numerics only.

Real consumer proof: if a numerical correction lands, prove the real
solver/cascade path and the D-val/H2637 harness consume it; shadow-only
metrics or stale compatibility paths cannot carry the closure claim.

Conservation/output acceptance: record operand lineage before
solver/cascade/handoff edits; separate plausible aliases; reject known
wrong formulas; independent reconstruction plus real closure audit;
convergence-trend evidence across at least three resolutions; never
one-sided bounds or self-consistency alone.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for heavy
Case-4/H2637 sweeps, full workspace nextest, and other heavy closure gates
when available; do NOT run them on the parent model unless unavailable, in
which case record command-level evidence. This prompt explicitly authorizes
subagent spawning/delegation to `comparator_suite_runner`, `explorer`,
`rust_code_reviewer`, and `rust_qa_reviewer` for heavy gate execution,
source/harness inspection, read-only review, and verification. Outputs:
compact metrics, findings, log paths, and package-local artifact text.
Write access: read-only unless a later operator grants a bounded write set.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases and
leave no accepted review finding undispositioned.
