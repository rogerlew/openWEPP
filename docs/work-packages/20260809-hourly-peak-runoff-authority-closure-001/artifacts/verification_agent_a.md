# Independent Terminal Verification A

Status: `complete`

Evidence class: `Static + Ran lightweight verification`

Candidate commit:
`d5320fafbe15001a64d9d0ba2274f527c36a61ba`

Implementation/contract/test commit:
`33831787b7029b28b0716c8458f08a11899db446`

Verdict: `PASS`

I did not read the other terminal verifier's artifact and did not rerun the
full workspace or Topanga cohort.

## Findings

No open finding remains.

### `VERIFY-A-MEDIUM-001` — accepted and closed

The initial closure candidate marked `worker-handoff.md` complete while still
instructing a worker to close the three defects that the disposition already
reported closed. Candidate `d5320fafb` corrects the handoff: all three defects
are closed through real production/publication consumers, no successor is
required, and only package-local terminal receipts plus prompt/status archival
remain. The correction is lifecycle-only and does not change executable,
contract, or test bytes.

## Identity, Diff, And Lifecycle

- The worktree was clean at candidate `d5320fafb`.
- `d5320fafb` descends through closure-evidence commit `e0e6d9e7c` directly
  from exact implementation identity `33831787b`.
- The complete base-to-candidate diff from
  `a65cc3973ddd04b07cad108fcb33d83a8c161abb` contains 111 paths. Independent
  comparison against `owned-file-manifest.md` found 111/111 exact matches,
  with no actual-only or manifest-only path.
- The implementation-to-candidate delta contains only package evidence,
  lifecycle/catalog/backlog text, execution receipts, and the two queued
  verifier files. It contains no production Rust, contract, test, Cargo, or
  runtime-input change, so heavy evidence remains bound to `33831787b`.
- Package, disposition, catalog, backlog note, and tracker consistently report
  a closure candidate with terminal verification pending. The active kickoff
  remains active until this final phase is reconciled.

## Contract, Runtime, And Real Consumers

Static inspection confirms one aligned authority chain:

1. `SC-WATBAL-001` defines the closing 24-bin post-partition depths, maximum
   hourly mean depth rate `max(q_hourly / 3600 s)` in `m/s`, modeled-hour
   surface return, rectangular-equivalent duration, fail-closed missing
   custody, and exactly-once area conversion.
2. `direct_runtime/runoff.rs` combines WB14 excess with WB19 hourly saturation
   carry, rejects positive runoff without a closing hourly ledger, and computes
   WB16 peak and duration from that series. Routed melt/runon enter WB14 supply
   and are not appended as later runoff limbs.
3. `SC-SED-001` and `direct_runtime/erosion.rs` consume the internal `m/s`
   maximum-hour operand and seconds-dimensional duration custody; they do not
   consume public `m3/s` or authorize an APPMTH/rainfall-window/uniform fallback.
4. `direct_runtime/01_publication.rs` applies
   `peak_runoff_rate_m_s * area_m2` once. PASS metadata names `m^3/s` and
   “Maximum hourly mean runoff flow.”
5. The real p61 and routed multi-OFE p102 tests join the produced HBP event to
   the public Parquet day, independently reconstruct
   `max(hourly_runoff_volume_m3) / 3600`, compare that with both published
   peaks, and verify hourly volume closure. This is real downstream-consumer
   evidence rather than producer-only or formula-tautology evidence.

The supported claim is therefore maximum hourly mean hillslope runoff flow.
It is not an instantaneous/subhourly peak, legacy-parity result, calibration,
observed-flow validation, or routed watershed/channel-flow claim.

## Complete Topanga Evidence

Static evidence plus independent read-only reconstruction verified:

- release binary SHA-256
  `ac8790faf32a5b98993427b636084c04ba468955458c4fc18f3874cea709c4c3`;
- frozen plan SHA-256
  `32e6f5e99a77747fcdd93388302f2a5ffb496a87b764ac4505e09691955db756`;
- 280 baseline receipts and 1,088/1,088 trial receipts, all marked `ran`;
- 1,913,199 paired event rows and 1,913,158 finite-positive peak pairs;
- zero invalid maximum-hour fractions;
- zero runoff/peak zero-topology mismatches;
- maximum ratio-decomposition residual
  `4.440892098500626e-16`;
- peak-ratio maximum `12965889426731.332`, p99
  `1.0000000000000002`;
- maximum-hour-fraction-ratio maximum `2.755595239734283`, p99
  `1.0000004332094452`; and
- zero event pairs with runoff volume within 5% and peak changing by at least
  2x.

I recomputed those metrics vectorially from the retained 1,913,199-row Parquet,
not from the package summary. All values exactly match `summary.json`. The
extreme raw peak ratio is bounded to a near-zero denominator by the independent
volume/shape decomposition; the evidence supports only the stated absence of
an unexplained volume-stable discontinuity.

## Workspace, Inventory, And Other Gates

- The retained exact-implementation full receipt has run ID
  `2a4b4f2c-d6c6-4bd6-a22f-e61bdb8f4576` and terminates:
  2,346/2,346 passed, 47 slow, 33 ordinary skips, 8,454.483 seconds.
- Independent inventory comparison found 2,346 full identities, 2,297 quick
  identities, zero quick-only identities, and 49 full-only identities. Thus
  quick is an exact subset of the admitted full receipt; no quick-selected test
  is missing.
- The retained workspace doctest command exits 0 and contains zero executable
  doctests.
- Retained affected-crate warnings-denied Clippy, focused authority/consumer,
  H2637, format, documentation, line-count, and authority anti-evasion receipts
  are PASS. Historical interrupted or failing attempts remain explicitly
  non-admitted and their defects are dispositioned.
- Both independent science reviews, Rust correctness review, and Rust QA bind
  `33831787b` and report PASS with no open implementation blocker. Later full,
  cohort, and lifecycle evidence fulfills the package-closure obligations that
  those implementation reviewers correctly left to terminal disposition.

## Lightweight Commands Run

Ran from `/home/workdir/openWEPP` without rerunning a heavy gate:

```text
git diff --check a65cc3973..d5320fafb
PASS

cargo fmt --all -- --check
PASS

bash tools/release/check_authority_suite_antievasion.sh
PASS

.venv/bin/python -m unittest \
  docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001/tools/test_topanga_openwepp_census.py
PASS: 6/6

markdown-doc lint --path \
  docs/work-packages/20260809-hourly-peak-runoff-authority-closure-001 \
  --format plain
PASS: 28 Markdown files, 0 errors, 0 warnings before this receipt
```

Additional read-only checks reproduced the binary and plan hashes, record
counts, Parquet metrics, full/quick inventory relationship, exact manifest
path set, full receipt summary, and clean candidate identity.

## Disposition

`PASS` — the exact implementation and candidate identities, terminal write
set, lifecycle state, contract/runtime/public-consumer chain, complete 1,088
trial evidence and bounded claim, 2,346-test receipt, inventory reuse,
doctest/format/Clippy/anti-evasion/review evidence, and finding disposition are
coherent. No unresolved current-scope gate or severity finding prevents final
package reconciliation.
