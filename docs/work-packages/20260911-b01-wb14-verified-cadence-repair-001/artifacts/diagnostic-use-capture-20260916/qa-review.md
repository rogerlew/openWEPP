# Secondary QA review — diagnostic-use capture corpus

**Reviewer:** secondary QA
**Evidence class:** Ran (read-only offline inspection of retained artifacts and
SHA-256 checks); Static (review of receipts, summaries, commands, and the
previously approved focused-tool test receipts). No Rust/Cargo, model, native
importer, probe, collector, or source execution was performed.

## Findings

### High — the retained regression and the captured runner both remain failed; this corpus is diagnostic evidence only

`raw-launch-terminal.json` records collector exit `0`, but child/runner exit
`101`, `timeout: false`, `runner_execution: FAIL`, and
`observation_complete: true`. `terminal/receipt.json` preserves the same
distinction. This captured runner failure is the E008 cadence failure. It is
separate from the retained preliminary direct-segment canonical-covered
evaluation-budget regression failure on baseline and candidate. A successful
collection/export/inspection pass establishes neither runtime correctness nor
recovery of that retained regression. The full preparation/package disposition
remains **HOLD**.

### No new QA blocker — complete captured stream and required target set are retained

The post-model custody record binds unchanged source tree, binary, and all 13
generated inputs. The terminal collection and inventory command bind the raw
`observations.json` to SHA-256
`1555efe1b592081a3dd67639d71a18e4e67f518e24b93a95ddfe18de2023d87c`,
8,701,419,029 bytes, 123,094 physical rows, complete syntax/EOF, and zero
inventory anomalies. The inventory command exited 0 under its recorded 1 GiB
address-space limit.

The completed export uses that exact source digest and reports eight selected
records, complete syntax/EOF, and 1,796,372,827 output bytes: four committed
day archives (ordinals 27431, 54078, 79222, 107936), the day-4 prepared/provider
context (108033), the exact snow-free pre-child target (123091), and the guard
and caller failures (123092 and 123093). The target selectors are singular for
kind, day 4, interval 22, snow-free phase, and support bounds. This is the
required diagnostic retention set; the inventory also records the 480 broader
provider-phase events that were not part of this narrowly selected export.

I independently recomputed the four retained archive canonical-record hashes
and the target, guard, and caller event-file hashes. They match
`export-summary.json`. The inspection summary structurally checks the archive
entry/canonical-record pairing and reports the corresponding actual lengths:
307,105,003; 376,086,364; 373,891,449; and 273,071,144 bytes. The context
summary records day 4 / interval 22 / transaction 255 and exact guard-to-caller
input and beginning-state byte equality, plus caller beginning-to-working byte
equality. These are structural/provenance checks, not physics validation.

## Non-blocking debt and follow-ups

- `payload-publication-manifest.json` correctly records the compact custody
  boundary: 62 published files totaling 1,176,572 bytes, with each published
  source at most 8 MiB, and five large originals retained locally only. The
  raw 8.7 GB observation stream and 1.8 GB full export likewise remain local
  evidence. Publication consumers need access to those durable local paths for
  the four archive bins and the 457,913,228-byte native-consumer operand; the
  compact manifest is not a substitute for those payloads.
- I independently decompressed every gzip-published payload and matched its
  SHA-256 to the manifest source digest. All gzip copy checks passed; the
  compact copies preserve the recorded bytes while the five large originals
  remain explicitly local-only.
- Both inspectors accurately label the retained target, provider/day, archive,
  guard, and caller material as `present/exported/structurally checked/
  semantically unvalidated`. Large canonical values above the configured decode
  limit are intentionally not reconstructed. Any later claim about physical
  state, restart validity, or cause of E008 requires independent semantic and
  runtime evidence.
- `inspect-summary.json` has a narrow presentation defect: each
  `archive_integrity.day_index` is the literal string `"day_index"`, not an
  extracted numeric day. This does not change the independently matched archive
  hashes or lengths, but that field must not be used as day provenance.
  `structural-findings.json` now supplies the needed bounded correction without
  rewriting the original receipt: its SHA-bound member shards establish actual
  archive days 0, 1, 2, and 3, matching archive-entry day, length, and hash;
  its archive chain and provider joins are true; and all eight capture records
  agree on their physical ordinal, process 996446, and session 1. I reviewed
  those bindings directly. A collector or exporter rerun is not needed for this
  report-only correction.
- The recorded focused tests pass with the approved exporter
  `2e6e0d8c…`, inspector `57b5dc…`, and context inspector `58640828…` hashes;
  `export-tests.json` and `inspect-context-tests.json` each exited 0. The
  context test receipt preserves one earlier fixture-encoding failure before
  its terminal passing run. These focused tool tests improve fixture coverage,
  but do not replace package Rust gates or regression recovery.

## QA disposition

**PASS — scoped diagnostic corpus custody, targeted-record completeness, and
structural provenance are acceptable.** The full work package remains **HOLD**
because the retained preliminary regression is failed and the captured runner
execution also failed (E008, exit 101); this review does not establish semantic
or runtime correctness.

Primary evidence reviewed: `post-model-custody.json`,
`raw-launch-terminal.json`, `terminal-collection.json`,
`inventory/summary.json`, `export-command.json`, `export-summary.json`,
`inspect-summary.json`, `context-summary.json`, `payload-publication-manifest.json`,
`structural-findings.json`, `export-tests.json`, and `inspect-context-tests.json`;
retained raw/export and both inspector directories named in those records.
