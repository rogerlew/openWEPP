Status: controlling results recorded 2026-09-08 PDT.

Ran: controlled fresh-process campaigns on CPU 0. Production remains HOLD;
these are differential experimental results, not release qualification.

| Arm | Science/admission | Timing | Memory and teardown | Scale |
|---|---|---|---|---|
| A | exact protected baseline; admission PASS | median 4.9906 s in final F comparison; 4.9604 s in R comparison | lifetime peaks about 76–79 MiB; variable post-drop RSS | 10-OFE admission executed |
| F | protected outputs/closure/custody match on the admitted one-OFE workload; providers/carriers 400→200; frames 1205→805 | median 3.8794 s; 22.44% gain, 95% CI 22.10–22.58%; 1.1207 s saved (95% CI 1.1016–1.1281 s); CPU ratio 0.776 | no material peak increase; ten-run peak 84.6 MiB vs A 86.1 MiB; no consistent post-drop growth | NOT ADMITTED: 10-OFE fixture recorded provider=carrier=0 and identical D=5330; 19 OFE NOT RUN |
| R | protected outputs/closure/custody match; positive replay and complete stencil/oracle evidence | median 5.2413 s vs A 4.9604 s; 5.72% regression, 95% regression interval 5.45–6.03%; seconds-saved CI -0.2983–-0.2708 s; CPU ratio 1.057 | neutral/slightly lower peak; no consistent teardown growth | NOT RUN: failed competitive timing predicate |

The first resumed A/F warmup stopped before measurements on a shared-cwd
provenance defect. The corrected collector freezes a cwd per arm. A later common
scale assertion correction changed only OFE>1 validation; the final A05/F06
12-pair timing series controls.

All 12 individual paired wall differences/ratios and all six lifecycle records
are retained in the named `artifacts/raw/resume-20260908-*-results.jsonl` files.
F memory pair peak deltas are -2.6 to -1.1 MiB; post-drop deltas are -11.0 to
+9.1 MiB. R peak deltas are -0.5 to +0.5 MiB; post-drop deltas are -9.1 to
+8.2 MiB. Across three ten-run processes, F last-minus-first post-drop change
was -10.9 to +5.6 MiB versus A -3.0 to +17.0 MiB; R was -1.9 to +5.7 MiB versus
A -6.1 to +16.0 MiB. This ten-run horizon does not establish long-run boundedness.

At one OFE, the active engineering peak ceiling is 144 MiB and both arms remain
below it. The post-return allowance is 9 MiB; both baseline and treatments remain
well above it (roughly 47–79 MiB above pre-fixture), so that inherited engineering
requirement remains FAIL and is not experimental admission.

F: `USEFUL_ARCHITECTURE_BUILDING_BLOCK`. Its one-OFE gain is well bounded, but the
prescribed scale fixture does not exercise the treatment.

R: `REJECTED`. It is scientifically admissible and memory-neutral, but slower.
Retain its dependency graph and stencil oracle as future validation assets.
