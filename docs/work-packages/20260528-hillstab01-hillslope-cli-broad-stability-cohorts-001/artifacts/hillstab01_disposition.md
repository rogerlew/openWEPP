# hillstab01_disposition

Status: complete  
Evidence mode: Ran

## Disposition
- decision: HOLD
- date: 2026-05-28
- reason: broad stability cohort gate failed (`0/1185` pass).

## Blocking Conditions
1. Parser incompatibility against large legacy cohort input surfaces:
   `SOL-E-006` and `MAN-E-009` dominate failures.
2. Runtime domain-guard failures on a subset of cases that do parse:
   `HKERNEL-WB16-PEAK-E-003` and `HKERNEL-EROD14-WAVE2-E-003`.

## Closure Statement
HILLSTAB01 objective (execute broad stability cohorts and assess readiness) is
complete. Release readiness is **not** achieved; follow-on remediation package
is required before attempting hold lift.
