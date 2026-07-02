# Codex Review - MOFEFID-C03 Lateral-Flow Envelope Judgment

Date: 2026-07-02
Reviewed head: `aa43ddd8` (`b2807d5d` C03 verdict, `aa43ddd8` campaign status)
Scope: review only. No production code, tests, or contracts were modified.

## Evidence Classes

Static:

- Read `package.md` and `artifacts/verdict.md`.
- Read the `SC-SUBHYD-001.md` rev 14 diff for `INV-SUBHYD-033`.
- Compared C03 against C01's final envelope/disposition artifacts.
- Read WAT/PASS publication code and tests around `QOFE`, `latqcc`, `runvol`, and `Area`.
- Checked the committed file set for C03 runner/input artifacts.

Ran:

- `.venv/bin/python /tmp/claude-1000/-home-workdir-openWEPP/e46d9841-ba57-46c6-9ae7-061c6c19110b/scratchpad/c03_quickflow.py` - reproduced the C03 table.
- `sha256sum` on the scratch runner and scratch input parquet:
  - `c03_quickflow.py`: `b8057d928e8693967d96447f16fdff38f75f6c68abb8fa67e2c6dcbab52b439c`
  - `dc01-m3/out/H2637.wat.parquet`: `828c648202bd99cc78f406d2d4a8cf7ce4e3d32143f63a6cc2d11ceca713da4c`
  - `dc01-m3/out/openwepp_hillslope_run_manifest.json`: `d053cb7580e3fabe977ea892b6ca401f274bae43bc90da71ed2dbd6efd840a77`
