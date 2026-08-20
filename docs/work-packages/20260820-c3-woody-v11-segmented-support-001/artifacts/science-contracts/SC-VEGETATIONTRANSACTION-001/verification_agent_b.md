# Authority Verification B — Parent Custody And Restart

Status: **FAIL**

Date: 2026-08-20

Verified exact commit: `cf1fc326d76e9e4c0cbd4c6e1b94febf263878e0`

Reviewed authority commit:
`c53adab0a91c0ecbe853c884bfe05591826441c5`

Evidence class: `Static + Ran + independent adversarial execution`

## Verification summary

The canonical seven-owner aggregate manifest, typed water/NH4/NO3 custody,
complete-owner reconstruction, consuming atomic commit, receipt-body closure,
full V10 source binding, 46-case chronology oracle, and the package's 36
semantic poisons all pass their declared gates. Both contract BEI and unit
lints, all package JSON parsing, the 5/5 Rust authority test, and diff hygiene
also pass. DirectV10 V1 artifacts and coupled-time restart V2 remain
byte-identical to the protected base.

All prior Review B findings are recorded as accepted and corrected in the
review/disposition history. Independent verification nevertheless found one
new release-blocking restart-custody defect.

## Finding

### `V11-TXN-VERIFY-B-001` — BLOCKER — restart does not authenticate the accepted slab prefix

A direct forgery deleted `accepted_slab_receipts[0]` from a canonical
`after_event` `OPENWEPP_C3_WOODY_V11_RESTART_V1` checkpoint while retaining
`next_slab_ordinal == 1`, `accepted_until_ns == 600000000000`, the event and
resource prefixes, and staged owners. `restore_and_continue` accepted it and
committed the uninterrupted ending.

The validator checks event count and resource/material counts against the
cursor, but has no equivalent slab-count join. It then rebuilds the staged
vegetation transition from slab 0 in the supplied complete candidate, allowing
the absent persisted receipt to be silently replaced by external candidate
state. This violates the transaction contract's accepted-prefix authentication
and equivalent-continuation requirements and can conceal lost or contradictory
accepted support chronology.

Required correction: enforce exact slab prefix cardinality, uniqueness, order,
receipt authentication, support continuity, and cursor/ordinal joins; derive
replay from that authenticated prefix; add omission/duplicate/reorder/payload
restart poisons; and rerun the invalidated review/verification cycle.

An additional 28 direct restart forgeries were rejected as required, covering
authority, configuration, sequences, cursor, ordinals, phase, participants,
coupled-time digest, owner manifest, parent/staged owners, staged state,
event/resource/material receipts, scheduled-once state, resources, reduction,
publication/outbox, and parent receipt. The isolated successful omission is
sufficient to fail release.

## Verdict

**FAIL.** Version 4 is not releasable transaction authority at
`cf1fc326d76e9e4c0cbd4c6e1b94febf263878e0`. Production implementation and
authority promotion remain prohibited until the accepted slab-prefix defect is
closed and dual verification passes at an exact corrected checkpoint.

## Corrected-checkpoint re-verification

Status: **PASS**

Verified exact commit: `a7bfbbac57bd2661948ce516cd18fc34e5bd98a8`

Evidence class: `Static + Ran + independent adversarial execution`

This section supersedes the prior FAIL while retaining the finding history.
`V11-TXN-VERIFY-B-001` is closed. The restart validator now joins accepted slab
receipt cardinality to `next_slab_ordinal`; deleting the accepted slab prefix
from the exact `after_event` checkpoint rejects with `V11-RESTART`.

Rerun results at the corrected identity:

- strict BEI and science-contract unit lint: PASS for both contracts;
- all package JSON parsing: PASS;
- chronology oracle: 46/46 PASS;
- semantic custody/restart oracle: 37/37 poisons PASS;
- direct restart-forgery matrix: 29/29 rejected, including omission,
  duplication, payload, cursor, owner, state, resource, reduction,
  publication/outbox, and parent-receipt mutations;
- Rust authority test: 5/5 PASS;
- seven-owner manifest and complete V10 source binding: PASS;
- protected DirectV10 V1 and coupled-time V2 bytes: unchanged;
- protected diff and diff hygiene: PASS.

No transaction custody or restart finding remains.

**Superseding verdict: PASS.** `SC-VEGETATIONTRANSACTION-001` Version 4 may
proceed to authority promotion and the exact preimplementation checkpoint,
subject to Verification A and the package release procedure. Production and
terminal implementation claims remain outside this verification.

---

## Restart V2 amendment Verification B — checkpoint `6c74d866dba776189ec9bc6b8bd62901aecf4917`

Date: 2026-08-20

Status: **FAIL**

