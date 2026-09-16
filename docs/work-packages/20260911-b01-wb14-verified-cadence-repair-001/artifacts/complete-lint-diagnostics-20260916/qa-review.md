# Secondary QA review — complete supplemental lint diagnostics

**Reviewer:** `/root/diagnostics_qa`, an attributable replacement for the
unavailable prior QA reviewer; independent of the collection author,
orchestrator, and correctness reviewer. This review makes no continuity claim
with the unavailable conversation.

**Evidence class:** Static: reviewed the authorization, retained source and
command receipts, raw JSON/verbose stderr (and their gzip copies), collection
processor, target inventory/coverage and comparison records, prior QA review,
prior relocation evidence, and the current detached candidate/base source.
Ran-readlogs: inspected the four recorded capped Clippy collections and their
toolchain identity receipt. I did not run Cargo, Clippy, Rust tests, models, or
runtime programs.

**Source custody:** baseline tree
`114bd2877dd4141ec5023dbd99e1d7c1c97e68dfd75b5fdf6a570350c8f463e5`;
candidate tree `ecc48d5235d8d53af496856459e3eaa29cca2dda2511697350a67bac172f9592`;
candidate cumulative patch
`4559c7d21fbcdb9314845e6a5c19940d051d0ef861e48c7cb2a4ae7e51020c5d`.
Both sources retain supplemental-input custody
`2e9e27eeb0a355672e7a74c8fd645a2ce95182ae88bf78741bc8f98ecac9b082`.

## Findings

- **High — retained package acceptance blocker; no new QA defect:** Strict
  Clippy remains **FAIL** (the retained strict runs exit 101), and the original
  regression remains **FAIL**. The authorized capped commands exit zero only
  because `--cap-lints warn` is a diagnostic-acquisition method; it cannot pass
  strict Clippy. The unresolved regression/scientific disposition therefore
  keeps recorder preparation **INCOMPLETE / HOLD**.

- **High — prior coverage blocker resolved for the amended diagnostic criterion:**
  The previous QA review correctly found that strict commands aborted before all
  selected targets. The four matched capped receipts instead retain the wrapper,
  explicit source manifest, `--all-targets`, `--locked`, original feature sets,
  JSON output, `--no-deps -D warnings`, and only the authorized
  `--cap-lints warn` addition. Each exits 0 and has `build-finished: true`.
  The coverage reconciliation records normal/test library profiles plus four
  owning examples (six configurations) and normal/test runner library and five
  binaries plus nine runner integration tests (21 configurations), for both
  sources. Verbose compiler-driver records and primary compiler artifacts back
  the zero-diagnostic newly reached targets; message absence alone was not used.
  This resolves the target-completion defect for the amended observational
  comparison, not the retained strict failure.

- **Medium — no unreviewed lint-class or diagnostic-attribution gap found:** All
  strict lint classes remain represented after capping (owning: 86/86 strict
  classes on each side, 93 capped classes; runner: 17/17 strict classes on each
  side, 26 capped classes). Full retained raw streams hash to the receipt values;
  gzip round trips are lossless. Runner diagnostics are exactly 291/291. Owning
  diagnostics are 2,956 baseline / 2,954 candidate with 2,944 exact normalized
  matches, leaving six candidate groups (10 occurrences) and eight baseline
  groups (12 occurrences). Their source locations, multiplicities, and lint
  content reconcile with the prior relocation evidence: the six
  candidate-present groups are two `result_large_err` sites, three
  `too_many_lines` sites, and one `too_many_arguments` site. The two additional
  baseline-only groups are `needless_pass_by_value` for the now-consumed context
  and `fn_params_excessive_bools` for the bool-parameter cleanup. The processor
  flags five unmapped candidate spans rather than silently matching them, and
  the manual relocation/source evidence covers their relevant groups.

- **Medium — reviewed external warning class has no demonstrated defect:** The
  matching capped streams each retain nine `dangerous_implicit_autorefs` warnings
  (and four `unnecessary_transmutes` where applicable) from pinned dependencies.
  The correctness review's dependency-source inspection identifies no invalid
  pointer lifetime, aliasing, or behavior issue. They remain dependency
  maintenance debt and are not a candidate-introduced source-quality defect.

## Non-blocking debt and follow-ups

- The coverage JSON intentionally summarizes artifact profile completion but
  does not carry `fresh` onto its summarized rows. The raw compiler-artifact and
  verbose logs retain the primary-build evidence used here. A future reusable
  coverage tool should expose `fresh` in the summary to make this claim easier
  to audit.
- The capped evidence establishes compilation/lint coverage only. It does not
  execute examples, binaries, integration tests, runtime paths, or scientific
  workflows; those claims must continue to rely on their separately bounded
  evidence.
- The retained dependency warnings and the strict inherited lint debt require
  their own authorized maintenance work if cleanup is desired. No suppression,
  manifest, feature, or production-source change is supported by this review.

## QA pass statement

**Supplemental diagnostic collection: COMPLETE. Complete matched diagnostic
criterion: PASS for the authorized bounded observational increment.** Exact
source/command custody, selected target/configuration completion, strict lint
class representation, raw-log preservation, and all diagnostic differences have
adequate QA evidence. **Strict Clippy: FAIL. Original regression: FAIL. Full
recorder preparation: INCOMPLETE / HOLD.**
