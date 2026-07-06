# Disposition (D10B dual review)

Status: executed
Reviews: Agent A GO-WITH-AMENDMENTS; Agent B GO-WITH-AMENDMENTS. No
CRITICAL findings. All amendments applied in-package before closure.

| finding_id | source | severity | decision | action_taken | artifact_ref | rationale |
|---|---|---|---|---|---|---|
| A-MAJOR-1 | agent_a | major | accepted | frontmatter `contract_version` -> 26 | SC-OFEROUTE-001 rev 26 | version-lag defect class (rev-14 precedent) |
| A-MAJOR-2a / B-M4 | both | major | accepted | INV-011 acceptance wording aligned to the ratified bounded-wobble form the tests enforce | SC rev 26 | contract text must match ratified acceptance + evidence |
| A-MAJOR-2b | agent_a | major | accepted | test-vector row (b) -> TV-transient bound wording | SC rev 26 | same |
| A-MAJOR-2c | agent_a | major | accepted | Algorithm item 4 + INV-007 -> TRUE-celerity CFL (landed scheme) | SC rev 26 | algorithm spec must describe production |
| A-MAJOR-3 | agent_a | major | accepted | material-interface zero-dissipation + boundary mirroring recorded as Algorithm-item-3 departures | SC rev 26 | acceptance-path decisions must be contract-side |
| A-MAJOR-4 | agent_a | major | accepted | Algorithm item 6 -> conservative bin-series handoff wording | SC rev 26 | text matched to the (stronger) landed mechanism |
| A-MAJOR-5 | agent_a | major | accepted | demotion ground 3 requalified (confounded metric, not trace evidence); demotion retained on clean-room + spec grounds | SC rev 26 + oracle-reanchoring-evidence.md | contract must not retain a refuted evidentiary interpretation |
| A-MINOR-6 | agent_a | minor | accepted | BEI-check row appended to gate-results (PASS-DEFERRED, attributed) | gate-results.md | required gate row |
| A-MINOR-7 | agent_a | minor | rejected (stale) | none — Progress/Surprises were already updated (7 checked entries incl. the latent-instability surprise); reviewer read mid-update | package.md | verified current state |
| A-MINOR-8 | agent_a | minor | accepted | TV monitor scope caveat added to residual item 1 | iwagaki-case4-evidence.md | prevents over-reading "essentially-non-oscillatory" |
| A-MINOR-9 | agent_a | minor | accepted | this disposition + verification artifacts complete the set | (this file) | completion tracking |
| B-M1 | agent_b | major | accepted | integer-index bin loops (record_step + integrate_bin_series); regression `bin_loops_progress_on_non_dyadic_sample_dt` | kinematic_wave.rs, cascade.rs | demonstrated fp zero-progress hang witness |
| B-M2 | agent_b | major | accepted | exact-total non-negative forward redistribution of the bin series; fail-closed guard preserved; regression `runon_only_ofe_handoff_is_nonnegative_and_conservative` | kinematic_wave.rs (BinRecorder::finish) | spurious cascade abort on runon-only OFEs; ledger exactness preserved |
| B-M3 | agent_b | major | accepted | span-aware bin integration (RoutingResult.outlet_bin_spans_s; UpstreamHandoff spans); regression `partial_final_bin_handoff_is_exact` | kinematic_wave.rs, cascade.rs | seam exactness no longer conditional on divisibility |
| B-M4 | agent_b | major | accepted | (= A-MAJOR-2a) | SC rev 26 | — |
| B-m5 | agent_b | minor | accepted-as-noted | test retained as a drift guard; enforcement burden documented as resting on the residual-exactness tests | review_agent_b.md | tautology post-rev-24 is by-construction; guard still valuable against future booking drift |
| B-m6 | agent_b | minor | accepted | `scheme_inflow_m2` doc updated to rev-24 semantics | kinematic_wave.rs | stale doc |
| B-m7 | agent_b | minor | accepted | oracle cutoff-fan rationale requalified in code doc (boundary seeding is the trailing-limb carrier; fan = density aid) | iwagaki_oracle.rs | wrong rationale, results unaffected (cross-validation stands) |
| B-m8 | agent_b | minor | deferred | `is_break` exact f64 equality noted; production paths use literal-copy meshes; quantization required if computed per-cell params ever feed meshes | review_agent_b.md | no current consumer; guard documented for the D14-refresh/activation packages |
| B-m9 | agent_b | minor | accepted | structural note added to INV-007 (max_courant evidence <= target by construction; headroom covers intra-step growth) | SC rev 26 | honest evidence reading |
| B-m10 | agent_b | minor | accepted | handoff test comment corrected to bin-series wording | d10b_reconciliation_tests.rs | comment drift |
| B-m11 | agent_b | minor | accepted | upwind-oracle convergence claim scoped (uniform-coefficient theorem; interface case carried by cross-validation) | iwagaki_oracle.rs | doc overreach |
| B-m12 | agent_b | minor | accepted | bin-mean vs instantaneous peak surface note added to Case-4 evidence | iwagaki-case4-evidence.md | surface-pairing hygiene (program lesson) |

No finding rejected on substance except A-MINOR-7 (stale observation,
verified). Post-fix gates: focused `ofe_routing` 64/64; fmt PASS; clippy
0 errors; full workspace nextest re-run recorded in gate-results.
