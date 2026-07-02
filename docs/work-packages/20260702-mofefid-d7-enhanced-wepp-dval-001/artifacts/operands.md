# D7-S1 — Operand Completion

Evidence: **Static** — authoritative operands from `3.1_Validation_Input.docx`
+ paper Table 1; Green-Ampt soil parameters texture-derived from Rawls,
Brakensiek & Miller (1983) class-average tables. Soil params are **operands of
record**, encoded in `ofe_routing::dval`. Verdict class per case below.

## Fully-specified operands (docx / Table 1)

| Case | I (mm/h) | S₀ | plot L×W (m) | k_o | roughness | duration |
|---|---|---|---|---|---|---|
| 1 bare | 60 | 9% | 7.5 × 1.2 | 500 | skin | 5 h |
| 2 isolated | 74 | 2.2% | 6 × 1 | 500 | Cd=1, Dr=0.06, λ=0.2 | 3 h |
| 3 vegetation | 74 | 7% | 6.1 × 1.8 | 500 | LAI=1, hc=0.1, Cd=1 | 1 h |
| 4 shock | — | 2/1.5/1% | 24 × 0.196 flume | **unspecified** | lateral 0.108/0.0638/0.08 cm/s, 10 s |

## Green-Ampt soil parameters (texture-derived — operand gap filled)

Cases 1-3 need `Ks`, `ψ`, `Δθ`, none in the paper. Derived from soil texture
(Rawls et al. 1983):

| Case | soil (texture) | Ks (mm/h) | ψ (m) | Δθ | source |
|---|---|---|---|---|---|
| 1 | Tama (5/26/68 sand/clay/silt → silt loam) | 6.8 | 0.167 | 0.35 | Rawls silt-loam class |
| 2 | 41% sand + gravel | 20.0 | 0.11 | 0.30 | Rawls sandy-loam-ish |
| 3 | Miami silt loam | 6.8 | 0.167 | 0.35 | Rawls silt-loam class |

**Uncertainty is material.** Case 1's `NS_trace` is sharply Ks-sensitive
(0.868 at Ks=6.8; −0.51 at Ks=2; 0.37 at Ks=10 — see execution-report). The
literature value 6.8 mm/h happens to be NS-optimal, which is a genuine (not
tuned) signal, but the verdict is **operand-sensitive**. Cases 2-3 soil params
are looser (sand/gravel and Miami series unspecified in-fixture).

## Case 4 flume k_o (operand gap, unresolved)

The paper gives **no** friction coefficient for the smooth Iwagaki flume.
A k_o scan under the CORRECTED zero-intensity forcing (Iwagaki has no rain;
the earlier runs wrongly fed the lateral rate into the skin-term `I`) shows the
timing and rise shape reproduce at k_o ~ 200 (t_peak 28 s vs 26 s; rise 20.6 s
vs 20.9 s), with the residual being peak magnitude (~20% low) and moderate
`NS_trace` (~0.30). The shortfall tracks the unspecified flume `k_o`. Verdict
class: **operand-limited** (the earlier solver-side attribution was withdrawn
as a forcing-bug artifact — see execution-report).

## Verdict-class rule applied (package acceptance §S1)

- Case 1: operand-sensitive (Ks) → verdict qualified, not robust.
- Case 2: operand-loose (Ks/gravel) → `operand-limited`.
- Case 3: operand-loose + S0 magnitude caveat → `operand-limited` / shape-only.
- Case 4: operand gap (unspecified flume k_o) **is** load-bearing — at k_o~200
  the timing/rise reproduce; residual peak/NS tracks k_o → `operand-limited`.