- Ran an independent operand reconstruction on the same scratch WAT/PASS parquet.
- Ran an independent quickflow-slope sensitivity check over the scratch WAT parquet.
- Re-read Panola and WS10 threshold anchors with targeted `pdftotext`/`rg`.
- `bash tools/release/check_authority_suite_antievasion.sh` - PASS.
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract` - PASS, 2 tests.

Not run:

- I did not regenerate the H2637 parquet from source inputs. The package does not provide a committed C03 run recipe or input manifest sufficient for that.

## Findings

### C03-CX-001 - Accepted Candidate - The `Ran` verdict is not reproducible from committed C03 artifacts

Evidence class: Static + Ran.

`artifacts/verdict.md:3-5` says the result comes from per-storm decomposition of a post-DC01 H2637 WAT parquet using `c03_quickflow.py`. The committed C03 package contains only `package.md` and `artifacts/verdict.md`; `git ls-files` found no `c03_quickflow.py`, no input parquet manifest, no script hash, no command line, and no generated raw output. I found the script only in scratch at `/tmp/claude-1000/-home-workdir-openWEPP/e46d9841-ba57-46c6-9ae7-061c6c19110b/scratchpad/c03_quickflow.py`, and it hard-codes the scratch input path `/tmp/.../scratchpad/dc01-m3/out/H2637.wat.parquet`.

I did run that scratch script and reproduced the package table, so this is not a claim that the numbers were fabricated. It is a closure evidence defect: C03 resolves FARPOINT01 from `Ran` evidence, but the package does not preserve the runner, input provenance, hashes, or reproduction command in the artifact set.

Disposition recommendation: add a package-local raw-evidence artifact or committed analysis script with exact input path/provenance, input hash, command, dependency assumptions, and output. The scratch hashes above are enough to start that record but should not remain only in a review artifact.

### C03-CX-002 - Accepted Candidate - Quickflow separation parameter authority is unpinned and can change the ENV-E boundary result

Evidence class: Static + Ran.

`package.md:30-42` and `artifacts/verdict.md:46-57` correctly identify total daily export as a false comparand for event tiers, and `SC-SUBHYD-001.md:298` now requires quickflow-separated event response. That direction is sound. The problem is that the package does not specify a reproducible, authority-bound separation algorithm.

The scratch script uses `bf_slope = 0.02` at `/tmp/.../scratchpad/c03_quickflow.py:14` and comments that this is a conservative daily equivalent of the Hewlett-Hibbert line. The committed artifact only says "Hewlett-Hibbert-style constant-slope baseflow separation" (`artifacts/verdict.md:13-15`). The WS10 source states the separating line slope as `0.55 L s^-1 km^-2 h^-1`; the package does not show the unit conversion or justify why `0.02 mm/day per day` is the accepted daily-step parameter.

My sensitivity check on the same scratch WAT parquet:

| baseflow slope used in script variant | wet-large event ratio | Spearman ratio vs P | Spearman ratio vs AP14 |
|---:|---:|---:|---:|
| 0.020 | 0.4637 | 0.2936 | 0.6517 |
| 0.04752 | 0.3999 | 0.3924 | 0.6328 |
| 0.55 | 0.2914 | 0.5434 | 0.5042 |
| 1.14048 | 0.2449 | 0.5544 | 0.4743 |

The reported ENV-E pass depends on this parameter being defined. Under a daily-step interpretation of the cited hourly slope, the wet-large ratio falls just below the `[0.25, 0.80]` lower bound. That is close enough that C03 needs a parameter-authority/sensitivity artifact before the all-tier verdict is review-clean.

Disposition recommendation: pin the exact quickflow separation algorithm in the artifact/contract or a committed script, document the Hewlett-Hibbert unit conversion, and either justify the selected slope from authority or show the ENV-E verdict is robust across the accepted parameter range.

### C03-CX-003 - Accepted Candidate - ENV-Y operand lineage is ambiguous between contract text and the outlet-export convention C03 actually used

Evidence class: Static + Ran.

`SC-SUBHYD-001.md:298` still defines ENV-Y as `(SUM runvol + SUM latqcc*A_ofe)/(SUM P*A_total)`. `artifacts/verdict.md:9-15` says C03 used outlet OFE `QOFE` plus outlet OFE `latqcc`, both scaled by `A_out/A_total`. Those are not the same reading on the H2637 WAT file.

Independent reconstruction on the C03 scratch parquet:

| Operand interpretation | Fraction of P |
|---|---:|
| outlet `QOFE * A_out/A_total` | 0.4697999895 |
| outlet `latqcc * A_out/A_total` | 0.2032773321 |
| outlet combined, C03 value | 0.6730773216 |
| literal sum of all WAT `latqcc * Area/A_total` | 4.2474719439 |
| literal sum of all WAT `QOFE * Area/A_total` | 2.6633891376 |

The outlet-export convention is plausibly the right one for a hillslope outlet/trench comparand, and it matches existing `totalwatsed3` behavior: `crates/openwepp-runner/tests/totalwatsed3_cli_contract.rs:315-345` verifies PASS `runvol` plus outlet lateral flow. But the canonical ENV-Y formula and C01 wording are too easy to read as summing internal OFE transfer rows. If read literally, the annual tier is not the calculation C03 judged.

Disposition recommendation: amend `INV-SUBHYD-033`/C03 artifacts to define the ENV-Y modeled operand as hillslope outlet export, for example PASS `runvol` plus PASS `sbrunv` or outlet `QOFE`/`latqcc` scaled to total hillslope area. Include the operand reconstruction table in package evidence.

### C03-CX-004 - Accepted Candidate - ENV-T evidence overstates monotonicity and leaves two threshold estimators unresolved

Evidence class: Static + Ran.

`artifacts/verdict.md:28-31` says the median ratio and response frequency "both ascend monotonically" and places commencement at `~10-20 mm`. The table immediately above does not support strict monotonicity: the 10-20 mm bin has median `0.095` and response fraction `0.528`, while the 20-30 mm bin has median `0.092` and response fraction `0.522` (`artifacts/verdict.md:21-24`). The scratch script's own step-fit threshold estimator also reports `threshold_mm: 5.0`, while the package uses the binned median-zero transition instead.

This does not by itself prove ENV-T fails. The table does show a real low-storm median-zero pattern and stronger response at larger storms. But the current prose is stronger than the evidence and does not explain why the step-fit threshold is rejected in favor of the binned transition.

Disposition recommendation: correct the monotonicity statement and explicitly choose the ENV-T estimator. If the verdict remains PASS-with-note, make it a noisy wet-end threshold-shape pass rather than a precise monotone threshold result.

## Accepted Checks

- The C03 correction to C01's over-claim is preserved: C03 is now the event-tier judgment package, not C01.
- The methodological insight that total export is baseflow-contaminated is valid in direction. The scratch run reproduced the false-negative trap: total export produced high small-storm ratios and negative storm-size correlation.
- The Panola 55 mm and WS10 30 mm/quickflow-ratio anchors checked out against the local PDFs/transcriptions. I found no contradiction in those source anchors.
- The external-authority source guards passed independently:
  - `bash tools/release/check_authority_suite_antievasion.sh`
  - `cargo nextest run --test auth11_required_suite_obligation_guards_contract`

## Review Outcome

Do not close C03 as written. I accept the high-level direction: event tiers should use quickflow/event-responsive export, and the field-data posture remains the right authority. But the all-four-tier FARPOINT01 resolution is not review-clean until the package preserves its runner/input evidence, pins the quickflow separation parameter authority, resolves the ENV-Y outlet-export formula, and corrects the ENV-T threshold wording.
