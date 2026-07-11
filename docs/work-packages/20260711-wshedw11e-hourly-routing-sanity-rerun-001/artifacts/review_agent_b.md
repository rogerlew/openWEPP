# Review Agent B — Consumer, Anti-Alias, and Gate Legitimacy

Status: `EXECUTED-PASS-RECOMMENDATION`

Evidence mode: `Static + Ran`

Reviewed at UTC: `2026-07-11T06:42:55Z`

Role: independent WSHED-W11E downstream-consumer, anti-alias, ownership,
security, and gate-non-deferral review. This review does not set package
disposition.

## Recommendation and findings

Recommendation: `PASS` for the package design and current debug evidence.

The proposed `SANITY-PASS` classification is technically supported by the
fresh debug rerun and W11D authority, but it is **not yet a final earned
classification**. At this review snapshot, exact-release provenance and the
required heavy closure gates are still queued. W11E must remain active until
those gates are recorded and pass; any required-gate or exact-release failure
changes the terminal recommendation to `SANITY-FAIL`.

| Severity | Count | Finding IDs |
|---|---:|---|
| High | 0 | none |
| Medium | 0 | none |
| Low | 0 | none |

## Authority and evidence boundary

Static:

- W11C's `EXECUTED-HOLD-SANITY-FAIL` disposition remains immutable historical
  before-correction evidence. W11E does not use W11C comparator agreement as a
  correctness target.
- W11D is complete on accepted fingerprint
  `c7e0d2ab4b688356fe269acc279f3aa4cd0e62a03b494b3e8f890b43d7debbf6`.
  Its final gates include 1,693/1,693 full-workspace tests, the exact release
  consumer, independent Manning/storage and final-slot reconstructions, and
  dual verification with no residual finding.
- W11E correctly treats `SC-ROUTE-001` v56, `SC-SYSTEM-001` v90, and
  `SC-INFILE-CHANINP-001` v0.1.4 as authority. It makes no new physical,
  calibration, or universal validation claim and authorizes no source/test/
  fixture/contract edit.
- Public `chanwb` balance is explicitly supporting self-consistency only.
  Binding W11E's consumer observation to W11D's independent storage operands
  avoids presenting the public ledger's inverse formula as independent
  conservation proof.

## Real downstream-consumer legitimacy

Static:

- `mt3_hbp_hourly_consumer_contract.rs:1091-1114` launches an actual
  `openwepp-cli-watershed` process with explicit arguments. The default is
  Cargo's test-built CLI; `OPENWEPP_W11C_WATERSHED_CLI` selects the exact
  release binary for the delegated reproduction.
- The fixtures serialize actual HBP event/no-event payloads and a watershed
  runfile. The CLI executes dispatch/publication, and tests decode
  `ebe_pw0.parquet` and `chanwb.parquet` through the Parquet reader
  (`:121-130`, `:679-689`, `:738-749`, `:935-944`). This is a real consumer
  path, not a producer-only, shadow, counter, or skeleton assertion.
- The two protected consumer tests retain hourly timing sensitivity, positive
  one-/two-channel routed sediment, terminal identity, water closure, and
  same-grid downstream publication behavior.

Ran:

- Independent debug command
  `cargo nextest run -p openwepp-runner --test
  mt3_hbp_hourly_consumer_contract --no-capture` passed 7/7, run ID
  `881c90ff-a9e0-4375-b8b2-b53eed0e3e2e`, nextest 13.988 seconds. It emitted
  15 KW/CREAMS result rows, four timestep rows, and no `W11C_FINDING` row.

## Zero-count and terminal-publication anti-aliases

Static + Ran:

- The zero-count test (`:89-166`) runs three otherwise identical real CLI
  cases. Canonical three-record `nchnum=0`, 600-second output matches the
  positive-count 600-second control for peak, volume, storage, and balance at
  `1e-12`, while peak or storage differs by more than `1e-6` from the
  positive-count 60-second default candidate. It also independently inspects
  `ParsedBranch`, `dtchr=600`, `ntchr=144`, empty IDs, no warnings, and output
  disabled. The test passed.
- The CREAMS serial vector (`:243-267`) reads the external serialized HBP sums
  and requires terminal element 2, 7,200 m3 runoff, and 240 kg sediment. These
  values reject the old channel-1 identity, 14,400 m3 internal-throughflow
  sum, and rate-as-mass aliases. The test passed.
