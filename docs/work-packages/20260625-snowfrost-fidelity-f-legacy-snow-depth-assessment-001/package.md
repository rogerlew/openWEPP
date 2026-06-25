# SNOWFROST-FIDELITY-F Legacy Snow-Depth Assessment

Status: completed

Package type: diagnostic comparator/output-capture assessment.

Primary gap: `GAP-SNOWFREEZE-002`.

Objective: determine whether pinned legacy WEPP is materially better than
current openWEPP at predicting observed physical snow depth, capture legacy
physical snow depth through an explicit output surface, and quantify current
openWEPP `Snow-Water` SWE and `Snow-Depth` against pinned legacy WEPP on the
five snow/frost observation pilot fixtures.

This package follows `AGENTS.md`, `docs/work-packages/AGENTS.md`,
`docs/specifications/science-contracts/AGENTS.md`,
`docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`,
`tests/AGENTS.md`, `tools/snowfreeze_observed/README.md`, and
SNOWFROST-FIDELITY-A/B/C/D/E.

Subagent authorization: this package explicitly authorizes
spawning/delegating to read-only science-review, harness-review, and
verification subagents for legacy output-lineage review, comparator
anti-alias review, and final evidence review. Expected outputs are compact
findings summarized into `artifacts/review-disposition.md` and
`artifacts/verification.md`; subagents may not edit files. Current execution
uses local reviews unless the operator separately requests subagent dispatch.

## Purpose

SNOWFROST-FIDELITY-E proved that openWEPP physical snow depth fails paired
snow-depth control at Sites 1, 2, and 4 and that the failures are dominant
modeled-over-observed. Before changing snow physics, this package asks a
separate comparator question: does the pinned legacy baseline do better on the
same physical snow-depth observations, and how close are current openWEPP SWE
and snow depth to legacy?

Legacy remains a flagging comparator under ADR-0017, not a correctness target.
Observed physical snow depth plus `INV-SNOWFREEZE-048` remain the
correspondence authority.

## Non-Goals

- Do not change production snow/frost physics, constants, runtime control flow,
  or default activation.
- Do not tune snow depth, frost depth, heat flow, frozen conductivity, SFCC,
  impedance, snow settling, or precipitation partitioning.
- Do not enable, port, approximate, or promote `Qwet`/`frzftp`.
- Do not classify openWEPP as defective based solely on legacy agreement.
- Do not make legacy WAT `Snow-Water` a physical snow-depth proxy.

## Authority Envelope

In scope:

- `SC-SNOWFREEZE-001#INV-SNOWFREEZE-048` physical snow-depth correspondence;
- pinned legacy WEPP baseline under
  `/home/workdir/wepp-forest_260430_baseline`;
- legacy WAT `Snow-Water` as SWE only;
- legacy daily-winter hour-24 snow depth as date-aligned physical snow depth;
- legacy daily-winter hour-24 snow density as density anti-alias evidence;
- legacy large-graphics `treal(73)=snodpy*1000` and `treal(75)=densg` as
  sparse source-line/operand provenance, not the date-aligned comparator feed;
- current openWEPP WAT `Snow-Depth` and `Snow-Water`;
- all five `tests/fixtures/snowfreeze_observed` pilot sites.

Out of scope:

- production process-physics edits;
- observation tolerance changes;
- new external observation acquisition;
- frost heat-flow/frozen-K/SFCC mechanism selection;
- compatibility bit-parity or direct default activation.

## Intended Write Set

- `docs/work-packages/20260625-snowfrost-fidelity-f-legacy-snow-depth-assessment-001/**`
- `docs/work-packages/README.md`
- `tools/snowfreeze_observed/README.md`
- `tools/snowfreeze_observed/legacy_snow_compare.py`

## Phase Plan

### Phase 0: Scaffold and Authority Lock

- Create package scaffold, kickoff prompt, and required-reading evidence.
- Record the SNOWFROST-FIDELITY-E route decision as the starting point.
- Lock the no-physics-change and legacy-as-flag boundaries.

Exit criteria:

- Package artifacts exist.
- Package scope distinguishes legacy SWE from physical snow depth.

### Phase 1: Legacy Snow-Depth Capture

- Add a bounded legacy replay helper that copies each fixture into a temporary
  run directory and changes only the existing `.run` large-graphics and
  daily-winter answers from `No` to `Yes`.
- Parse legacy WAT `Snow-Water` for SWE.
- Parse legacy daily-winter hour-24 snow depth and density for date-aligned
  physical snow-depth comparison.
- Parse legacy large-graphics `treal(73)` and `treal(75)` as sparse operand
  provenance.
- Fail closed if the WAT, daily-winter, or large-graphics files are absent.

Exit criteria:

- Source-line evidence identifies the legacy snow-depth and density operands.
- The helper emits explicit lineage for legacy SWE and physical snow depth.

### Phase 2: All-Site Comparator Assessment

- Run current openWEPP observed comparisons for all five pilot sites.
- Run pinned legacy WEPP large-graphics replays for all five pilot sites.
- Compare observed snow depth against openWEPP and legacy physical snow depth.
- Compare openWEPP `Snow-Depth` against legacy daily-winter snow depth on
  common model dates.
- Compare openWEPP `Snow-Water` against legacy WAT `Snow-Water` on common
  model dates.

Exit criteria:

- JSON and Markdown comparator artifacts exist under this package.
- The report states paired observed-depth counts, pass/fail counts, mean/max
  residuals, legacy-better rows, openWEPP-better rows, and openWEPP-vs-legacy
  depth/SWE delta statistics.

### Phase 3: Review, Verification, and Disposition

- Run focused validation commands.
- Record dual local review and finding disposition.
- Record verification and closure evidence.
- Update tool/package documentation.

Exit criteria:

- Gate table has no unjustified `FAIL`, `BLOCKED`, or `NOT RUN`.
- Final disposition states whether the next package should use legacy as a
  source-code guide, a non-target flag, or no useful snow-depth comparator.

## Validation Commands

Run from `/home/workdir/openWEPP`.

- `.venv/bin/python -m py_compile tools/snowfreeze_observed/legacy_snow_compare.py`
- `cargo build -p openwepp-runner --bin openwepp-cli-hill`
- `.venv/bin/python tools/snowfreeze_observed/legacy_snow_compare.py --observations-dir tests/fixtures/snowfreeze_observed/observations --output-dir target/snowfrost_fidelity_f_legacy_compare --output-json docs/work-packages/20260625-snowfrost-fidelity-f-legacy-snow-depth-assessment-001/artifacts/legacy_snow_comparison.json --output-md docs/work-packages/20260625-snowfrost-fidelity-f-legacy-snow-depth-assessment-001/artifacts/legacy_snow_comparison.md`
- `rg -n "treal\\(73\\)|treal\\(75\\)|snodpy\\(iplane\\)|tmpvr7=snodpt|densgt|Snow depth \\(mm\\)|daily winter|Snow-Water" /home/workdir/wepp-forest_260430_baseline/src/bigout.for /home/workdir/wepp-forest_260430_baseline/src/bighdr.for /home/workdir/wepp-forest_260430_baseline/src/outfil.for /home/workdir/wepp-forest_260430_baseline/src/winter.for`
- `rg -n "qwet|Qwet|frzftp" crates || true`
- `git diff --check`

## HOLD Boundaries

Close as `HOLD` only if the pinned legacy binary cannot run the checked-in
fixtures, if legacy physical snow depth cannot be captured without modifying
legacy source or production openWEPP physics, if current openWEPP WAT outputs
cannot be regenerated, or if source provenance contradicts the assumed legacy
large-graphics snow-depth semantics.
