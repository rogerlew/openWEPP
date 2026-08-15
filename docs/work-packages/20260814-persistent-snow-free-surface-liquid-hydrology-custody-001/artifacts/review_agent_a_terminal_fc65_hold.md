# Rust Correctness Review at `fc65b2819`

Evidence class: `Static + Ran`

Verdict: `HOLD`

The fresh review found five material issues:

1. raw or already canonical surface-liquid callback failures retain the wrong
   phase/context instead of the known ResourceCandidate boundary;
2. nested land-surface error classes are collapsed to E003 rather than mapped
   to canonical E001/E002/E003/E004/E011 taxonomy;
3. unified full-validation failures discard the complete raw malformed
   configuration/state attempted hash;
4. the pre-callback receiver-expectation check does not bind the configured
   infiltration thermal layer; and
5. duplicated taxonomy match tables contributed to the drift.

The integration suite passed 57/57, the complete orchestrator library passed
600/600, custody authority passed 10/10 and formatting passed. Passing tests do
not override the counterexamples. No finding is rejected or deferred.
