# Independent terminal verification B

Static: root/work-package governance, `role-verification.md`, testing strategy
sections 9--10 and 18, package/handoff/final/finding dispositions, both terminal
reviews and focused re-reviews, protocol, treatment/results/gate records,
source/build manifest, architecture handoff, collector/analyzer source, and the
current exact package status. Ran: independent read-only reductions, inventory,
hash, manifest, collector-test, and scale-log checks listed below.

Role/session: terminal verifier B; configured/requested effort: high; effective:
UNOBSERVED. Reviewed identity: dirty uncommitted postmeasurement cut observed
2026-09-08, source/build manifest status `postmeasurement-review`, protocol SHA-256
`40033ea2...f115f`; assigned scope: commands, timing/memory reconstruction,
negative controls, evidence inventory, scale applicability, exact status and
non-deferral truthfulness.

## Independent checks

- Ran: from `artifacts/reproduction`,
  `/workdir/openWEPP/.venv/bin/python -m unittest test_collectors.py`; exit 0,
  PASS 8/8. This exercises identity/path poisons, failure retention,
  prespecified statistics, lifecycle validation, stencil boundaries, and
  synthetic (non-admitting) scale arithmetic.
- Ran: `.venv/bin/python analyze_series.py <durable timing JSONL> | jq <fields>`
  for both timing series; each exit 0. A05/F06 independently reduces to 12
  pairs plus four warmups, medians 4.9905795/3.879376 s, 1.1207305 s saved,
  improvement 0.2243838724, improvement CI 0.2210093289--0.2258312125,
  seconds-saved CI 1.101593--1.1280925 s, and CPU ratio 0.7760731939. A04/R05
  reduces to 4.9603805/5.241341 s, -0.2840685 s saved, -0.0572410830
  improvement, interval -0.0603427221---0.0544947388, seconds interval
  -0.2983---0.2708 s, and CPU ratio 1.0567202621. The treatment report agrees.
- Ran: independent Python parsing of the four durable lifecycle series; exit 0.
  Row inventories are 12/6/12/6 for F-memory/F-teardown/R-memory/R-teardown,
  all valid and arm-balanced. Paired lifetime-peak deltas are F -2620 to -1100
  KiB and R -472 to +488 KiB. Ten-run last-minus-first post-drop changes are A
  `[16976,4428,-3048]` versus F `[5552,-2396,-10916]`, and A
  `[16336,-188,-6252]` versus R `[-1972,5824,2116]` KiB. These support the
  reported bounded `no consistent growth` statement, not heap ownership or
  long-run boundedness; the report states that limit.
- Ran: counted all six controlling package-local JSONL files: 28+12+6+12+6+28
  = 92 rows. `sha256sum` exits 0 and reproduces the review-B hashes, including
  timing `313d3ceb...e83e91c`/`9aee044a...884a582`, lifecycle
  `208eb2f6...664b2c`, `c3341d4d...6e2d4ced`, `5e5e7465...de4453d0`,
  `fa43fad4...e946e129`, and freezes `bfc7b0ee...6ca11f`/
  `dd6d640a...e2e4`.
- Ran: `sha256sum` on A05/F06/R05 `source-manifest.json`; exit 0 and exact
  agreement with source/build manifest hashes `aa1abb4b...fc1b`,
  `d9fa2541...0ecd4`, and `275da2b1...9ecc`. Unique executable hashes embedded
  in durable timing rows agree with the terminal manifest for A05, F06, and R05;
  A04 is separately and truthfully identified in the handoff as R's baseline.
- Ran: parsed both corrected 10-OFE admission logs; exit 0. Each contains one
  OFE=10 controlled record with D=5330 and provider/carrier completed counts
  zero (the non-provider counters are `[0,1920,0,0,64]`). Thus the authentic
  fixture fails the protocol's positive matched-native-work applicability rule.
  `rg` confirms 19 OFE is consistently `NOT RUN`, never PASS, in compact results,
  treatment, gates, and final disposition.

## Finding

`ST3-VB-001` -- LOW -- `artifacts/results.jsonl:1-2`: the compact timing rows'
`raw` fields still point to mutable `/tmp/.../results.jsonl` locations, although
the complete controlling series are now retained package-locally and correctly
named by `treatment-results.md`. Correction: replace those two pointers with the
durable package-relative paths. This is a navigation/provenance inconsistency;
the retained rows, hashes, manifest identities, calculations, and HOLD decision
were independently recoverable without those fields, so it does not overturn
the bounded verdict.

## Disposition and verdict

Accepted review findings ST3-B-001 through ST3-B-003 are fixed and independently
rechecked. Non-deferral is truthful: F met the one-OFE competitive predicate,
but its mandatory 10/19-OFE scale obligation remains unmet because the available
authentic fixture observes P=0; 19 OFE is explicitly NOT RUN, with the next
authorized Stage3 package named as owner and an authentic native-path fixture as
trigger. R scale is legitimately not selected after its competitive predicate
failed. Full-workspace, broad-Clippy, release-golden, and post-return ceiling
failures remain visible; production remains HOLD.

Uncertainty: expensive controlled processes and broad failing gates were not
rerun. Runtime configuration is UNOBSERVED. The cut is dirty/uncommitted, so
subsequent substantive byte changes require bounded freshness reconciliation.

Verdict: **PASS for the evidence-backed, truthful `EXECUTED HOLD` disposition**,
subject to the non-blocking stale-pointer finding above. This is not COMPLETE,
production/release qualification, a standalone-F recommendation, multi-OFE
scale evidence, or long-run memory proof.
