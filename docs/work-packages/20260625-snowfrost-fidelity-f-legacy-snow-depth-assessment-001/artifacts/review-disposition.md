# Review Disposition

Evidence mode: Static + Ran.

## Review 1: Comparator Lineage

Finding: The initial large-graphics plan would have produced an invalid
calendar alignment because large graphics is sparse for these hillslope
fixtures.

Disposition: Accepted and fixed. The helper now captures date-aligned legacy
physical snow depth from daily-winter hour-24 rows. Large graphics remains
enabled and documented only as sparse `snodpy`/`densg` operand provenance.

Finding: Legacy WAT `Snow-Water` could be mistaken for depth.

Disposition: Accepted and guarded. The helper reports WAT `Snow-Water` as SWE
only, and the package docs state that it is not a physical snow-depth proxy.

## Review 2: Scientific Disposition

Finding: Legacy is closer on two paired sites but worse on one paired site; a
legacy-port recommendation would overstate the evidence.

Disposition: Accepted. Closure states that legacy is source-line guide and
flag evidence only. Both models fail snow-depth control on the paired sites.

Finding: SWE deltas are materially smaller than observed depth residuals, so
the next package should prioritize depth producer/carry/input/settlement rather
than WAT SWE publication.

Disposition: Accepted. ROADMAP now routes next work to the snow-depth
producer/carry/input/settlement DC.
