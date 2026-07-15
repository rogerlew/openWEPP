# ASSURE-02 Review Finding Disposition

Evidence class: Static + Ran

Disposition owner: parent execution agent

All eleven independent coding-agent findings were accepted. Ten were closed by
ASSURE-02 documentation or evidence changes. The eleventh, the live release-
automation conflict, was removed from ASSURE-02's claimed safety closure and
recorded as named executable blocker `ASSURE03-REL-001`; changing release code
would violate ASSURE-02's explicit documentation-only boundary.

## Review A

| Finding | Disposition | Remediation and evidence |
| --- | --- | --- |
| `RA-001` — claim-evidence record not independently reproducible | Accepted; remediated | Added `groundwater-current-tree-confirmation.md` with the exact twelve-path comparison, command, empty output/exit, exact nextest command, run ID, seven tests, output, and independent arithmetic. Expanded every matrix locator and digest, separated active-router exclusion as `GW-P07`, and mapped each key finding to claim IDs. |
| `RA-002` — formulation-domain boundary omitted | Accepted; remediated | The prototype and matrix now state `Q = kSΔt`, `Δt = 1 d`, finite nonnegative coefficient authority, combined daily export admissibility, and exclusion of negative `ks`/upward lower-aquifer exchange. Limitations state both boundaries. |
| `RA-003` — acceptance tolerances omitted | Accepted; remediated | Methods now give the analytical absolute and both storage-scaled H2637 rules, units, provenance, and interpretation. Results pair observed residuals, allowed residuals, and PASS, and explicitly reject numerical-convergence or solver-error inference. |
| `RA-004` — public open-research assets lack an owner/home | Accepted; remediated | ADR-0038, architecture, lifecycle, source/build contract, report standard, and ROADMAP now require a version-bound public research-object surface, name accountable owners/reviewer, distinguish protected internal objects, and fail publication on absent/stale required safe assets. |
| `RA-005` — internal vocabulary and observational context | Accepted; remediated | The prototype defines OFE, HBP, and `cbase` at first use; gives both tables descriptive titles; and describes Priest River as calibration-conditioned, coupled-model observational evidence that motivates but does not isolate or validate the recurrence. |

## Review B

| Finding | Disposition | Remediation and evidence |
| --- | --- | --- |
| `B-001` — prose-only release hold conflicts with live automation | Accepted; safety claim withdrawn and named blocker recorded | The runbook now says the current script and PR/push workflow are not release-safe or authoritative and prohibits aggregate execution. The migration plan inventories script/workflow/test/docs consumers and defines `ASSURE03-REL-001`: split validation-only CI, fail closed in release mode, add negative tests, then prove zero reports. The implementation roadmap makes it ASSURE-03's first technical gate. No release code changed because ASSURE-02 is documentation-only; openWEPP remains ineligible for release assembly. |
| `B-002` — undefined waiver authority and weak independence | Accepted; remediated | Architecture, lifecycle, and source/build contract now define hard role incompatibilities, a change/decision matrix, required independent scientific/reproduction/publication/steward approvals, old/new-root binding and independence attestations, full rereview for unclear impact, and builder fail-closed behavior. The undefined “impact owner” was removed. |
| `B-003` — synthetic fixture crosses tracked public tree | Accepted; remediated | ASSURE-04D now uses only a confined temporary `usersum`-shaped root, requires tracked-public byte identity, marks fixture snapshots test-only and release-prohibited, and reserves actual tracked promotion for the genuinely reviewed ASSURE-05 report. |
| `B-004` — v1 retired before v2 acceptance | Accepted; remediated | The v1 standard is now proposed retirement under an interim no-new-public-v1 moratorium. Architecture and standards index require one atomic human disposition to accept ADR-0038, activate v2, and finalize v1 retirement. |
| `B-005` — retirement inventory omits live consumers | Accepted; remediated | Migration plan now lists source files, schemas, templates, generated/public pages, root navigation/model narrative, compiler/workspace/lock, integration tests, release check/script/README/workflow/runbook, dormant handoff, and historical records with keep/update/remove action, preservation identity, and a negative zero-report proof. Directory rows must expand to individual files during execution. |
| `B-006` — active package absent from catalog | Accepted; remediated | `docs/work-packages/README.md` records the package under Current Active/Held Packages with its documentation-only scope, current review/verification posture, and human-acceptance boundary; final disposition updates the state without changing that boundary. |

## Boundary Disposition

`ASSURE03-REL-001` is not deferred evidence for a claim that ASSURE-02 closes.
ASSURE-02 explicitly retracts the earlier transitional-release-safety claim and
documents the current conflict. The release-code correction is a normal
implementation step of ASSURE-03 after the v2 direction is accepted. Until it
passes, no openWEPP release candidate is authorized.
