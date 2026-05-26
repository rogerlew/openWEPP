# MOFE11 Disposition

Status: complete
Evidence mode: mixed (Static + Ran)
Disposition: GO-WITH-AMENDMENTS

Disposition date: 2026-05-25

Static:
- Scoped legacy `oratea/orater=0` compatibility is implemented in runtime
  projection and decomposition transition guards with canonical contract
  authority alignment.

Ran:
- Contract-derived tests pass post-implementation.
- Carved-letter `H324` lane rerun now completes and emits candidate outputs.
- Semantic comparator was executed (direct and normalized-baseline modes).

Accepted amendments (non-blocking for MOFE11 objective):
- Semantic comparator promotability is limited by expected baseline/candidate
  shape differences rather than runtime seam defects:
  - baseline keys: multi-OFE + absolute years (`Y=2020..2025`)
  - candidate keys: single-row canonicalized aggregate + relative years
    (`Y=1..6`)
- Direct comparator invocation against legacy `H324.wat.dat` fails because the
  baseline includes a 26th `InterceptionStorage` column (post-260430 shape)
  while current comparator dat parsing supports 20/25-column rows.

Follow-on required:
- New package to close MOFE comparator readiness for carved-letter parity:
  define authoritative key-normalization/aggregation mapping (OFE/year basis)
  and baseline-schema handling, then rerun semantic comparator for `H324` with
  promotable evidence criteria.
