# Independent Clippy delta review B

Ran: read-only parsing of the three completed structured compiler logs and
verification of their metadata SHA256 bindings. Static: source/diff mapping and
prospective correction review. B ran no compiler, build, or Rust test and made
no source edits. Applicable root/work-package instructions were discovered.

## Findings first

- **Medium — new F ergonomics diagnostics:** ten emitted diagnostics at nine
  distinct source locations, listed below. These require the narrow reviewed
  correction and actual current-cut rerun; they are not inherited by subtraction.
- **Medium — new R diagnostics:** twenty-one emitted diagnostics at fourteen
  distinct primary locations, all in the new graph/replay/test/wiring code.
  They require correction after the parent unfreezes R and actual rerun.
- **Medium — target coverage:** A/F fail in orchestrator; R fails earlier in
  LSE. None proves runner lint coverage. R also does not prove orchestrator
  coverage. A smaller error total does not close these missing targets.
- **Low — new F test-adapter large-error diagnostic:** the compatibility
  adapter at `v11_covered/carrier_phase.rs:1275` adds a real occurrence of
  `result_large_err`. Retaining the canonical typed Result is appropriate;
  B approved an explicit function-only expectation on this cfg(test) adapter,
  with its architectural reason. This is adjudicated new debt, not inheritance.

Paths below are relative to the indicated crate's `src/`; lines refer to the
recorded first JSON cuts, before corrective edits. The raw messages retain all
primary/secondary spans, expansions, source text and diagnostic children.

## Command, raw custody, and actual coverage

All three execution metadata records select the same command and environment:

```sh
nix develop --offline -c cargo clippy --offline \
  -p openwepp-land-surface-energy -p openwepp-hillslope-orchestrator \
  -p openwepp-runner --all-targets --message-format=json -- -D warnings
```

`RUST_MIN_STACK=67108864`; `CARGO_NET_OFFLINE=true`. Every command exits 101 and
contains `build-finished: success=false`. Duration is execution custody only,
not a controlled mechanism performance comparison.

| Raw log | SHA256 | Bytes | Seconds | Actual failed target totals |
| --- | --- | ---: | ---: | --- |
| `raw/A-clippy-json-01.log` | `330eb5b67734e00d725eaade659af5753529b8e335e8c9f4a7e153c95aa0db8c` | 12067281 | 40.787432740 | orchestrator lib 1136; lib-test 1374 |
| `raw/F-clippy-json-01.log` | `767911dafff06fbf93a813f23ad00986719af68a27d0f504fcbd213f63531f61` | 12302864 | 50.779825037 | orchestrator lib 1132; lib-test 1384 |
| `raw/R-clippy-json-01.log` | `2a307d9a332dbf71fe26573f2be4c94ae73d76d2979d6b0f4f53bbb851cd22ee` | 182086 | 10.860924710 | LSE lib 8; lib-test 13 |

A emits successful LSE lib and lib-test artifacts with `fresh=true`; F emits
both with `fresh=false`. Neither has LSE diagnostics. R emits neither successful
LSE artifact. No run emits a runner artifact or runner diagnostic. This is
dependency-blocked coverage, not a runner PASS. The plain-text abort messages
separate lib/lib-test totals; Cargo `compiler-message.target` identifies both
as the same lib target and has no profile.test discriminator. The comparison
therefore preserves occurrence multiplicity rather than inventing per-profile
attribution for individual messages.

## Exact F mapping

Parsed valid `compiler-message` records only: A 2510, F 2516. Matched package,
target kind/name, code, message, source-backed span location and text. Line
relocation was allowed only through unchanged source-line mapping, followed
by inspection of the changed enclosing function. Included macro expansion
call-site spans; matching only `/rustc/.../macros/mod.rs` would wrongly conflate
distinct assert sites. Preserved primary and secondary spans and inspected
child-note span differences. Did not erase diagnostic text or classify by
filename age, count subtraction, or merely encountering the same lint elsewhere.

The complete F partition is:

- 2489 occurrences match primary, secondary, expansion and child-span notes
  exactly after source mapping.
