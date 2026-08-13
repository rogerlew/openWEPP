# Evergreen Disposition

Static: evergreen requires exact binary64 `current_growth_fraction=1.0`.
Every one of the twelve storage/transfer C/N values must be in the zero class;
`+0.0` and `-0.0` are accepted as zero. Evergreen cannot enter onset
preparation or deployment.

Non-one `f_cur` or a nonzero storage/transfer value rejects configuration or
state. Migration reports violations exhaustively and never normalizes them.

