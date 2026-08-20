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
