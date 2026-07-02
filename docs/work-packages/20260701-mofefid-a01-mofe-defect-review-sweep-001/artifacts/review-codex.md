# Codex Review - MOFEFID-A01

Scope: Lane A/A01 closeout at `5e46d563`, with A01 package content landed
through `b413fa37`. The A01 commit range changes only package artifacts; no
production source or tests are modified by A01.

## Findings

### A01-R1 - Deferred cleanup - F-A2 should cite the pinned baseline source line

Evidence class: Static + Ran.

F-A2's technical conclusion is sound: current openWEPP computes WB14
infiltration from the local hyetograph only
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:1315-1371`)
and adds upstream surface/lateral carry later as `runon_input_m`
(`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:167-174`,
`crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs:652-688`).
The legacy source does feed upstream water into the downstream OFE water supply.

The cleanup is provenance precision. `findings.md:10` and
`sweep-notes.md:149-150` cite current `/workdir/wepp-forest/src` line numbers
(`watbal_hourly.for:411-413`). Root provenance defaults normative legacy source
reads to `/workdir/wepp-forest_260430_baseline`, where the same expression is at
`/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:361-363`, and the
per-hour array carry into `xfin` is at `:471-473`. The current-source citation is
not wrong on substance, but the final artifact should include the pinned
baseline line(s) so the source-intent evidence matches repository governance.

Disposition candidate: deferred evidence-hygiene cleanup. This does not block
Lane A's technical verdict because the pinned baseline confirms the same
behavior.

## Review Verdict

Accepted, with A01-R1 as non-blocking cleanup.

The six-surface sweep is adequately evidenced. I agree with the package's main
dispositions:

- F-A1 is real but correctly kept inside the existing `INV-RUNOFFPART-030` /
  `INV-SED-012` sediment-coupling hold; the current path is manifest-labeled as
  water-transfer-only compatibility, and the water path does not consume the
  seeded case classifier.
- F-A2 is the right headline: it is a source-intent and fidelity divergence, not
  a conservation failure. Current contracts already mark MOFE carry/runon
  storage-ingress promotion as follow-up scope, so routing it to Lane D D1 /
  `MOFEFID-D01` contract authority is defensible.
- F-A3, F-A4, F-A5, and F-A6 are reasonable hardening/capability/watch-item
  dispositions, not current production defects.

No production defect or closure-gate issue was found in A01 itself. The package
still needs the normal review-disposition artifact at close; if A01-R1 is
accepted, that disposition can be a one-line provenance cleanup or an explicit
decision to keep current-source line numbers as supplemental evidence.

## Commands Run

- Ran: `git status --short`
- Ran: `find docs/work-packages -maxdepth 1 -type d | sort | rg -i 'mofefid-a|defect-review|sweep'`
- Ran: `find docs/work-packages/20260701-mofefid-a01-mofe-defect-review-sweep-001 -maxdepth 3 -type f | sort`
- Ran: `nl -ba docs/work-packages/20260701-mofefid-a01-mofe-defect-review-sweep-001/package.md`
- Ran: `nl -ba docs/work-packages/20260701-mofefid-a01-mofe-defect-review-sweep-001/artifacts/findings.md`
- Ran: `nl -ba docs/work-packages/20260701-mofefid-a01-mofe-defect-review-sweep-001/artifacts/sweep-notes.md`
- Ran: `git show --stat --oneline b413fa37`
- Ran: `git show --name-only --format='%h %s' b413fa37`
- Ran: `git diff --stat fe65662c..b413fa37 -- docs/work-packages/20260701-mofefid-a01-mofe-defect-review-sweep-001`
- Ran: `git log --oneline --decorate --grep='MOFEFID-A01' --all`
- Ran: `git diff --name-only fe65662c..b413fa37`
- Ran: `nl -ba docs/planning/mofe-fidelity-campaign-strategy.md`
- Ran: `rg -n "Lane D|MOFEFID-D01|runon|re-infiltration|reinfilt|Papanicolaou|F-A2|A01" docs/planning/mofe-fidelity-campaign-strategy.md docs/work-packages docs/backlog docs/specifications/science-contracts/contracts -g'*.md'`
- Ran: `nl -ba docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- Ran: `nl -ba docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- Ran: `nl -ba crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`
- Ran: `nl -ba /workdir/wepp-forest/src/watbal_hourly.for`
- Ran: `nl -ba /workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- Ran: `rg -n "fin\s*=\s*fin|ui_HUrunf|ui_LfUrf|roffon|runoffin|subrin" /workdir/wepp-forest_260430_baseline/src/watbal_hourly.for /workdir/wepp-forest/src/watbal_hourly.for`
- Ran: `git -C /workdir/wepp-forest_260430_baseline rev-parse HEAD`
- Ran: `git -C /workdir/wepp-forest rev-parse HEAD`
- Ran: `nl -ba crates/openwepp-runner/src/hillslope/direct_seed_projections/02_mofe03_wave2_projection.rs`
- Ran: `nl -ba crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs`
- Ran: `rg -n "case_value|erod14_case|vj|qj|fh|fp|QcapSoftLimit|closure_residual_m" crates docs/specifications/science-contracts/contracts docs/work-packages/20260701-mofefid-a01-mofe-defect-review-sweep-001`
- Ran: `nl -ba crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`
- Ran: `nl -ba crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
- Ran: `rg -n "QcapSoftLimit|QCAP_SOFT_LIMIT" crates tests docs`
- Ran: `nl -ba crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`
- Ran: `nl -ba crates/openwepp-hillslope-orchestrator/src/direct_runtime/projection.rs`
- Ran: `rg -n "watbtm|watpdg|frwatc_net_liquid_delta_m|Dp \\+=|deep.*watbtm|frost residual|frost.*residual" crates docs/work-packages/20260701-mofefid-a01-mofe-defect-review-sweep-001 docs/work-packages/20260701-hillperf-frost-single-solve-001 -g'*.rs' -g'*.md'`
- Ran: `rg --files tools crates tests docs | rg 'owcmp|closure|wat|pass|audit|per.*ofe'`
- Ran: `find docs/work-packages -maxdepth 1 -type d | sort | rg -i 'mofefid-d|runoffpart-030|erod14|mofe.*sediment|qin|routing-contract|papanicolaou'`
- Ran: `rg -n "INV-RUNOFFPART-030|MOFEFID-D01|Lane D D1|D01|hold-closure package|EROD14|qin" docs/planning/mofe-fidelity-campaign-strategy.md docs/work-packages docs/backlog docs/decisions docs/specifications/science-contracts/contracts/SC-SED-001.md docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- Ran: `nl -ba docs/decisions/0018-defect-closure-execplans-conversion-rule.md`

Not run: cargo fmt/clippy/nextest/deny, simulations, or comparator harnesses.
A01 is a review/artifact package with no production source changes, so static
source and contract reads were the load-bearing review evidence.
