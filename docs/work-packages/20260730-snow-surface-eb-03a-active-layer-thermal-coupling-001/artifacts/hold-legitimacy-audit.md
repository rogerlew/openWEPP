# HOLD Legitimacy Audit

Status: `complete / historical hold lifted`

Evidence mode: `Ran`

The hold is not a surrogate for incomplete science or an attempt to move a
current-scope mechanism into EB-04. Every EB-03A science, conservation,
real-consumer, frost, contract, lint, and review gate passes.

The first blocker is the prospectively required
`cargo nextest run --workspace --profile quick`. It failed after 181 passing
tests because `cqr_quality_evidence_self_test_passes` reported that its
generated valid exact-head fixture was not CURRENT. The test reproduced alone
twice, including independent verification.

The second blocker is the ADR-0043 Critical full profile. It timed out in two
assurance publication matrices at the configured `720 s` limit.
The run had 191 passes and no EB-03A failure when its already definitive
non-pass inventory was interrupted after `1,598.956 s`.

The package amends assurance inputs, so independence from the publication
timeouts is not established. Repairing or adjudicating CQR and assurance
publication machinery is nevertheless outside the authorized production
write set. Required passing evidence is unavailable until separately
authorized resolution and exact quick/full profile reruns.

No calibration round, clamp, coefficient, observation, or additional snow
physics is proposed. The hold condition is precise: the CQR and assurance
profile blockers are repaired or otherwise closed under separate authority,
then the complete quick and full profiles pass.

SNOW-SURFACE-EB-03B met that condition. It corrected both validation defects
without a snow-physics change, and complete quick, frost, and Critical full
profiles pass. This audit remains as historical justification for the earlier
hold; it is no longer an active blocker.
