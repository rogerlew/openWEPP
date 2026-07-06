# Kickoff Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):

Core:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`
- `docs/planning/mofe-fidelity-campaign-strategy.md` §6.1 and §7
- `docs/ROADMAP.md` §M
- `docs/work-packages/20260705-mofefid-d14-laned-runtime-profile-optimization-001/package.md` and artifacts
- `docs/work-packages/20260705-mofefid-d15-opt-in-production-activation-001/package.md`
- `docs/work-packages/20260706-mofefid-d10b-gap005-source-authority-reconciliation-001/package.md`
- `docs/work-packages/20260706-mofefid-d10b-gap005-source-authority-reconciliation-001/artifacts/gate-results.md`
- `docs/work-packages/20260706-mofefid-d10b-gap005-source-authority-reconciliation-001/artifacts/review-response-claude.md`
- D11/D12/D13 final dispositions and handoffs enough to verify operand,
  source-shape, and routed-hydrograph consumer obligations.

Conditional:

- `crates/AGENTS.md` and `tests/AGENTS.md` before Rust/test edits.
- `tests/fixtures/AGENTS.md` before fixture edits.
- `SC-RUNOFFPART-001`, `SC-SUBHYD-001`, or `SC-SED-001` before changing their
  owned contract surfaces.

On-demand:

- Lane D seam-design, seam-implementation, and runtime-shadow package
  artifacts when tracing `INV-OFEROUTE-012`.
- `docs/standards/local-ci-gate-selection.md` for gate tier selection.

Required-reading budget: 350,039 bytes, OK; map:
`artifacts/required-reading-map.md`.

Files: package-local artifacts and prompts; `docs/work-packages/README.md`;
`docs/planning/mofe-fidelity-campaign-strategy.md`; `docs/ROADMAP.md`;
conditional Rust/test write set from `package.md` only if Phase C proceeds.

Task: refresh the D14 endpoint timing on the D10B-corrected Lane D path, audit
D15 activation readiness, and either complete the contract-authorized opt-in
production activation flip or close in an executed hold with exact blockers and
first follow-on.

Constraints: contract-first sequencing; canonical `SC-*` authority; typed
guards; no silent defaults; no canonicalize-and-proceed for domain violations;
no surrogate/provisional/proxy/heuristic process physics; no compatibility
wrapper or shadow path can carry the production activation claim.

Real consumer proof: move the real active downstream consumer to the routed
path and prove old DC01/compatibility/shadow paths are not carrying the
activation claim. Producer-only, shadow-only, and counter-only evidence cannot
close activation.

Conservation/output acceptance: record active closure operand lineage; reject
old/DC01 aliasing; prove `ui_SCrunf` source consumption, `latqcc` bypass,
runtime closure hard-fail, DC01-disable/no-double-feed, routed-hydrograph to
erosion consumer, protected subsystem-off identity, and H2637 timing.

Subagent requirement: REQUIRED: spawn `comparator_suite_runner` for H2637
timing and heavy closure gates when available; spawn review/verification
subagents for package closure. Standing user authorization for openWEPP
subagent delegation is present in this session. This prompt explicitly
authorizes subagent spawning/delegation to comparator/timing, explorer,
review, and verification roles for the package scope; outputs: compact metrics,
findings, and log paths; write access: read-only unless an implementation fix
is explicitly assigned a bounded write set.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts and final disposition for all completed
phases.
