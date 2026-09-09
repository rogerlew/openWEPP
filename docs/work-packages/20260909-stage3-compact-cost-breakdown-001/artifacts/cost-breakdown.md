# Compact one-OFE cost breakdown

Ran: six balanced fresh-process off/on pairs after two warmups per posture, CPU0,
the same observation-capable P executable. All 16 runs completed; protected
inputs, outputs, closure operands, control counts, and solver counts matched the
admitted A identity. Each on profile reconciled with accounting difference 0 ns.

Median runner time was 4.944 s off and 4.951 s on. Paired observer perturbation
was +0.101% wall (range +0.015% to +0.362%) and +0.147% process CPU (range
-0.074% to +0.385%). Costs are raw on-arm exclusive time; no overhead subtraction.
Measured full-process medians were 4.982 s off / 4.983 s on wall and 4.960 s off /
4.967 s on CPU; fixture/test-harness work is outside the runner denominator.

| Phase | Basis | Median time / share (range) | Interpretation |
| --- | --- | --- | --- |
| Unassigned runner | exclusive remainder | 2.439 s / 49.24% (49.18–49.29%) | Honest unresolved execution; not labeled removable. |
| Mixed physical/construction envelopes | exclusive | 1.577 s / 31.85% (31.78–31.90%) | Native-ET, terminal-provider carrier, covered-carrier physical setup/evidence/completion, and imported physical/frozen envelopes, excluding measured children. |
| Custody + validated state | exclusive, merged | 0.385 s / 7.76% (7.74–7.82%) | Receipt, encoding, projection, publication; not pure serialization. |
| Other Jacobian probes | exclusive LSE child | 0.314 s / 6.34% (6.33–6.36%) | Non-target FD residual calls. |
| Orchestration/setup | exclusive | 0.072 s / 1.46% (1.45–1.47%) | Runtime setup and validated-state blocks. |
| Inactive sun/shade probes | exclusive LSE child | 0.065 s / 1.31% (1.31–1.32%) | 9,844 executed FD columns. |
| Matrix construction/adjustment | exclusive LSE child | 0.038 s / 0.770% (0.767–0.772%) | Allocation, arithmetic, scaling, row/RHS adjustment. |
| Linear solve | exclusive LSE child | 0.025 s / 0.497% (0.494–0.500%) | Factorization/pivot/substitution. |
| Remaining small solver work | exclusive LSE children | 0.037 s / 0.749% | Preparation 0.347%, trial 0.243%, remainder 0.159%. |
| Routing | exclusive | 0.0008 s / 0.015% | One Lane-D day plus bookkeeping. |

LSE rows are a drill-down placed directly in the exclusive runner partition; no
inclusive LSE parent is added. Raw receipts and reconstruction are under `raw/`.

## Reduced-system decision

Every exercised system dimension was 29. Four structurally eligible inactive
coordinates occurred in Covered Potential (8,000 eligible FD columns), Covered
FixedFinal (no FD sweep), GenericV3 Potential (916), and GenericV3 FixedFinal
(928). Identity-anchor columns are separate and not candidate savings.

Assembly and linear solving together consumed a median 1.268% (range
1.261–1.270%). Eliminating both entire buckets cost-free caps the assembly/linear
effect at 1.268%; assumed 2x and 5x acceleration save 0.634% and 1.014%. This is
not a universal coordinate-compaction ceiling. The disjoint inactive probes add
1.312%, making the still-impossible union of all three full buckets a median
2.579% (range 2.572–2.585%) ceiling. Real work must retain mapping,
reconstruction, guards, some factorization, and unmeasured residual effects.
Reduced linear-system assembly is **deprioritized as the primary throughput project**.

The prior inactive-column target itself consumed 1.312% here and only part of
that bucket is removable. This explains why a 9.44x local acceleration can yield
a sub-percent whole-run improvement: counts overstated consequence.

## Ranked next experiment

1. Prototype a typed single-pass physical-result boundary across terminal
   provider, covered carrier, and imported frozen-evaluation consumers. The
   measured 31.85% mixed envelope is only an inclusive upper bound: this package
   did not demonstrate same-state duplicate evaluation, and much construction,
   validation, and custody must remain. The first decisive test must prove whether
   any authoritative physical result is actually recomputed for identical inputs,
   then preserve exact physical/result/output identity while independently
   reconciling invocation counts. Until that proof, achievable saving is unmeasured
   and no 2x/5x saving claim is made.
2. Resolve the 49.24% remainder before selecting a broad runner refactor. It may
   contain a larger competitor, but this package cannot name it honestly.
3. Other FD probes have a 6.34% perfect ceiling. They outrank reduced matrices
   but require separate derivative/dependency authority.

This is one authenticated one-OFE day, one host, same-thread scoped timing, and
an audit-capable test executable—not a production CLI or multi-OFE/year result.
The single sampling refinement was attempted, but `perf_event_paranoid=4` denied
all events; no privilege change was made. Inclusive native-ET and physical-
evidence timers were not relabeled removable. The recommendation is qualified
because both the mixed envelope and unassigned work could hide larger or smaller
opportunities than their labels suggest. Production remains HOLD.
