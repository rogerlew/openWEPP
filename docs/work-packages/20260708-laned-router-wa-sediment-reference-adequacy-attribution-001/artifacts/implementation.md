# Implementation

Evidence mode: Static.

No production code, contract text, mesh-policy selector, tolerance, or runtime
behavior changed in this package.

Implemented package-local evidence tooling only:

- Added `artifacts/analyze_wa_sediment_reference.py`.
- The analyzer reads the prior coupled space-time package's ignored WA raw
  outputs, recomputes the failing annual `tdep:4` surface, decomposes the
  implicated daily sediment delta, compares that day against the active trace,
  records file hashes, and writes:
  - `artifacts/wa-sediment-attribution.json`
  - `artifacts/wa-sediment-attribution.md`

The package deliberately did not amend `SC-OFEROUTE-001` because the
mechanism-specific metric policy remains a contract-authority question for the
next package.