- W11E repeats the W11C two-channel topology by design. It does not claim to
  rerun every W11D topology; the separately accepted W11D
  channel/impoundment/channel vector remains the binding broader terminal
  selector evidence.

## Admitted/rejected MC non-vacuity

Static + Ran:

- The rejection vector (`:270-296`) exercises static/dynamic MC at both 3,600
  and 600 seconds over five scenarios. Four exact-zero controls execute, and
  all 16 active inadmissible cases must fail before publication with typed
  `WKERNEL-WS10-CHANNEL-E-003`. It passed.
- The distinct admission vector (`:299-321`, `:693-750`) executes nonzero
  static and dynamic MC on the admitted 60-second geometry through the same
  CLI and Parquet consumer. Both branches require finite positive passive
  peaks and a closed public channel balance. It passed.
- Therefore neither "reject every MC route" nor "accept every finite MC
  route" can satisfy the suite. W11D's matched full-route unit vector remains
  the independent proof that dynamic refresh produces branch divergence; W11E
  only claims current CLI admission/rejection continuity.

## Numerical sanity classification

Ran:

- Every zero control is exact zero.
- Across the 15 printed KW/CREAMS rows, runoff, peak, sediment, and storage are
  finite and nonnegative. KW storage is in `[0, 110.260168180] m3`, active
  peak/input is at most 1.0, and terminal runoff exceeds 7,200 m3 only by about
  `1.0e-11 m3`, below the asserted roundoff tolerance.
- Maximum observed absolute public channel-balance residual is about
  `1.779e-12 m3`; maximum absolute sediment residual is about
  `4.83e-13 kg`.
- Uniform KW's printed raw `input - outlet - final storage` value is negative
  because authorized initial storage is present. The initial-storage-aware
  public ledger closes, and W11D independently reconstructs both operands; the
  row is not generated-water evidence.
- The reported KW 3,600/600-second peak and terminal-storage differences are
  characterization observations, not violations of a canonical convergence
  tolerance. No sign, passive-bound, terminal-volume, storage, sediment, or
  typed-error invariant is violated. On current evidence they do not require
  `SANITY-PASS-WITH-FINDING`.

## Gate non-deferral

Static snapshot:

| Gate class | Current W11E evidence | Review disposition |
|---|---|---|
| Debug seven-test real CLI | PASS, independently rerun 7/7 | satisfied |
| Exact release build/provenance and seven-test CLI | `QUEUED` / `NOT RUN` in artifacts | mandatory before final classification |
| Format, workspace clippy, erosion profile, full profile, deny | delegated and pending | mandatory before completion |
| Scoped Markdown lint and diff hygiene | final evidence pending; reviewer `git diff --check` passed | mandatory before completion |
| Dual review, disposition, same-agent verification | in progress | mandatory before completion |

This is not a HOLD finding because the package truthfully remains `ACTIVE`,
its sanity artifact says `DEBUG-PASS — RELEASE PENDING`, and the authorized
heavy runner is concurrently producing the missing evidence. It becomes a
closure-blocking gate-non-deferral finding only if the owner marks W11E
complete or final `SANITY-PASS` while any required row remains failed,
blocked, or unjustifiably not run.

## Security, ownership, and line count

Static:

- W11E's owned paths are Markdown package/lifecycle files only. No production,
  test, fixture, schema, dependency, guard, threshold, or contract change is
  present. Current W11E additions are the package plus catalog/roadmap entries.
- The dirty `docs/dev-guide/06-history-and-performance.md` and untracked
  `docs/audits/20260710_h2637_34yr_laned_active_endpoint_audit.md` are outside
  the declared W11E write set and must remain excluded at handoff.
- No network call, secret, `unsafe`, shell interpolation, path resolver, or
  production debug hook is introduced. Existing CLI subprocess arguments are
  explicit Rust `Command` arguments.
- No `.rs` file is changed by W11E. The observed real-consumer test owner is
  1,541 lines, below the 2,000-line advisory threshold; line-count governance
  is satisfied.

Ran:

- `git diff --check`: PASS.
- W11E package artifact inventory: Markdown only.

## Conclusion

No High, Medium, or Low review finding is open. The real downstream path and
all required anti-alias families are legitimate, MC acceptance is non-vacuous,
and the fresh debug numbers support the proposed `SANITY-PASS` rather than
`SANITY-PASS-WITH-FINDING` or `SANITY-FAIL`. Final classification and package
completion must wait for the exact-release and broad delegated gates, explicit
review disposition, and same-agent verification.
