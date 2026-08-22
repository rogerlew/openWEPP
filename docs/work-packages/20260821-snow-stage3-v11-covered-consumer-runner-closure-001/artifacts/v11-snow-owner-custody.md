# V11 snow-owner custody

Required invariant:

```text
authoritative V11 "snow" owner bytes
  = canonical ordered Stage-3 persistent states by lane
hydrology winter-column snow fields
  = checked compatibility projection only
```

`Static:` The attachment now rejects a beginning V11 parent or coupled-time
clock whose `snow` bytes differ from the canonical ordered Stage-3 envelope.
The V11 owner stack also accepts the staged Stage-3 ending envelope as the
ending `snow` owner, and the receipt projection emits that same canonical
payload. Hydrology winter-column bytes remain outside the authoritative
envelope; a full covered-segment compatibility projection is still required.

`Static:` The stable covered adopter now produces the per-lane shared-carrier
receipts, an exact covered `(OFE, tile)` boundary-receipt set, exact Stage-3
boundary/result join, and canonical ending Stage-3 snow envelope before
constructing the V11 candidate. The LSE covered branch now suppresses its
snow-free ground and WB14-facing operators, but complete atomic hydrology
compatibility projection, keyed physical consumption, Stage-3 shortwave/
advection/soil energy custody, terminal parcel custody, restart, and closure
of the V11 lower-surface operator remain open. No Child-1 closure claim is
made.
