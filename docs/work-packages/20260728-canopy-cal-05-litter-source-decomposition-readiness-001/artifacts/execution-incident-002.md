# Execution Incident 002

Evidence class: `Static correction from terminal review`

Terminal review found that the first sensitivity summary used endpoint slopes
and mislabeled the dimensional covariance unit. Before closure, the summary
method is frozen as a central finite difference around source `S020` using
`S010/S030`, and a central finite difference around rate `K050` using
`K000/K100`. These points and steps were all in the prospectively frozen grid;
no execution axis or result changed.

Source-rate covariance units are corrected to `kg m^-2 yr^-2`. Rate
sensitivity units are written `kg m^-2 d`. The summary must be regenerated and
independently reviewed. This amendment cannot select a parameter or upgrade a
status by itself.

## Operator governance adjudication

On 2026-07-28, after the incident and its scientific effect were presented,
the operator stated: “I authorize the retrospective analysis.” The
authorization permits these diagnostics to support the ADR-0042 readiness
classification for the narrowly named direct-runtime surface source/rate
operator. It does not make the method prospective, select a parameter, alter
the frozen grid or ridge, authorize empirical calibration, or resolve missing
source authority. The package therefore preserves this incident as
retrospective history while lifting its sole governance hold.
