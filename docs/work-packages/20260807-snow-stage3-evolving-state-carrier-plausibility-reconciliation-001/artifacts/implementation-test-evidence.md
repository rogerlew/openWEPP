# Implementation And Test Evidence

Status: `Phase C PASS / result execution authorized`.

Evidence mode: `Static + Ran`.

The package-local analyzer consumes immutable schema-v6 traces and imports only
the predecessor's independent validation/reconstruction helpers, never runtime
producer calculation helpers. Existing schema-v6 is sufficient; no Rust,
trace-schema, production, or public-output change is justified.

Implemented gates include exact retained hashes, climate/observation custody,
cross-lane/fixed-field identity, operator-specific non-formulation anti-alias,
S/F typed N/A, every-Q-tuple validation before window selection, three-way
common/all support, S/F/Q term and raw-vapor attribution, bounded transfer,
latent views, cold/melt chronology, endpoints, active state/volume, humidity,
exchange velocities, stability, per-WY reduction, canonical-only decisions,
and immutable clean-output preconditions.

The protocol was prospectively tightened before results with exact observation
hashes and an explicit rule: any nonfinite numeric evidence is invalid and
hard-holds the package because reconstruction is undefined; finite
reconstructable domain failures retain completed attribution and may emit the
nonexclusive active-state class.

Ran from `/home/workdir/openWEPP`:

- package analyzer pytest: `29 passed`;
- Python bytecode compilation: `PASS`;
- protocol JSON validation and `git diff --check`: `PASS`.

Review history: initial Phase-C reviews found missing F/Q common support,
custody, canonical reduction, per-WY support, cross-lane identity, attribution,
classification, and invalid-evidence precedence. Every finding was accepted,
fixed, and re-reviewed. Final science, Rust, and consumer dispositions are
`GO` for immutable four-site execution only.

## Four-Site Execution

Ran once successfully through the required comparator runner at exact clean
`e07cdbdf976b9cfeeb3d8ac825411ee41ad1b737`. The exact attempt-004 command
completed in `1,969.55 s` per the retained timing log, maximum RSS
`76,812 KiB`, exit `0`. Result:

- execution receipt: `923` bytes, SHA-256
  `ba922327a66184112bbcebd45dc0ec4d6f2ccd1d885e0eb085b7279de1b5cc59`;
- result JSON: `958,807` bytes, SHA-256
  `7bd19a24b63375dba9f61e8d522afcc43b42b9f9a8dd8d6156cbe9fad1fbbbff`;
- wrapper stdout/stderr/timing hashes:
  `aa65fa878fc5b53a289d10249aee4d75a36374481fc3f44b79ab462adf74600f`,
  `e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855`,
  and `636986e5498bb479b9e3b82171a62e12f03f06535fc75d32fe4c0912efe493ec`.

Attempts 001-003 are retained invalid-execution/consumer evidence and produced
no scientific result. They are not counted as scientific retries.
