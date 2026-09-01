# V49 multi-child prepared-install authority implementation and validation

Status: `IMPLEMENTED; DUAL REVIEW APPROVED; CANONICAL R125 PENDING`

Evidence mode: `Static + Ran`

## Exact correction

R124 proved the real `1920..2040 s` fixed-point final install carries three
distinct authenticated roles: mutually equal outer source 42, installed soil
resident/predecessor 43 on `1860..1920 s`, and prepared target 44. V48's
two-ID prepared installer incorrectly required predecessor 43 to equal outer
source 42.

V49 adds opaque
`DirectSoilThermalPreparedBeginningInstallAuthorityV2`. It owns the unchanged
explicit source/soil-target physical authority, the complete validated
authoritative resident including accepted custody, and the complete prepared
beginning. Construction and installation independently reconstruct that value.
The atomic install has one distinct `AuthenticatedPreparedBeginning` posture
that validates the physical authority, resident-to-prepared exact successor,
prepared predecessor-to-resident transaction, and accepted latest-custody
predecessor-to-prepared equality before the existing clone-and-replace install.

Generic installation remains same-ID-only. The retained physical V47/V48 arm
still requires exact `predecessor==source`; it was not broadened. No identity
is derived by adjacency, no outer owner is rebased, and no receipt, state,
carry, support, accepted result, or seal is repaired.

The first independent reviews found two implementation/evidence gaps. The V49
authority constructor now joins the candidate's mutually equal outer source
to the independently validated authoritative beginning's outer source, so a
concerted vegetation/LSE/BGC rebase cannot mint authority. V48 and V49 now
share one private accepted-resident/candidate/no-op validation engine while
retaining their distinct authority reconstruction and atomic-install
postures. The exact R124 positive and no-op vectors execute the real
`install_v2_soil_from_authenticated_prepared_beginning_v1` finalizer helper,
and individual authoritative-resident custody poisons assert full-shadow
rollback.

## Ran evidence

- Expected source red: `faf395ca-82c5-4943-8e7e-271b24e622c2`, 1/2 passed.
- Focused V49 runtime: `8e133d07-70a4-41b9-be21-baf73eecacb3`, 5/5 passed.
- Focused V49 source: `f05e8355-185b-4a08-926e-1d1fb5f70097`, 2/2 passed.
- Retained V39/V46/V47/V48/V49 runtime: `30b12faf-e4a9-4762-a263-720d05e5799b`, 41/41 passed.
- Complete snow source-contract target: `48d26c06-b717-4076-ac12-4489cf37f043`, 42/42 passed.
- Persisted restart: `c95507ab-8088-41c3-be60-0e240e7adaca`, 40/40 passed.
- Orchestrator all-target/all-feature check: pass.
- Authority anti-evasion: pass.
- Required-suite guard: `1e70e29f-4fe1-4a7f-b147-08f8b375f569`, 3/3 passed.
- Workspace formatting and `git diff --check`: pass.
- Exact production diagnostic scan: no V49/R124 probe or `eprintln!` seam.

Post-review corrective reruns:

- Focused V49 runtime: `50aef712-37bf-46e4-9a29-73137b185e7d`, 5/5 passed.
- Focused V49 source: `1e42e517-4452-4567-910a-7d78ea262434`, 2/2 passed.
- Retained V39/V46/V47/V48/V49 runtime:
  `d1cf7bb1-4da8-4cd3-a48f-34b656a1cecd`, 41/41 passed.
- Complete snow source-contract target:
  `208ec63c-40e2-4d66-a264-7e852f0cf9fa`, 42/42 passed.
- Persisted restart: `a90a80f2-81c8-4f94-a7a5-3e5b64c7ae7b`, 40/40 passed.
- Orchestrator all-target/all-feature check, workspace formatting, and
  `git diff --check`: pass.
- Authority anti-evasion: pass; required-suite guard
  `ab0c3fcd-86c0-4aa1-be72-b9604624a3d8`, 3/3 passed.

The focused behavior suite executes the real finalizer with exact
source42/resident43/predecessor43/
target44 support and a further source42/resident44/target45 successor. It also
executes source-owner divergence, foreign resident, prepared target/
predecessor/start/end/receipt/state/layer, accepted target/predecessor/support/
receipt/state/layer/seal, and opaque authority source/target/resident/custody/
prepared substitutions. A concerted mutually equal foreign outer-source rebase
and individual authoritative-resident transaction/support/receipt/resealed
state/layer/latest-custody/seal substitutions refuse. Generic missing authority
refuses; exact success/no-op retains soil/source owner equality, full-shadow
rollback, and publication history.

## Governance

Terminal line counts are recorded in `line-count-governance.md`: production
2,645, retained V10 tests 2,962, exact-move V49 tests 552, and finalizer 2,936.
No exception is requested. Independent Rust correctness and QA reviews remain
mandatory before parent-owned canonical r125. Both final re-reviews are
`APPROVE`: correctness independently passed runtime 5/5 (`2aa4f6af`) and
source 2/2 (`986149c6`); QA independently passed runtime 5/5
(`e98b3616-e941-45e0-ad20-52a13f4258b2`) and source 2/2
(`51ec200e-c28d-4bfa-9587-b49d0de34e15`).
