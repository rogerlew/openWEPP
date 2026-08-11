# Disposition

Status: `PASS — terminally closed`

Defects `PEAK-HOURLY-001`, `PEAK-RETURN-002`, and `PEAK-UNITS-003` are closed.
The production path consumes the closing 24-bin post-partition hourly runoff
ledger, preserves modeled-hour surface return, stores the maximum hourly mean
as `m/s`, and applies hillslope area exactly once when publishing `m3/s`.
Positive runoff without hourly custody fails closed.

Closure binds implementation/contract/test commit
`33831787b7029b28b0716c8458f08a11899db446`, release binary SHA-256
`ac8790faf32a5b98993427b636084c04ba468955458c4fc18f3874cea709c4c3`,
and frozen plan SHA-256
`32e6f5e99a77747fcdd93388302f2a5ffb496a87b764ac4505e09691955db756`.
The complete Topanga cohort ran 280 baselines and all 1,088 mutations with no
unexplained volume-stable peak discontinuity. The exact-head Critical workspace
gate passed 2,346/2,346 tests. All four implementation reviewers and both
independent terminal verifiers returned PASS with no open findings. Verifier A's
sole lifecycle finding, a stale completed worker handoff, was corrected and
closed in candidate `d5320fafb`.

The executed kickoff prompt is archived with SHA-256
`c971e828559b515f433efd2d951972df9d0702219cf95369f476f8a31034c63b`.

The admitted claim remains maximum hourly mean hillslope runoff flow. This
package does not claim instantaneous/subhourly peak flow, legacy numerical
parity, calibration, empirical accuracy, or routed watershed/channel flow.

This disposition was reopened after ADR-0036 D4 and Alternative 4 were found
to retain contradictory independent-analytical-peak authority. That authority
defect is closed at `669269ee4fff3aab89ba2d5c72e4fdd34b12b7c2`: ADR-0036 now
agrees with the canonical SC-* contracts and production maximum-hour rule, and
the integration guard requires the corrected authority while rejecting the
retired wording. Both independent science reviewers, the Rust correctness
reviewer, and Rust QA returned PASS for the narrow reopened increment. The
interrupted `nextest-full-669269ee4.log` is non-admitted. The exact-source full
workspace retry passed 2,346/2,346 at `a8a96498`; its inventory contains every
quick-selected test. Fresh terminal verification remains required before PASS.

Ran on 2026-08-10 from the terminal lifecycle worktree: the focused peak
authority contract passed 4/4, formatting passed, ADR Markdown lint passed,
and diff hygiene passed. The reopened increment changes decision prose,
source-reading guard assertions, and lifecycle evidence only. It does not
change runtime arithmetic, contracts, serialized output, the release binary,
or frozen Topanga inputs. The admitted 2,346/2,346 exact-runtime workspace
receipt and complete 1,088-trial cohort therefore remain bound to
`33831787b7029b28b0716c8458f08a11899db446` and are reused rather than
misrepresented as newly run evidence.

Ran exact-source closure evidence at `a8a96498`: full workspace PASS
2,346/2,346, run ID `64cd5e97-d253-4da1-a3cf-3c4e16f83d22`, in 8,193.187
seconds; workspace doctests PASS; `cargo deny check` PASS; authority
anti-evasion PASS; peak authority PASS 4/4; and required-suite obligation guard
PASS 3/3. The quick profile's 2,297 identities remain an exact subset of the
2,346 admitted full identities, so the full receipt discharges the known
quick-timeout path without skipping any quick-selected test.

Both fresh independent terminal verifiers returned PASS for the reopened
candidate. No authority, implementation, evidence, finding-disposition,
write-set, or lifecycle blocker remains. The predecessor is terminally closed.