Evidence class: `Static + Ran + independent adversarial execution`

Exact tree: `2295f9525ab54ba03eb951be253b3db27eba0300`.

Technical transaction verification passes: the independent reference closes
54/54 poisons and complete continuation digest
`512c259be830ad33de578f9cd26f8931fb334e7b361c4387f8e7562de4f8cf0e`;
all 15 accumulated direct probes reject; exact state/segment/owner/sequence/
event/resource/material/reduction/publication/outbox joins pass; schema meta,
BEI, unit compliance, and authority tests pass (6/6); Restart V1, DirectV10,
and coupled-time restart protected diffs are empty; production activation and
cutover remain excluded.

Artifact identities are shared with the paired vegetation verification:
Restart V1 `79f4d1dd...624b3`, Restart V2 `af9314c3...2441`, poison population
`fa5ae93f...ad34`, reference `13f3d009...f7c`, DirectV10 checkpoint
`c5221657...f842`, and coupled-time restart `37601306...d5de`.

### `V11-TXN-AMEND-VERIFY-B-001` — BLOCKER — transaction amendment disposition is stale

The transaction disposition still identifies `c53adab0a` and Version 4 and
delegates to the equally stale vegetation disposition. Neither file inventories
or dispositions the Restart V2 Review A/B findings, correction checkpoints,
or final Version 7/Version 18 review PASS at `5918d4dbd`. Consequently the
transaction's technical findings are closed in review evidence but not in the
mandatory canonical disposition record.

Required correction: reconcile both disposition files with every amendment
finding and its accepted correction, record the exact final dual-review PASS
identity and no-waiver result, and rerun verification. This is documentation-
only and does not require reopening the technically passing authority.

### Verdict

**FAIL.** Transaction custody and restart behavior pass, but Version 7 cannot
advance while its amendment findings remain undispositioned at the canonical
contract-cycle surface. Promotion and production implementation remain gated.

---

## Restart V2 disposition regression — checkpoint `1a3aa9d7953d03b2be7d7b5ddce3ce4ba9d66087`

Date: 2026-08-20

Status: **FAIL**

Evidence class: `Static + Ran + exact finding-ID reconciliation`

The amendment technical artifacts are byte-identical to the passing authority;
complete continuation, 54/54 poisons, and authority tests 6/6 pass. The updated
disposition records the exact authority/review checkpoints and no waiver.

### `V11-TXN-AMEND-VERIFY-B-002` — BLOCKER — transaction finding families are still not dispositioned by ID

The transaction review issued independent IDs that are absent from both
disposition records:

- `RA-TXN-001..005`;
- `TA-TXN-001`;
- `FA-TXN-001..004`;
- `RVA-TXN-001..004`;
- `RVF-TXN-001..002`;
- `V11-TXN-RESTART-V2-B-001..005`.

The disposition lists vegetation-side analogues only. Referencing that shared
list does not close separately labeled transaction findings under an
all-finding-ID verification requirement.

Required correction: add these exact ranges with accepted corrective evidence
and final `5918d4dbd` Review A/B PASS mapping. No authority, schema, vector,
reference, or production edit is required.

### Verdict

**FAIL.** The technical transaction authority and no-waiver posture pass, but
canonical disposition remains incomplete by exact finding identity. One final
documentation-only reconciliation is required before Verification B PASS.

---

## Restart V2 final bounded Verification B — checkpoint `081a0169634ff30f916f8af5642e5a3c03a4f922`

Date: 2026-08-20

Status: **PASS**

Evidence class: `Static + Ran + exact finding-ID reconciliation`

The transaction disposition now explicitly records and accepts every exact
transaction finding family omitted at the prior checkpoint:

- `RA-TXN-001..005` and `TA-TXN-001`;
- `FA-TXN-001..004`;
- `RVA-TXN-001..004`;
- `RVF-TXN-001..002`;
- `V11-TXN-RESTART-V2-B-001..005`.

It maps each range to the reviewed complete-checkpoint, owner, receipt,
continuation, event/resource/material, reduction/publication/outbox, sequence,
and terminal-owner corrections; records final review closure at `5918d4dbd`;
and explicitly states no waiver. All transaction and shared vegetation
amendment findings are therefore closed and dispositioned.

Technical artifacts are byte-identical to the passing authority. The
independent reference again passes 54/54 with complete-continuation digest
`512c259b...cf0e`; no production or contract artifact changed in this bounded
correction.

### Superseding verdict

**PASS.** Transaction Restart V2 Verification B is complete at `081a01696`.
This supersedes the documentation-only FAILs at `6c74d866d` and `1a3aa9d79`.
Promotion remains subject to Verification A and exact package checkpointing;
production activation remains outside this verification.
