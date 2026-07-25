# No-Recollection Proof

Evidence class: Static and ran.

`inspect` contains no coverage, CRAP, test-execution, transition, or CQR-batch
launcher. It may invoke the adopted verifier's `cargo nextest list` inventory
enumeration and associated instrumented compilation for exact-current
verification.

Every intake receipt records `collection_launched=false`. `CURRENT` is rejected
by `authorize-recollection`. A typed noncurrent receipt is accepted only when:

1. its exact locators, expected evidence ID, and selection limit reproduce the
   intake receipt byte-for-byte; and
2. the supplied directive exactly requests CQR Nightly execution.

Hand-forged reasons, vague directives, and output aliasing were rejected.
