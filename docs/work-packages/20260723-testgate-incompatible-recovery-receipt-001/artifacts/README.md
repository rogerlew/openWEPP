# Evidence

The triggering forest1 observation is run `30026171869`:
`GATE-RESUME-RECEIPT-INVALID` wrapped
`GATE-COMMITTED-CHECKOUT-NOT-EXACT` from stale recovery root
`29991322951-1` after LIGHT 6/6 and a READY 10/10 audit.

Run `30043078267` proved the forest1 execution path and every selected heavy
gate passed. Its hosted verifier exposed a nonportable audit binding: the
forest1 execution-root pathname digest could not equal the hosted archive
extraction pathname digest.

Ran before changed-head dispatch:

- Exact `30043078267` archive replay: `PASS`,
  `trust_class=LOCAL_UNTRUSTED`.
- Relocated audit-root regression: 1 passed.
- Clean public audit reconstruction: 1 passed in 412 seconds.
- Pre-heavy focused run: 33 passed; the already-passed clean reconstruction
  duplicate was interrupted after 91 seconds.
- Verifier suite: 15 passed in 1,553 seconds.

Run `30049926340` failed four reconstruction fixtures because their private
whole-workspace builds were not globally isolated from full-suite linker
pressure; one also used the non-executable checkout target. The retained JUnit
reported the typed planning failure, `Permission denied`, and three
`rust-lld` `SIGBUS` terminations. After moving the fixture root to executable
temp storage and reserving all nextest slots for the repository-snapshot
cohort, the exact full-profile reconstruction passed in 389 seconds.
