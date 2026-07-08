# Review Agent A

Static: read the required package, contract, and governance files with line-numbered review.
Ran: `git status --short` and `jq` spot-checks over `artifacts/mesh-ladder-summary.json`; no mesh ladder, cargo, comparator, or doc-lint gates were rerun.

## Findings

### MEDIUM - `dx5` is stated as a passing candidate even though the required fine-reference basis is not adequate

The hold itself is supported, but the package and rev-42 contract overstate what the evidence can prove about `dx5`. The package predeclares that candidate-vs-reference and baseline-vs-reference comparisons are made against `dx2p5` only after fine-reference adequacy is established (`docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/package.md:73` to `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/package.md:76`). The current evidence then records that `mn_corn_h4` fails that adequacy gate on routed-hourly shape (`docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/artifacts/mesh-policy-adjudication.md:21` to `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/artifacts/mesh-policy-adjudication.md:27`; `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/artifacts/mesh-ladder-summary.md:55`).

Despite that, the adjudication labels `dx5` as `PASS` in the candidate verdict table (`docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/artifacts/mesh-policy-adjudication.md:31` to `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/artifacts/mesh-policy-adjudication.md:38`), the hold audit says the ladder "identifies `dx5` as the only tested candidate that passes" (`docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/artifacts/hold-legitimacy-audit.md:10` to `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/artifacts/hold-legitimacy-audit.md:12`, `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/artifacts/hold-legitimacy-audit.md:23` to `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/artifacts/hold-legitimacy-audit.md:26`), and the canonical contract rev 42 repeats that `dx5` passes every real selected-cohort candidate tolerance (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:490` to `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:500`; `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:558`).

That wording is too strong for a gate sequence where the reference itself failed. The evidence supports a hold and supports saying the nominal `dx5`-vs-`dx2p5` deltas are the best observed candidate deltas under the not-yet-adequate reference, but it does not support carrying `dx5 PASS` as a production-promotable candidate fact. Before closure, revise the package and contract language to mark the `dx5` result as conditional/non-promotional until the `mn_corn_h4` reference adequacy blocker is resolved or the adequacy rule is explicitly amended under contract review.

### LOW - `INV-OFEROUTE-013` guard-map evidence still points at the stale rev-39/rescope package

The rev-42 contract text and BEI row record the current readjudication, but the invariant guard map for `INV-OFEROUTE-013` still lists `20260708-laned-router-tier2-dx-target-mesh-policy-rescope-001` as the evidence artifact (`docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:263`). That stale package is the prior run whose WA fine-reference closure blocker was lifted by this package (`docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/artifacts/rev41-ladder-results.md:45` to `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/artifacts/rev41-ladder-results.md:55`; `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:490` to `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:500`).

Update the guard-map evidence cell to include this readjudication package as the current rev-42 authority, optionally retaining the rescope package as superseded context. Leaving the guard map stale creates contract-navigation drift for future mesh-policy validation.

## Residual Risk And Missing Tests

- I did not rerun the 24-rung ladder, cargo gates, comparator harnesses, or doc lint. I only reviewed the artifacts and spot-checked the JSON values used by the markdown.
- The hold is independently supported by the failed `mn_corn_h4` fine-reference adequacy gate and by the absence of runtime-cost ratification for the only nominally passing target-dx candidate (`docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/artifacts/hold-legitimacy-audit.md:34` to `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/artifacts/hold-legitimacy-audit.md:54`).
- The markdown artifacts summarize closure/clamp success, but the per-rung residual maxima live in JSON. Before final package closure, `gate-results.md` should surface the active-closure, rev-40 clamp-source, and rev-41 roundoff-clamp proof required by `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/package.md:171` to `docs/work-packages/20260708-laned-router-tier2-dx-target-mesh-policy-readjudication-rev41-001/package.md:172`.

## Disposition

No production target-dx promotion is supported. The correct high-level disposition is hold, not production `dx5` promotion, but the `dx5 PASS` language and stale guard-map evidence reference should be corrected or explicitly dispositioned before package closure.
