# V39 soil-energy transaction separation implementation and validation

Status: `IMPLEMENTED_FOCUSED_GREEN_CANONICAL_PENDING`

Implemented:

- `PhysicalSoilEnergyTransactionAuthorityV2` carries separate exact nonzero
  source and soil-target transaction identities.
- Physical ingress and every nested receipt remain joined to the source
  transaction; each V2 pre-ingress candidate remains joined to the
  authenticated soil-target transaction and exact support.
- Internal-energy and infiltration debit-credit digests bind both transaction
  identities. Independent receiver cancellation reconstructs the same split
  digest.
- All physical operand call sites derive the soil-target identity from the
  authenticated candidate beginning. No adjacency inference or substitution
  is present.
- Unpublished physical-beginning and same-support fixed-point custody no longer
  receive the outer source transaction. They retain the existing independently
  validated prepared-soil support path and carry that prepared beginning's
  exact transaction through prior-support reconstruction and the unpublished
  physical beginning.
- Initial covered V2 trials and finalization-equivalent private coordinate
  projections now prepare the exact child support only through the resident's
  authenticated next-support chain. Neither path consumes vegetation/LSE
  transaction authority. Coordinate projection additionally requires its
  endpoint seed trial to match the prepared soil transaction and exact support.
- First-child, second-child, source/target digest, support, mixed-target, zero,
  and rollback/no-publication refusal vectors are covered by focused tests.

Ran:

- `cargo nextest run -p openwepp-hillslope-orchestrator v39_`: run
  `29a2fa64-1c20-4239-85fd-e451955dcbd4`, 4 passed after exact
  candidate-owner/state/identity lineage validation and downstream
  continuation custody were added.
- `cargo nextest run --test snow_terminal_enthalpy_event_numerics_contract v39`:
  run `387865b3-287b-4bf1-888d-7d4c57228989`, 3 passed, including the V38
  production seam under the V39 successor-custody supersession.
- `cargo nextest run -p openwepp-hillslope-orchestrator unpublished_physical`:
  run `8ffdffd2-afc3-472b-98bc-14d3cb0364e9`, 3 passed.
- `cargo nextest run -p openwepp-hillslope-orchestrator same_support`: run
  `82051fe6-c9f1-4635-9329-b44baff6a333`, 6 passed.
- Combined `native_v2` plus V38 focused regression: run
  `bdb00484-ee27-4163-8170-c3702b496dd4`, 35 passed.
- V34 focused regression: run `83726d31-d7dd-48c9-84b3-115fc3767cf6`,
  6 passed.
- V35 focused regression: run `c4741253-36ba-4087-9554-ddadbdf0d620`,
  6 passed.
- V36 focused regression: run `54054512-da46-459a-b2e7-df4626643c8b`,
  16 passed.
- V37 focused regression: run `aae1cd28-5620-43b6-8c73-02b92d7cabb5`,
  5 passed.
- `cargo check -p openwepp-hillslope-orchestrator --all-targets`: passed.
- `cargo check -p openwepp-land-surface-energy --all-targets`: passed.
- `cargo fmt --all`: passed.
- `git diff --check`: passed.
- bounded diagnostic scan: no `DFF_V*`, `eprintln!`, or `dbg!` remains in the
  V39 production/test write set.

The unfiltered 22-test snow contract target, run
`75d1d92f-d023-4000-85c5-9ae7a2bb13de`, is not a V39 green claim: 17 passed,
four older V32/V33 source-string obligations failed, and one test was
cancelled. The failures require the absent V32 behavior name and three retained
index phrases; both V39 tests pass independently above.

Canonical r103 advanced through V39 operand construction, then retained the
next downstream defect at `V2 unpublished physical transaction join`: a
prepared second-child soil successor was still compared to the outer source
transaction. Retained log `/tmp/wghl_001d_v39_64m_r103.log`, SHA-256
`66864ee1f3314daaca574bf82ecca3ef58388eee885771931737894cc0fb52fd`;
317.32 s test, 5:34.21 wall, 1,411,364 KiB maximum RSS.

Canonical r104 passed the V8, operand, and continuation joins, then exposed two
remaining raw outer-transaction soil-support callers in the covered initial
trial and finalization-equivalent private projection. Retained log
`/tmp/wghl_001d_v39_64m_r104.log`, SHA-256
`5749d657761615d139576daddad578c2996532695d768b816c06f677216c6959`;
309.11 s test, 5:49.12 wall, 3,561,580 KiB maximum RSS. Both callers now use
the independently validated resident successor chain and the focused-green
correction awaits canonical r105 by the root owner.

Legacy V1 compatibility clarification:

- V39 does not supersede V1 single-transaction posture. V1 source and soil
  transaction identity remain exact and equal; V1 beginning identity and the
  legacy one-transaction digest domains/bytes remain unchanged. Mixed V1/V2
  posture or identity fails typed.
- Focused compatibility/source-target transaction run
  `5f08a672-a9e4-42cc-94da-c3948d6178e6`: 6 passed.
- The first full persisted-restart fixture run after the typed correction
  reached 69/71; the only failures were independently attributed to the V32
  cold-content-export coordinate omission, not transaction custody.
- After the contract-first V42 correction, full persisted-restart fixture run
  `a11b4e2f-249d-4b62-b17d-98a5b39336b5`: 71 passed, 0 failed, 0 skipped in
  444.519 s. This closes the required V1/V2 adoption regression without
  weakening either posture.
