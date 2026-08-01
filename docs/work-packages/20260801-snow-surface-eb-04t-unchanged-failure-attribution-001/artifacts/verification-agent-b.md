# Independent Terminal Verification B

Evidence class: `Ran + Static`.

Verdict: `PASS`.

## Verification Scope

Verified the amended EB-04T analytical consumer, hash-bound retained inputs,
JSON/CSV outputs, process attribution, criterion-fitness assessment, scientific
synthesis, regenerated figures and sidecars, review disposition,
roadmap/catalog claims, and protected diff. This verification did not execute
the model or alter EB-04S.

## Ran Evidence

- `.venv/bin/python tools/analyze_failures.py --self-check`: `PASS`, including
  exact frozen failure inventory and rejected-alias control.
- Independent regeneration in `/tmp/tmp.3Y8twIcuIG`: all generated JSON, CSV,
  synthesis, three SVGs, and three sidecars are byte-identical to the package.
- Independent CSV reconstruction: `16` rows; LS direction `15 away / 1
  unchanged`; five timing rows are all `open`, with `4 away / 1 unchanged`.
- Independent factorial reconstruction: `13` nonzero selected-error
  interactions (`12` mitigating additive error, `1` amplifying) and `3` exact
  zeros. Niwot peak-depth reproduces B/S/LS offsets `-46.5/-42.5/-46.5 d`
  with a `+4 d` interaction; Niwot peak SWE reproduces
  `-31/-33.5/-31.5 d` with a `-2 d` interaction.
- All six recorded retained-input hashes match their current read-only files.
- All three SVGs parse. The generated interaction column and open-control
  timing labels are present.
- `markdown-doc lint --path <EB-04T package>`: `20 files validated, 0 errors,
  0 warnings` before this verification artifact was added.
- `git diff --check`: `PASS`.
- Scoped status over `crates`, `tests`, science contracts, EB-04R, and EB-04S:
  empty.

The first isolated regeneration attempt used system Python and stopped because
that interpreter lacks Matplotlib. Re-execution with the repository-required
`.venv/bin/python` completed successfully and produced the byte-identical
evidence above; this was a verifier environment mismatch, not a package defect.

## Amendment Verification

### Timing terminology — PASS

The figure, SVG axis/title, and sidecar consistently use modeled minus observed.
The artifact stem is now `eb04t-open-control-timing`; the obsolete
`eb04t-target-sensitive-timing` stem is absent from generated artifacts and the
generator. Negative offsets are correctly described as early model timing.

### Noncausal mechanism language — PASS

The synthesis says responses are associated with S-enabled cells and explicitly
denies unique causal assignment. Process attribution likewise calls the
factorial readings associations. The L result is accurately scoped to exactly
zero primary-error effect on these 16 selected metrics, not to zero effect on
complete trajectories.

### Open-control/canopy-longwave boundary — PASS

All five timing failures independently reconstruct as open SNOTEL controls.
Package prose consistently treats them as evidence about sublimation and
combined-LS noninferiority and states that they cannot identify canopy-longwave
efficacy. The two depth-SWE rows are retained as mixed/ambiguous density and
interception geometry guards rather than clean longwave evidence.

### Interaction reporting — PASS

The analytical consumer uses the declared factorial residual
`LS - L - S + B` on the selected primary-error magnitude. Counts and Niwot
examples reproduce, the matrix figure includes the interaction column, and the
sidecar distinguishes mitigation/amplification of additive error from movement
toward/away from observations. The no-materiality and no-unique-causality
limits remain explicit.

### EB-04S preservation — PASS

EB-04T remains retained, diagnostic, and result-aware. It introduces no
threshold, rerun, calibration, default activation, or retrospective promotion.
Its conclusion—that the original criterion was mixed but the retained evidence
still provides no affirmative combined-default basis—is consistent with and
does not rewrite EB-04S.

## Terminal Assessment

The accepted review findings are corrected, generated artifacts are
deterministic, empirical counts reproduce, and the amended claims are properly
bounded. No verification finding requires remediation. `PASS` supports the
owner's remaining mechanical status, gate, prompt-archive, and final-disposition
reconciliation.

## Exact-Terminal Closure Addendum

Evidence class: `Ran + Static`.

Final closure verdict: `PASS`.

Ran after final lifecycle reconciliation:

- confirmed package status `complete / DIAGNOSTIC_COMPLETE` and all progress
  items checked;
- confirmed `gate-results.md` records every declared gate as `PASS` and closes
  `DIAGNOSTIC_COMPLETE`;
- confirmed all reviewer findings are accepted/resolved and both independent
  verifications are recorded as passing;
- confirmed `exact-diff-reconciliation.md`, `worker-handoff.md`, and
  `final-disposition.md` preserve the retained-only scope, EB-04S
  nonpromotion/default-off, no-materiality boundary, open-control limitation,
  and EB-05 handoff;
- confirmed the kickoff prompt exists only under `prompts/archived/` and the
  prompt-state README records terminal archival;
- confirmed all six retained-input hashes still match;
- confirmed the exact terminal status contains only the new EB-04T package and
  the three declared roadmap/catalog files; production, tests, contracts,
  EB-04R, and EB-04S remain clean;
- confirmed `eb04t-open-control-timing.{svg,md}` is present, the obsolete
  target-sensitive stem is absent, and the inventory/generator use the new
  stem;
- ran scoped Markdown lint over the 24 then-current package Markdown files and
  each roadmap/catalog file: zero errors and zero warnings; and
- reran `git diff --check`: `PASS`.

The finalized roadmap and catalog consistently record EB-04T as diagnostic
complete, preserve EB-04S, and name EB-05 assurance as next. No terminal
finding remains.
