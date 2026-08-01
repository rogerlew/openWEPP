# Retained-Output Verification Evidence

Status: `PASS`

Evidence class: `Ran`

The independent verifier streamed all 761,212 retained daily records and read
the real WAT `sim_day_index`, `Snow-Water`, and `Snow-Depth` columns. It compared
them row by row with the research trace, runtime aggregates, and complete
serialized layer vectors. The exact frozen 12-lane by four-cell key set, the
48 unique execution-log PASS keys, the runner manifests, and all retained file
hashes agree.

| Independent check | Population result |
| --- | ---: |
| WAT/trace SWE residual | `5.079270337660091e-15 m` |
| WAT/trace depth residual | `8.881784197001252e-16 m` |
| layer density reconstruction | `1.324886011389026e-5 kg m^-3` |
| resolved-layer cold-content reconstruction | `1.862645149230957e-9 J m^-2` |
| layer cold-content sum | `1.862645149230957e-9 J m^-2` |
| enabled longwave paths exercised | `24/24` |
| enabled sublimation paths exercised | `24/24` |

Every numeric operand anywhere in each retained trace record and every selected
WAT value is finite. All five hourly vectors contain exactly 24 values. Layer
counts equal serialized vector lengths. Density agrees with
`rho_w SWE / depth`; resolved thermal layers agree with
`c_ice rho_w SWE (-T)`; and each aggregate cold-content value equals its layer
sum. The canonical unresolved-total boundary (`<=0.001 m` SWE) retains cold
state without requiring an artificial resolved temperature reconstruction.

Negative controls are consumed through the real verifier path. It rejects a
deleted represented fragment through layer-count identity, an aggregate-only
layer substitution through WAT/trace reconciliation, and an enabled latent
path injected into a B cell through selector behavior.

This is explicitly a forensic retained-output seal, not a prospective trace
seal. The direct runner manifests prospectively bind command, source commit,
binary, runfile, WAT hash, completion class, and executed-day count, but do not
hash the opt-in research trace. The added seal binds every trace and verifies
that both modification and inode-change timestamps fall inside the original
result-bearing execution window. That closes retained-output identity for this
characterization package without overstating the runner manifest contract.

The runner manifest and trace also do not serialize the complete inherited
ambient environment or the selected liquid-model string. For EB-04E, identity
is source-and-behavior bound: the frozen launcher overwrites the four
non-target model variables and two target selectors after copying ambient
state; traces serialize the density, melt, and phase models; all 48 show the
expected Stage 3 liquid/energy behavior; and B/L/S/LS mechanism behavior is
exact. This is sufficient for characterization, but it is not an independent
runtime-environment serialization claim. A promotion-bearing EB-04R must
sanitize or snapshot relevant `OPENWEPP_*` variables and serialize the selected
liquid, longwave, and sublimation models in its result-bearing provenance.

Machine evidence is retained in `retained-output-seal.json`,
`retained-output-verification.json`, and
`cmd4_retained_output_verification.log`.
