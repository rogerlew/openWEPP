# Verification Agent A

Status: `PASS FOR EXECUTED / HOLD / PARTIAL ADMISSION`.

Evidence mode: **Ran + Static**.

## Independent Checks

| Check | Result | Evidence |
|---|---|---|
| Frozen identities | PASS | Current HEAD, package tool, snowbench binary, direct binary, and freeze SHA-256 values equal the freeze and receipt. The raw results bind the current receipt. |
| Retained authorities | PASS | Rehashed 64 retained anchor, fixture, observation, run-file, and provenance records; no mismatch. |
| Harness inventory | PASS | Exactly four lanes by two models exist. All eight receipt return codes are zero, all eight provenance hashes match, and 200 provenance-bound output hashes rehash exactly. |
| Within-lane parity | PASS | Each model pair has the same fixture, run-file hash, precipitation multiplier, and snowbench binary hash. |
| Direct closures | PASS | Maximum retained production mass residual is `2.220446049250313e-15 m`; maximum Stage-3 energy residual is `6.094342098e-08 J m^-2`, below `1e-12 m` and `1e-6 J m^-2`. |
| Harness closure | FAIL AS DECLARED | Independent CSV reconstruction finds paired maxima of `0.0376727`, `0.04539`, `0.059136`, and `0.0708 m` on the four documented dates. This correctly forces HOLD and withdrawal. |
| Adjudicated counts | PASS | Reapplied the frozen cold-content disjunction per window: Mica `8/23`, Niwot `16/40`, Paradise `0/19`, Snowbird `12/22`. Restricting late-input tests to early peak gaps gives Niwot `0/27` and Snowbird `5/16`. |
| Machine withdrawal | PASS | Adjudicated JSON has null chronology, seasonal-SWE, and albedo fields for the harness. Adjudicated CSV leaves both harness columns empty and marks every lane `WITHDRAWN`; raw files remain separately hash-bound. |
| Claim boundaries | PASS | Human- and machine-facing dispositions prohibit unique causality, independent validation, transferability, fitting, precipitation extension, albedo conclusions, default changes, and promotion. The activation diagnosis is limited to snowbench typed forcing. |
| Figures and sidecars | PASS | Four SVGs parse and render. Withdrawal banners are visible, invalid series are gray, the raw-flag panel is masked, and every figure has a same-stem sidecar with population, units, aggregation, uncertainty, and interpretation limits. |
| Prompt and tool hygiene | PASS | Execution prompt exists only under `prompts/archived/`; no active prompt or package-local bytecode/cache remains. Tool self-check and compilation pass with target-only bytecode. |
| Roadmap/catalog and write set | PASS | Root roadmap, snow campaign roadmap, and catalog consistently route EB-04W2B next and EB-04X after it. Git status contains only those three authorized docs and the new package tree; no protected production path changed. |
| Documentation and whitespace | PASS | `markdown-doc` independently validated 26 package files plus the three authorized roadmap/catalog files with zero errors or warnings; `git diff --check` passes. |

The acceptance failure is legitimate and cannot be repaired inside W2A because
the required correction touches production Rust, contracts, and tests that the
prospective write set protects. The package does not defer that failed current-
scope evidence as a pass: it records the exact blocker, withdraws the affected
science, and names defect-shaped EB-04W2B hold-lift requirements.

## Recommendation

`PASS` the truthfulness and completeness of W2A's terminal `HOLD / PARTIAL
ADMISSION` disposition. Admit only the closed, descriptive direct-production
screens. Do not interpret the snowbench trajectories or albedo contrast, and do
not lift the science hold until EB-04W2B reconciles phase/input authority,
closes the real consumer, and reruns the unchanged frozen comparison.
