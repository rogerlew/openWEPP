# CANOPY-CAL-02 Intent Plan

Evidence class: `Admitted pre-implementation record`

Admitted base: `2b7dde32afce9151cdeff17269a1d56062e492be`

Admitted on: `2026-07-26`

## Intended outcome

Execute the five CAL-01-admitted, source-forcing management arms with the
hash-bound WEPP 2012.800 executable; normalize and independently reconstruct
the published process, hydrology, sediment, and return-period evidence; and
issue a bounded reproduction verdict plus CAL-03 handoff. This package changes
no production code, model equation, science contract, or public schema.

## Declared terminal write set

- `docs/work-packages/README.md`
- `docs/work-packages/20260726-canopy-cal-02-elliot-reproduction-001/**`
- `tools/canopy_phenology/**`
- `tests/fixtures/canopy_phenology/elliot_reproduction/**`
- `.gitattributes` only if a new path-confined LFS rule is required

CAL-01 evidence, commissioned references, production crates, contracts,
schemas, existing unrelated tests, and WEPPcloud production state are
read-only.

## Admitted implementation

- Add a standard-library Python harness confined to `tools/canopy_phenology/`.
- Derive each run directory from exact source fixture inputs in a fresh
  temporary root.
- Replace only the management selected by the experiment matrix; derive
  Hubbard `dropfc=0.92` by one visible token change from the delivered `0.95`
  file.
- Derive an arm-local run control by enabling only the existing daily
  plant/residue diagnostic and inserting its confined output path.
- Under the operator-authorized hold lift, replace each source-native 9002 soil
  only in the paired reconstruction lane with WEPPpy's `2006.2` serialization
  of the exact site mukey (`665220` or `131976`). Preserve both representations
  and label the lane bounded rather than historically exact.
- Under the second operator-authorized follow-on, run the same five management
  arms locally with the hash-bound Linux `wepp_260725` release, source-native
  9002 soils, `wepp_ui.txt`, and `wepp_observe.on`. Preserve this as a distinct
  executable/soil lane. Extract annual litter transfer only from a directly
  published observe value or an exact reconstruction of the production
  senescence operands; do not infer a gross flux from stock decline alone.
- Invoke BLARHG through argument-safe `ssh`/`scp` commands, using a unique
  package-owned remote temporary directory and the exact hash-bound executable.
- Parse retained legacy text outputs into stable CSV/JSON result objects and
  generate deterministic SVG figures from those objects.
- Add only compact parser/harness fixtures or manifests required to verify the
  analysis. Generated mutable run trees remain outside Git.

## Risk and gate selection

Classification: analysis tooling and durable scientific fixture/result
evidence; no Cargo dependency, production parser, public API, schema, test
posture, or process-physics impact.

Selected increment gates:

1. dependency/admission JSON identity and both source `SHA256SUMS` manifests;
2. harness unit tests, branch-derivation exactness, path-confinement and
   executable-identity guards;
3. five-arm isolated execution by the required comparator runner;
4. parser row/date/unit/pool-sum and rejected-alias validation;
5. independent equilibrium, annual, hydrology/sediment, and return-period
   reconstruction;
6. deterministic table/figure rebuild and retained-result checksum validation;
7. package/write-set reconciliation, documentation/path checks, credential and
   generated-debris scans, and `git diff --check`;
8. two independent scientific reviews, finding disposition, and two
   independent terminal verifications.

Broad workspace Rust, Clippy, Nextest, coverage, and CRAP gates are not
selected for the admitted analysis-only diff. Any production, Cargo, schema,
existing-test, or authority-suite change requires prospective plan amendment
and gate escalation before that edit.

No heavy workspace gate is selected, so the pre-heavy audit is not applicable.

## Claim boundary

The only admissible top-level successful characterization verdict is
`BOUNDED_NOT_EXACT`, because CAL-01 explicitly denies exact reproduction.
Missing historical Windows project files and runtime-library state remain
claim limits and may not be inferred from successful execution.
