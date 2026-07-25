# Runtime-Artifact Security Review

Evidence class: Static / Ran.

Reviewer: `measurement_review_b`.

Final disposition: `PASS`, no open findings.

The initial review found a HIGH lifecycle gap: the last executable-manifest
check preceded Cargo CRAP and evaluator work. The finding was accepted.

The final candidate centralizes exact executable/working-tree identity and
checks it after every profile, after coverage derivation, and after all
target-using finalization work immediately before PASS publication. Behavioral
self-test adds a new executable after sealing and requires rejection. No later
target-using subprocess exists.

Independent Python compile/self-test, focused 5-test Nextest contract, and diff
check passed. The HIGH is closed. The committed admitted-snapshot assurance
publication probe remains required consumer evidence before fresh HEAVY.