- Seven additional `drop_non_drop` occurrences have unchanged drop sites and
  child-note text embedding relocated closure coordinates: five in support.rs
  tests and two at the actual terminal-provider closure. Source diff maps the
  actual same closure/drop operation; these are inherited, not omitted notes.
- Nine additional occurrences map explicitly below.
- Ten genuinely new ergonomic occurrences and one new adapter occurrence.

| Additional inherited mapping | Multiplicity | Source basis |
| --- | ---: | --- |
| terminal execution A:689 -> F:711 `result_large_err` | 2 | same boxed carrier closure and typed error API; typed carrier callee renamed |
| carrier phase A:1939 -> F:2008 `too_many_lines` | 2 | same finalizer body, 243/100 in both; request type changes only |
| evaluation A:519 -> terminal carrier evaluation F:45 `clone_on_copy` | 2 | exact original boundary receipt clone moved with canonical flux arithmetic |
| stage3_solver A:195 -> F:198 `too_many_arguments` | 2 | same eight-argument internal function; provider type changes only |
| runoff reconciliation A:626 -> F:638 `too_many_lines` | 1 | same CaptureState validation; 377 -> 370 lines is formatting-only decrease |

Five A-only occurrences disappear because old feedback hooks are now cfg(test):
two `inline_always`, two `ignored_unit_patterns`, and one
`TerminalCouplingIterationHook` type-complexity occurrence. Their corresponding
test occurrences remain. Thus 2505 inherited F occurrences + 11 new = 2516;
A's 2510 = 2505 retained + five production-only removals. This reconciliation
follows source correspondence and does not itself prove correctness or PASS.

### F new source locations

Crate: `openwepp-hillslope-orchestrator`.

| File / line | Code | Occurrences | Prospective bounded disposition |
| --- | --- | ---: | --- |
| `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_carrier_evaluation.rs:86` | needless_pass_by_value | 1 | borrow physical request; retain exactly one owned callback clone |
| same file :136 | needless_pass_by_value | 1 | borrow in test-only feedback helper; preserve reference iterations |
| same file :119 | used_underscore_binding | 1 | meaningful evidence binding with explicit cfg-specific unused handling |
| `hydrology/support_helpers_mod/runoff_reconciliation/stage3_solver/terminal_feed_forward_tests.rs:82` | needless_pass_by_value | 1 | borrow while retaining exhaustive no-`..` structural assertion |
| same file :411, :418, :422 | semicolon_if_nothing_returned | 3 | semicolons only |
| `snow_stage3_v11_terminal_feed_forward_tests.rs:3` | type_complexity | 2 | named record tuple alias; preserve complete recorded fields |
| same file :168 | semicolon_if_nothing_returned | 1 | semicolon only |
| `v11_covered/carrier_phase.rs:1275` | result_large_err | 1 | function-only expected lint on cfg(test) canonical-error adapter |

B gave prospective GO on these narrow corrections. The newly unused legacy
`coupling_iteration` field is already explicitly expected only outside tests;
this run has no unfulfilled expectation or new field warning. The inherited
`ending_snow_hint` warning remains. No audit schema or derived Debug change is
authorized to hide it. The relocated boundary clone is left as inherited debt.

## R new source locations

Crate: `openwepp-land-surface-energy`. A/F both successfully check this crate;
all following occurrences refer to R additions with no matched A occurrence.

| File / line | Code | Occurrences |
| --- | --- | ---: |
| `solver_component_dependency_graph.rs:63` | single_match_else | 2 |
| same file :105 | unnecessary_map_or | 2 |
| `solver_component_dependency_replay.rs:350` | missing_panics_doc | 2 |
| same file :352, :353 | items_after_statements | 4 |
| `solver.rs:791` | wildcard_imports | 1 |
| `solver_component_dependency_graph_tests.rs:160` (also primary :162) | match_same_arms | 1 |
| same file :185 | filter_map_bool_then | 1 |
| same file :238 | format_collect | 1 |
| `solver_component_dependency_replay_tests.rs:1035, :1048` | cast_possible_truncation | 2 |
| same file :1218 | semicolon_if_nothing_returned | 1 |
| same file :1439, :1444 | items_after_statements | 4 |

