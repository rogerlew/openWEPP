# Prospective Evidence-Role Protocol

Evidence mode: `Static`.

All ten current EB-04S observation lanes and every water year summarized in
`evidence-role-manifest.csv` are `DIAGNOSTIC_ONLY` for EB-04V–04X. They were
already consumed by EB-04S/04T and are now used to select cohorts and operators;
they cannot regain independent-validation status by splitting years after this
design was informed by their aggregate outcomes.

No calibration dataset is assigned because EB-04U selects no tunable candidate.
If a successor introduces an empirically estimable parameter, it must freeze a
separate `CALIBRATION` dataset before fitting and may not use it for independent
validation.

Promotion-grade `INDEPENDENT_VALIDATION` requires one of:

1. a new observation source or site not used in EB-04S/04T;
2. newly acquired later water years whose values were unavailable during
   EB-04U design; or
3. a genuinely sealed record partition whose outcomes were never inspected or
   summarized during design or calibration.

The source identity, record-unit assignment, exclusions, observation operator,
and release condition must be frozen before candidate execution. Existing data
remain valuable for mechanism diagnosis and calibration-readiness work; their
loss of independence limits promotion claims, not authoritative implementation.
