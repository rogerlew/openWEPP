# Kickoff: Typed Seed-Authority Carrier Re-Architecture

Execute the package in
`docs/work-packages/20260630-typed-seed-authority-carrier-rearchitecture-001/`.

Build one typed per-lane seed-authority carrier for the entire computed
day-zero pipeline. Shadow-prove value identity against the existing
`HillslopeWritebackSurface` seed surface before cutover. Do not perform another
per-read burn-down and do not hide symbol-map authority behind wrapper
accessors.

If the typed carrier cannot be built from parsed input-contract data plus
day-one climate in this package, close HOLD with the exact missing typed
projection and first implementation step.