B prospective scope GO: preserve graph missing-node/unknown-coordinate
fail-closed behavior while using equivalent if-let/is_none_or; move Restore
type/impl declarations, not snapshot or guard-construction order; document the
actual runtime oracle's panic obligations; use explicit cfg-appropriate
imports; keep the graph predicate and exact serialization independent while
grouping identical arms and using filter/map/write; use checked bounded test
integer conversions and a semicolon. No authority predicate, numerical
operation, error/fallback, assertion population or source provenance changes.
R source remains frozen during the parent's full gate; this is not execution
approval before that barrier is lifted.

## Concrete F corrective source confirmation

Static GO: independently compared all five complete corrected files to the
preserved `F-source-04/changed-source-snapshot.tar.gz` entries. The only deltas
are the approved reference parameters/callers, cfg-specific evidence binding,
test-record alias, semicolons and function-local test-adapter lint expectation.
The actual callback still receives one owned request clone, the test pattern
remains exhaustive without `..`, and canonical arithmetic/error/audit shape is
unchanged. SHA256 bindings:

- `terminal_carrier_evaluation.rs`: `2f61e9524a493bff7a5739c3a267d90df212a954465a61ab11b4245086c08b7f`
- `evaluation.rs`: `c4b611892c194471c888726b39e18ca878363ab57c5792be678c0c1e8e81b40a`
- lower `terminal_feed_forward_tests.rs`: `85f540dec57b39ce3bafecef9c9c755d416693c2aad94bef9fef40c1a16f2d74`
- `snow_stage3_v11_terminal_feed_forward_tests.rs`: `4437dd3f2918d021f9bb6992f676e4f497d57c35ec385419b1d31b57da29f924`
- `v11_covered/carrier_phase.rs`: `fb1aeb1ddad426f1fcfa556a8a5dd851001df6e39264c3f3546fe25ff54f14df`

Current-cut execution remains pending; this source confirmation does not turn
the first failed lint gate green.

## Coverage follow-up and QA disposition

Prospective GO for a matched supplementary A/F/R runner-only command with the
same environment, `--all-targets --message-format=json -- -D warnings`. Ordinary
dependency checking can expose the actual runner lint target without selecting
its failed dependencies for Clippy. Require actual runner target evidence or
report any remaining dependency blocker; do not assume the command will pass.
No `--cap-lints`, inherited lint suppression, or replacement of the broad FAIL.

The strict broad gate is **FAIL for every arm**. New-code correction review is
currently **HOLD pending concrete corrective cuts and current-cut runs**; R
orchestrator and all runner lint coverage remain separately unresolved. B's
diagnostic classification is complete for these three logs. No full quality
PASS, scientific conformance, measurement admission, or terminal package
completion follows from this bounded review.

## Concrete R corrective source confirmation

Static GO: compared all five complete files to preserved R-source-04. Only
the prospectively reviewed equivalent graph expressions, explicit feature
imports, helper panic documentation, type/impl declaration moves, exact graph
test formatting/iteration and bounded test count conversions changed. Unknown
graph targets remain reachable/fail-closed. Restore snapshots and runtime
guard-construction order are unchanged. Exact corrected hashes:

- `solver.rs`: `7d3837a5dfdea1f35b37384637561c59947ca9f320865448c6463f11e264b460`
- graph: `4fca5d95b4fd764aa41ef1accbf1ea3a2723443fa68cea0c2d115c1d645cb135`
- graph tests: `36af591efe4f77618e0d30598c11f392982b880b024b22b104367751d0ba255c`
- replay: `ba07446fe108bedd96a5ded2fcac6fb493c6fe739f87b6827e699afd3ff45941`
- replay tests: `d606fb02f874fb53538c295282ba1c3ca45af129bd42a72d3ab70c47ea8393a2`

