# Review Agent B Remediation Addendum

Evidence mode: **Static + Ran**. Re-audit completed at
`2026-08-02T06:25:40Z` from `/home/workdir/openWEPP` at Git HEAD
`045cac9475738b0306a89a934702c479803f0935`. I did not read
`review-agent-a.md`.

## Findings

### Blocking pending evidence — B-04 terminal campaign

The final-source quick, frost, and full suites remain pending in
`gate-results.md`. The signal-terminated quick attempts are correctly marked
invalid and cannot close this requirement. This is a pending independent gate,
not a rejection of the source, science, analysis, or provenance remediations
below. Review B therefore remains **HOLD only for the terminal suite bundle**.

No other blocking or material QA finding remains.

## Remediation Disposition

| Original finding | Verdict | Exact-tree basis |
|---|---|---|
| B-01 — frozen operators and B/L/S/LS attribution | **ACCEPTED** | `run_accumulation_diagnostics.py` executes the retained `paired_snow_rows`, `last_snow_date_by_water_year`, and `peak_date_by_water_year` rubric for every cell. `accumulation-mechanics-results.json` contains all five operators and all 16 B/L/S/LS cells; baseline executed offsets exactly equal the frozen values (`-35.0`, `-46.5`, `-31.0`, `-37.0`, and `-44.5` days). Each cell also has the expected 19–23 water-year pre-peak ledgers. The synthesis now distinguishes seasonal peak magnitude from SWE retained on observed peak dates and keeps input/loss ownership unresolved. |
| B-02 — independent closures and real-consumer proof | **ACCEPTED** | The primary runtime now carries `active_precipitation_m` from the independent pre-partition SIMIMPL28 operand. Analysis independently reconstructs active total, rain/snow amounts and fractions, physical depth/SWE, uncapped four-term CoE sum, applied term-plus-cap sum, daily raw melt, accumulation, trace-to-WAT values, and zero implemented redistribution. Final maxima range from `0` to `2.998e-15 m`, inside the `1e-12` contract tolerance. Numeric CoE/cap/corruption tests and the real trace formatter test prevent field aliasing. Review-discovered density-100 masking in the snowbench adapter was corrected to use the exact runtime depth-to-SWE operand; the new 200 kg/m3 regression passes and proves the compatibility path is executable. |
| B-03 — exact-terminal behavior neutrality | **ACCEPTED** | `behavior-neutrality.json` is self-bound to final binary `b50dd71c…`, final receipt `6f6bfe36…`, and current comparator `1b3f331d…`. It compares all 16 final v3 cells against retained v2 and reports exact zero difference for 245,456 WAT rows, 736,368 WAT numeric values, and 72,093,744 numeric values across all 111 prior v2 trace fields. All ten receipt-bound production-source hashes and the release-binary hash match the current files. Superseded cohorts and interrupted attempts are explicitly invalidated and not reused. |
| B-04a — non-suite provenance | **ACCEPTED** | `validation-execution-provenance.md` records the final HEAD, dirty/diff identities, scoped source and input hashes, exact argv, working directory, exit, duration, log/time paths, and supported requirement for every direct gate. It also binds the final 16/16 cohort (`273.07 s`) and exact-terminal comparator (`254.85 s`) and reconciles the invalidation chronology. Durable logs confirm focused tests, semantic unit tests, affected-crate Clippy with `-D warnings`, fmt, diff check, unit registry, deny, assurance validation/render freshness, and scoped Markdown checks. |
| B-04b — quick/frost/full exact-head suites | **PENDING** | Kept separate as requested. Existing terminated starts are invalid evidence; a fresh final-source bundle must pass and be receipted before B-04 and package closure can be accepted. |
| B-05 — calibration readiness and causal wording | **ACCEPTED** | `calibration-readiness-matrix.md` uses the orthogonal `IMPLEMENTED`, `NOT_CALIBRATION_READY`, and `NONIDENTIFIABLE` statuses and dispositions all ten governed readiness obligations with evidence paths and applicability rationales. `scientific-synthesis.md` and `scientific-disposition.md` do not assign a unique cause to forcing, phase, retention, redistribution, or pre-peak loss and do not authorize tuning or promotion. |

## Reviewer-Run Checks

From `/home/workdir/openWEPP` on the final source identity:

- `cargo fmt --all -- --check` and `git diff --check` — pass;
- `cargo clippy -p openwepp-hillslope-orchestrator -p openwepp-runner --all-targets -- -D warnings` — pass;
- `cargo nextest run --test snow_surface_eb04w_accumulation_melt_diagnostics_contract` — 2/2 pass, run `57b81235-ec96-447f-b54e-1bb0aa4b4155`;
- targeted CoE ledger, real formatter, and non-100-density snowbench tests — 1/1 pass each, runs `166994c1-bb75-496f-a321-ba68f41cc6ba`, `4f657170-c832-49b2-8517-3ef7e783102d`, and `fde11210-f4cb-4f17-823f-c66d9943a20a`;
- `cargo deny check` — pass for advisories, bans, licenses, and sources, with only the retained unmatched `MIT-0` allowance warning;
- assurance `validate --all` — pass for three DRAFT sources and zero public reports;
- full-catalog rendered-review check — 92 files current; and
- independent receipt verification — all ten production-source hashes and the release-binary hash match the exact current files.

A preliminary mistyped nonexistent test-target invocation returned `101`
without running tests; the corrected canonical target is the 2/2 passing run
recorded above and in the durable provenance artifact.

## Non-Blocking Debt And Follow-Ups

- `infiltration_reconciliation.rs` and `runoff_reconciliation.rs` remain in the
  repository warning band at 2,353 and 2,504 lines. The EB-04W trace formatter
  was appropriately extracted; later mechanical decomposition remains useful.
- The integration contract includes source-text bindings that are sensitive to
  harmless refactors. The new numeric unit, real-formatter, cohort, and
  noncanonical-density tests now carry the behavioral assurance; retain those
  tests if source layout changes.

## QA Statement

The B-01, B-02, B-03, B-04 provenance, and B-05 remediations pass secondary QA
on the exact final source. The only remaining hold is the separately owned,
fresh quick/frost/full terminal suite bundle.
