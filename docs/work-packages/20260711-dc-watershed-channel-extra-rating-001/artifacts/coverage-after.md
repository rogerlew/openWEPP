# Coverage/CRAP after

Status: PASS
Evidence mode: Ran

Terminal targeted capture passed 38/38 at 662/665 lines (99.549%), 793/798
regions (99.373%), and 31/31 functions. Every named function exceeds 75%; the
minimum is `canonical_suffix_closes` at 96.970%.

Commands exited zero: clean, targeted LCOV, targeted JSON, and LCOV-backed
CRAP. Timings were 1.53, 21.51, 21.66, and 1.12 seconds. Evidence:

- `lcov.info`: SHA-256
  `01d9f5eae6386403df67a33d49587fc315a7567262fd1de590fdc2126bce07f5`,
  213,094 bytes.
- `coverage-after.json`: SHA-256
  `4cee3b1d6e5152668285fb7885c83086e03874fdd82ab7655822757f85252bf6`,
  1,073,538 bytes.
- `crap-after.json`: SHA-256
  `8e611c2990963c6f0917e4e83ccaa00dcd951e9714e9217f07c8a34787c9111e`,
  2,790,734 bytes.

Terminal source SHA-256 is
`a2b18016361731f8f568857de4210f5e207b03683744ff42f53e41323d206b1d`
(27,275 bytes, 956 lines); focused test SHA-256 is
`7999f66715eef99117426de1c32e5136f350a547d0f6c7c1e3dfad1fba3226ba`
(31,082 bytes, 932 lines).

Five residual regions are reviewed defensive/alternate closures: expected-
count Some/equal, strict `icntrl==0` with supplied override, and loop-internal
suffix memo cache hit. Mismatch/reject/compat branches, top-level memo hit, and
suffix true/false outcomes are directly covered. No exclusions reduce the
denominator.
