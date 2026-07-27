# Independent Review A

Evidence class: `Static`

Disposition: `PASS`

The initial review held for three issues:

1. ADR-0042 did not preserve ADR-0024/0028 authority-admission routes.
2. Calibration data and held-out A4 validation were conflated.
3. Combined terminal labels overlapped ordinary package dispositions.

Corrections preserve both admission routes, assign prospectively distinct
measured-data roles, reserve A4 for held-out validation, and use three
orthogonal science-implementation, calibration-evidence, and identifiability
fields. A follow-up wording defect tying data-limited calibration to absent A4
was also corrected to use insufficient `CALIBRATION`-role data.

Final review found no scientific-authority blocker.
