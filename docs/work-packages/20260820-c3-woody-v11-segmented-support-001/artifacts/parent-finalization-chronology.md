# Parent Finalization Chronology

Status: authority candidate

`parent beginning -> accepted segment/event chain -> complete ending candidates
-> cumulative ledgers -> one material batch -> one parent receipt -> consuming
atomic complete-owner commit -> checked sequence increment`.

Every segment predecessor and support is exact. Scheduled receipts are unique.
Finalization is consuming and bound to the live coupled clock/owner beginning;
it cannot run twice, publish early, or install a partial owner set.
