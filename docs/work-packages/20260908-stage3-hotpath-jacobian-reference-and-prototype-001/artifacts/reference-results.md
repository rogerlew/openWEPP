Static: independent baseline-only affine/FD oracle implemented in isolated J.
Ran: matched-release oracle PASS in j-release-numerical-tests2.log and
j-release-numerical-tests3.log. Earlier debug j-authentic-tests.log fails strict
base replay before reference construction; that failed attempt is retained.
Matched release uses the original corpus and unchanged comparison rules.

Independent read-only extraction of release3 evidence: 12 records, 48 complete
columns, 1,392 entries including 1,344 structural zeros. Maximum selected FD
uncertainty 4.083309083082123e-6 K^-1 versus allowance 2.5e-4 K^-1 (1.6333%).
Reciprocal uncertainty at most 2.220224026847629e-10 K^-1. The 192 canonical
probes comprise 160 centered and 32 inward, with no unresolved probes.
All three directional modes execute in each family: Covered Potential has
32 PASS and 16 domain-inapplicable directions; GenericV3 Potential and FixedFinal
each have 48 PASS. Two Covered boundary records allow only positive directions;
the two interior Covered records exercise all modes. No nonzero affine
truncation pattern is demanded. Log SHA256:
831c3c9a73cd783500c0d9e23434f08f70236783d7277bce2e182f3f0fbc2140.
The overall release3 suite has a separate test-local positive-area fixture
failure; oracle success does not override that open exclusion-coverage gate.

Frozen policy uses four lawful physical steps, actual abscissae, explicit
binary64 roundoff bounds and complete source-derived zero rows. Independent
integer reciprocal is rounded once. Candidate values do not select references
or bounds. An affine residual has zero truncation term; no mandatory nonzero
second-order convergence ratio is introduced.

Independent postadjustment helper reconstructs complete canonical nominal-step
FD matrices and compares supplied J columns after unchanged adjustments. This
passed across retained participating family/pass records. RHS evidence uses
identical explicitly declared captured-current anchors, not historical initial
anchor reconstruction. Actual full solve equality remains a separate gate.

Final measured cut3 reference JSON is byte-identical to the retained14-test
release reference result. Parent directly compared both marker payloads;
cut3 payload SHA2564117617f365aa38528f8790cf8ad06cd34a85185d6b7eae4f6a0bb9e5a510439.
The current15-test release selection therefore directly reproduces these
numerical values; this is not an inference from source-only lint changes.