The solver hash includes removal of the unused feature-only Digest import
identified by the parent's second actual run. No test glob or shared sha2
trait use was removed. The common observer's narrow lint correction is
separately reviewed in diagnostic-parity-review-b; none is hidden baseline
debt or evidence that the preceding failed runs passed.

### Runner-only coverage correction

The earlier prospective hypothesis that package selection alone avoids
dependency linting was disproved by actual A runner-only execution: parent
reports FAIL 101 with 1136 orchestrator lib errors, raw SHA256
`c6717e20817f1320495d1762bbc9b090f255edc3a877d6dcbc270cd25df9066d`.
It still does not prove runner coverage. The local Clippy help explicitly
provides `--no-deps` for linting only selected crates. B approves matched
A/F/R runner-only `--no-deps --all-targets --message-format=json -- -D warnings`
as supplementary coverage. Dependency typechecking/build and the broad
three-package FAIL records remain; this is not `--cap-lints` or a replacement
for required orchestrator/LSE lint review. Require actual runner target
diagnostics/artifacts before stating its result.

## Corrected broad-run diagnostic mapping

Ran: F broad02 emits 2505 orchestrator errors plus two new common observer
LSE errors (doc markup and constant assertion). Mapped every orchestrator
record to F broad01 using the preserved F-source-04 snapshot and corrected
source: exactly 2505 retained, zero new, and exactly the eleven declared new
F occurrences removed. This is occurrence/source mapping, not net-count
subtraction. The shared observer errors also occur in A broad02; their
separate narrow correction is reviewed above and is not an F mechanism lint.
F broad02 remains FAIL, raw SHA256
`2c019b1390611a08bfed5a59765f7620e8a87a037b2442104b1d5de1e473885f`.

R broad03 emits 2510 orchestrator errors and no LSE/common-observer error.
Its entire compiler-message JSON multiset equals A broad01 exactly after
only replacing the R worktree prefix: code, message, rendered text, target,
primary/secondary/child spans and multiplicity all match. No extra/missing
record remains. Raw SHA256
`c0ded82c8bc95585b85f8a8cf384ad56516958e6b5911e09b3ed2cf81e01f6e3`;
exit 101, 41.715464737 seconds. This closes the new-R diagnostic delta on
reached targets, not the broad gate or still-separate runner coverage.

### Prospective common runner correction

Explicit `--no-deps` does reach the actual A runner target. Its runner-only02
log SHA256 `0fec212cbd403703d2710a1b5ba56979fa44378e36527bfaf61c5501c2596a85`
has 21 runner errors plus one ordinary dependency dead-code warning; do not
count that warning as a runner error. The five common-helper occurrences are
entry/run constant assertions, a 353-line run function, a redundant fallible
u16 conversion and an inherited qualification telemetry guard drop.

B prospective GO on only function-local constant-assert expectations,
run-only `too_many_lines` expectation documenting the bounded protocol's
authentic execution/measurement/explicit teardown lifetime scope, and exact
`usize::from(hbp.nofe)`. The 353-line function is acknowledged architecture
debt, not relabeled inherited. Static pre-A e89 inspection confirms the run
assertion and qualification guard drop were copied from the original release
profile test. The guard is PhantomData-only without Drop; leave that old
occurrence unchanged. No extraction, physics, timing boundaries, object
lifetimes, teardown ordering, output normalization or crate-wide suppression
is authorized. Concrete source and matched current runner results remain
pending; the failed broad and earlier runner runs stay FAIL.

Concrete common runner source GO: compared A/F/R complete files to their
preserved A-source03/F-source04/R-source04 snapshots. Exactly the three
prospective function-local lint expectations and one u16-to-usize conversion
changed. No clock, explicit drop, lifecycle, closure/output/control code or
R-only wrapper changed. Main/F SHA256
`d37a7f681223117eaa3fb34f6b2a66ce80448cb2f5b021c01a64cca86d397aeb`;
R SHA256 `53f071425fc4618e8bf8bd46a9abd29381a8edcfff258a1f1b39516c26e72828`.
The exact corrected runner target checks and refreshed source/binary identity
remain parent-owned execution obligations.
