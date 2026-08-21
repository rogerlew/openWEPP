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

No closure claim is made: the covered adopter still must produce the shared
carrier and both ending owner candidates in one complete transaction.
