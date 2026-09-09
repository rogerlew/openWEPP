# Independent review B — compact cost characterization

Static: read package, authorization, protocol, source map, custody manifest,
collector/analyzer, gate results, cost report, and the isolated shared clock and
carrier bucket mapping. No source edits, builds, simulation reruns, or production
qualification performed by this reviewer. Ran: independent read-only Python
reconstruction directly from all 16 retained JSON receipts, their log bytes, and
the six measured on profiles; did not use the package analyzer for arithmetic.

## Findings

- **Low — QA-COST-DOC-01 — CLOSED after focused rereview:** `artifacts/cost-breakdown.md` contained stale
  summary values despite correct `raw/analysis-02.json`. Independently calculated
  assembly-plus-linear range is **1.2612327096–1.2698368428%**, median
  **1.2675360594%**; the printed 1.265–1.271% range is not current. Inactive-probe
  median is **1.3117015854%**, not 1.314%. Remaining solver median of each run's
  combined children is **0.7488325707%**, not 0.754%; component medians are
  preparation 0.3466851946%, trial 0.2432170269%, remainder 0.1589246280%.
  Current narrative now rounds the retained values correctly: 1.261–1.270%,
  1.312%, 0.749%, and 0.347%/0.243%/0.159%, respectively. Independently reread
  corrected source; no rerun or threshold change was needed.

## Independent results and acceptance assessment

- All 16 receipts are valid with process exit 0; all 16 raw-log SHA-256 values
  match independently read bytes. Six measured on profiles each reconcile
  `runner_ns - sum(exclusive_ns) = 0`, with family totals bounded by each shared
  bucket and identical recorded populations. This is accounting/receipt evidence,
  not an independent rerun of scientific consumers.
- Reconstructed median bucket shares: unassigned **49.23681848%**, mixed physical
  **31.85432430%**, custody/validated state **7.76458490%**, other probes
  **6.34483389%**, assembly **0.76955293%**, linear **0.49654201%**.
  The shared clock pauses enclosing buckets; the report does not add inclusive
  LSE parents to the runner partition. Mixed carrier setup/evidence/completion
  hooks really are mixed, not pure physics or pure serialization.
- Paired observer perturbation independently reproduces wall median
  **+0.10105815%**, range **+0.01518688–+0.36212145%**; process CPU median
  **+0.14738027%**, range **−0.07354878–+0.38517682%**. No overhead correction or
  precision claim beyond these one-host samples is justified.
- Amdahl saving fractions use matched exclusive time: 2x assembly-plus-linear
  acceleration saves **0.63376803%**, 5x saves **1.01402885%**; complete elimination
  has the **1.26753606%** median ceiling. This is a wall-time saving percentage,
  not a speedup ratio. Even eliminating both whole buckets is below the owner's
  approximately 5% criterion; actual reduced-system mapping/guards/work remain.
  Deprioritizing reduced matrices is supported, without a cubic-complexity guess.
- The revised report correctly avoids calling the assembly/linear bound a
  universal coordinate-compaction ceiling. Independently reconstructed per-run
  union with disjoint inactive probes is **2.579401252%** median, range
  **2.572182289–2.584827895%**, matching the new 2.579% / 2.572–2.585% summary.
  That union is still an impossible complete-elimination bound on named buckets,
  not a bound on every residual effect of an unimplemented architectural change.
- Recorded dimension 29 and eligible FD counts 8,000 + 916 + 928 = **9,844**
  agree across profiles. Covered FixedFinal has no FD sweeps. Four inactive input
  coordinates do not mean four derivative-treatment coordinates; identity-anchor
  columns remain separate. Counts alone are not a runtime fraction.
- The current ranking correctly limits the first recommendation to a hypothesis:
  repeated evaluation for identical inputs was **not demonstrated**. Its first
  decisive test must establish that fact before claiming a reusable subset of
  the 31.85% mixed envelope. No assumed 2x/5x gain is claimed for that envelope.
  The larger unassigned region can contain a better target. The denied single
  perf attempt is a legitimate documented refinement limit, not an excuse to
  relabel unresolved work or begin an unauthorized optimization here.
- Collector design retains fixed warmups, balanced paired order, fresh processes,
  CPU binding, exact binary identity and failed receipts; analyzer requires six
  complete pairs and additive partitions. Fixed-size scoped instrumentation has
  generation/lifecycle/order/overflow failure handling and no new off-mode clock
  read. Focused lifecycle and synthetic-error tests are useful but not a general
  proof of arbitrary invalid-physics error equivalence. Actual valid-run output
  neutrality remains the applicable executable obligation and is retained.

## Non-blocking debt and limits

Full-workspace green, prior derivative memory rejection, multi-OFE/year behavior,
production CLI performance and bit-identical rebuilding are not established and
are not silently waived. The bounded observation package does not close inherited
failure populations. Source recovery needs the declared A kit, complete ten-file
patch and external environment; retained source is not a standalone environment.
No optimization, dependency, production Rust or science authority change is
accepted by this review. Terminal dual verification remains a separate obligation.

## Verdict

**ACCEPT the bounded characterization and reduced-system deprioritization;
QA-COST-DOC-01 is closed. Production remains HOLD.**
This is not approval to start the recommended architecture experiment or push.
