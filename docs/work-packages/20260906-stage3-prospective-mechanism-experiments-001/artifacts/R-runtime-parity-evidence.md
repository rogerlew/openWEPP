# R actual-consumer parity evidence

Ran: independent read-only Python comparison of retained records using reviewed
`admit_identity.identity`, `run_series.record_identity`, and `exact_json`.
Static: traced the frozen callback/assertion source. No builds or runtime
experiments were launched by this reviewer. This is correctness evidence, not a
timing/memory admission or a claim of universal error-domain coverage.

## Scope and identities

Prospective write/read map: this artifact only; root and work-package AGENTS.md
apply. Read `reproduction/A-identity-1-v2.json`, `R-source-04` manifests,
`admit_identity.py`, `run_series.py`, four raw R logs/receipts and the detailed R
LSE trace, and the actual private replay/runner callback source. No source edits.

- R frozen source identity: `a1c128139825c70f48c1df86504f344fb9a0aff082a19645a409a5a64f3bb157`.
- Actual immutable runner SHA256: `5a7619bbdc8644ceb64ba79b7fc6052747398a773d9f2d3a0f6674d9c80c79a8`.
- All four execution receipts report exit 0 and unchanged binary. Identity
  processing separately checks the actual executable and sidecar hashes against
  the record provenance; it does not blindly erase different arm identities.
- Read source bytes match frozen manifest rows: replay tests `638de1d0…`, replay
  implementation `0c557403…`, runner experiment `43c3abec…`.

## Complete record comparison

All four R records exactly equal A `common_identity` after only the reviewed
rules. Compared every `IDENTITY_FIELDS` member: complete input-file hashes,
all seven published output-file hashes, independent closure operands, committed
day/publication/support counts, direct and split trials, accepted microsteps,
balanced/complete qualification flags, lane counts and source/outlet/storage/
clamp volumes, plus the entire normalized output manifest. No scientific field
was dropped to obtain equality. Run-root/timestamp and authenticated executable
provenance use their declared exceptions; the F frame-work rule is independently
bound to actual provider/lane/count operands. Here raw carrier counts also equal
A exactly, so that qualification conceals no R carrier-work difference.

| Retained raw log | SHA256 | A common identity |
| --- | --- | --- |
| `raw/R-admission-01.log` | `6ff7ce43b0a4f9b7390d66f28879e7f37781239297630cdc585bc261fcbfcf1d` | exact |
| `raw/R-forced-complete-01.log` | `fdc418b4d554a8fce564c761498f467981118e6d95023ff5158ac5a8fa0e72a3` | exact |
| `raw/R-node-oracle-01.log` | `7ea9bb011da22ca98ded1ae4d8c9926650526794557acd40536f1c32b937ac8a` | exact |
| `raw/R-trace-01.log` | `6a559ac6c195a934b4006047c24c9c3f0e13d6eb51302ae99aedda77e1a49d2c` | exact |

`exact_json` compares canonical JSON representations, not an approximate numeric
tolerance. Published binary/parquet/evidence files are compared through complete
file SHA256 values; internal node binary64 parity is separately asserted below.

## Actual work and control counts

All populations have 400 maps, 800 solves, 2,000 sweeps, 2,400 iterations;
ordinary/forced/trace have 106,800 probes. Every start has a completion, all
reported error counters are zero, and no event overflow/drops are reported.
Carrier starts/completions are `[72,200,400,400,0]` in every run, errors zero.

| Population | Complete probes | Identity anchors | Component replay | Evaluations | Leaf calls |
| --- | ---: | ---: | ---: | ---: | ---: |
| A identity | 78,800 | 28,000 | 0 | 109,512 | 438,048 |
| R admission and trace | 48,000 | 28,000 | 30,800 | 109,512 | 330,048 |
| R forced complete | 106,800 | 0 | 0 | 137,512 | 550,048 |
| R node oracle diagnostic | 48,048 | 28,028 | 30,826 | 109,690 | 330,792 |

The ordinary R consumer therefore executes real replay (>0), replacing 30,800
complete probes and eliminating 108,000 actual canonical leaf calls versus A.
No convergence/control/output change accompanies that work reduction.

Forced-complete counts are deliberately **not literally A counts**: the scoped
force flag disables existing identity anchors as well as component replay.
Relative to A, exactly 28,000 anchors become complete evaluations, adding 28,000
evaluations and 112,000 leaf calls (four per displaced anchor). The 106,800 probe
population and solve/sweep/iteration population remain unchanged. This stronger
complete oracle preserves the same complete scientific identity; it must not be
misreported as an unchanged A counter vector.

## Node assertions genuinely reached

The executed exact test is
`hillslope::tests::stage3_controlled_mechanism_node_oracle_experiment`, whose body
wraps the same successful consumer in `with_component_replay_runtime_oracle`.
The wrapper asserts final completion flags `(true,true)`. Those flags are set
only by `observe_runtime_base` for actual N=2/S=6 columns with represented Stage-3
lower boundary, once per potential (`caps=None`) and fixed-final (`caps=Some`)
posture, and only after `replay_probe_corpus` returns full-sweep success. An early
matched error returns false and cannot satisfy the wrapper. Thus an ignored-test
exit alone is not being treated as proof: the exact invoked body has a mandatory
successful callback assertion, and the retained runtime counters independently
show 102 additional probes, including 26 additional real component replays.

For those completed sweeps, source assertions compare normalized residual bits
for all admitted signs/all coordinates, canonical stencil-derived dense Jacobian
entries, and for eligible components every explicit evaluation/capture field
against a fresh complete evaluation at the same signed probe. These include raw
residuals, tolerances, normalized residuals, complete occupancy/ground/water/
longwave fields, preparations, liquid ledgers and all four leaf states. Replay
uses `base.capture.as_ref()` from the actual successful production base; the
separate frozen recapture is compared to that original, never substituted for it.
Leaf-call assertions require less real work for replay. Counters are not
suspended: this diagnostic run has 106,902 probes and is excluded from timing and
ordinary lifecycle-count admission. The full-node values are asserted in-process,
not exported as independently replayable per-node raw records.

The ordinary detailed trace contains 608,624 LSE events, `overflow=false`,
`dropped_events=0`, and aggregate counts equal ordinary admission. This review
checked that fact only; full per-event stencil/lifetime/parent graph validation
remains the separately owned detailed trace audit.

## Error and trajectory limits

All four complete consumers succeed; no error DTO was emitted. Their equality
therefore does not independently prove runtime error DTO parity or execute the
natural error rollback branch. Existing focused source-real named-error tests
and immutable-custody/compiler-negative gates remain separate obligations. The
node oracle checks the probe Jacobian but does not itself assert linear-solve
pivot/matrix-norm or every Newton/backtracking trajectory; those are covered by
the separately retained full-solve bit/trace fixture tests. No universal
crossability/noncrossability theorem or unexecuted error guard is claimed here.
