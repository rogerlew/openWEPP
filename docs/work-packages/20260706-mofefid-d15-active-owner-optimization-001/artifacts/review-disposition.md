# Review Disposition (D15A-P5)

Status: **EXECUTED**. All findings from both reviews dispositioned; every
`accepted` finding is FIXED and re-verified (see the post-review table in
`gate-results.md`).

| Finding | Disposition | Resolution |
|---|---|---|
| QA-H1 (completion claimed while gates unresolved) | **accepted (sequencing)** | The two IN-FLIGHT gates resolved PASS (workspace 1410/1410; ignored pair 2/2) before disposition; review/verification/disposition artifacts now exist; `worker-handoff.md` tense corrected. The catalog updates (ROADMAP/README/strategy) preceded review completion — acknowledged as a sequencing defect in this package's process; their content is now true on the final tree. Process lesson recorded in `final-disposition.md`. |
| QA-H2 (self-closing closure checks; no independent reconstruction) | **accepted** | FIXED in code: the SEAM cross-ledger hard-fail (`|injected_solver − Σ q_runoff·A|` ≤ 1e-9 rel, independent ledgers) restored + hourly forcing breakpoints make it exact; day identity re-formed on the soil-side release; (b) relabeled router-internal; R4B-tautology honesty note added to contract + lineage. Its introduction caught a real ~0.11 % booking error. Independent produced-output reconstruction added: `total_latqcc_outlet_m3` ≡ published `sbrunv` sum (1 ulp). Measured seam max 5.0e-14 over 610 days. |
| QA-M1 (mesh-basis conversion unrecorded in contract) | **accepted** | FIXED: rev-27 mesh-basis rule added to the contract tolerance notes; implementation-artifact claim now true. |
| QA-M2 (latqcc "in closure" text vs code; double-count formula) | **accepted** | FIXED: contract status text + operand-lineage formula corrected (latqcc INSIDE subsurface_loss; surfaced separately as evidence operand); no code change needed beyond the CR-L1 scope fix. |
| QA-M3 (uniform fallback consumed in production without amendment) | **accepted** | FIXED: rev-27 D12-row disposition — counted production residual class, no fidelity authority; fail-closed alternative rejected (kills valid climates). |
| QA-M4 (erosion water-magnitude follow-on mis-attributed) | **accepted** | FIXED: the follow-on gate is now recorded IN the contract (rev-27 status text, three named gates). |
| QA-M5 (consumer-path rubric rows missing) | **accepted** | FIXED: all named elements now present per consumer in `consumer-path-proof.md`. |
| QA-L1 (line counts stale; WARN-band file undispositioned) | **accepted** | FIXED: measured counts + explicit WARN acknowledgment for `00_builders_and_authority.rs` (2,732; +36 additive; < 3,000 bar). |
| QA-L2 (`total_source_m3` naming) | **accepted** | FIXED: with the seam check, solver-booked = soil-released within 1e-9 by enforcement, and the lineage text now says so. |
| QA-L3 (log retention; window wording; gate flag wording) | **accepted (minor)** | All three seam-fixed timing logs retained in `logs/`; window wording matches the D14 form (contract text unchanged — the implemented `(h+1)·3600` IS "the end of the last active hour"); `--no-fail-fast` retained in the handoff (it strengthens the package's `--no-capture` form; both recorded). |
| CR-M1 (env mutation unsound under threaded cargo test) | **accepted** | FIXED: harness contract documented at file top (nextest-only); SAFETY comments now accurate for the neutralization pattern. |
| CR-M2 (sibling env var not neutralized) | **accepted** | FIXED: every run helper removes the sibling selector at entry. |
| CR-L1 (latqcc routed-days-only scope) | **accepted** | FIXED: terminal-lane latqcc recorded on zero-source days too; manifest total now all-days and reconstructs `sbrunv`. |
| CR-L2 (no day coordinates in closure failures) | **accepted** | FIXED: day index in all three hard-fail details. |
| CR-L3 (OPT-5 finite-h_pow boundary) | **recorded** | No change: physically unreachable; the plan artifact already scopes the claim; kept on the record. |
| CR-L4 (test comment overstates in-test witness) | **accepted** | FIXED: comment now states the SHA comparison lives in the P4 evidence. |
| CR-INFO (zero-source lanes route as pass-throughs) | **recorded** | Cost-profile note; feeds the T3 implicit-stepping package (the recession/pass-through phases are exactly its target). |

Both reviewers' verdicts were GO-WITH-AMENDMENTS; all amendments are applied
and re-verified. No finding was rejected or deferred.
