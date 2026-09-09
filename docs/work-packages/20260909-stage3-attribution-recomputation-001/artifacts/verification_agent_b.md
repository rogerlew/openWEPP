# Independent terminal verification B

Static: root/common/verifier governance, package, protocol/analyzer, source and
command manifests, raw gate logs, both final reviews, finding dispositions,
artifact inventory, current handoff and populated final disposition. Applicable
testing-strategy sections 7, 9–10 and 18, assurance template and capture rules
were read in this verifier session's preceding bounded assignments.
Ran: independent receipt/log and custody hashing, exact seven-file P-to-Q
reconstruction, complete analysis execution, negative accounting checks, and
independent raw trace grouping. No simulation, build, source edit or profiling run.
Role/session: `/root/terminal_verifier_b`; configured medium; effective UNOBSERVED.
Write ownership: this artifact only. Reviewed scope: terminal evidence integrity,
command/recovery sufficiency and truthful EXECUTED HOLD / NO NEW PROTOTYPE.

## Checked identity and recovery

Q: `/tmp/openwepp-attribution-Q-3r3eP8`; baseline P:
`/tmp/openwepp-compact-cost-P-vKI6gb`. Final bundle:
`/tmp/openwepp-attribution-Q-final-bundle`, manifest SHA-256
`de78b15b7fa1f69252fdd073002fe6c25fc45bc0552e354045adcc45615291e5`.
Final runner SHA-256:
`3e684ac3e8b4dcc93f0dfcffe88faa0c18289f39770237ef4bd404f782d4ce53`.
Patch SHA-256:
`a254e07c09953339bb8f7c9ecfe354c799fc8d41db436ceb6684835cef4550f4`.

Ran from `/workdir/openWEPP` with `.venv/bin/python`:

- `tools/agents/evidence_bundle.py verify /tmp/openwepp-attribution-Q-final-bundle`
  returned exit 0. Independently hashed all 17 current Q/custody-stage members
  against the verified bundle entries, including seven source files, executable,
  sidecar, identity, protocol/scripts/patch and two analyses: all match.
- Independently applied the unified patch in memory to retained P text, requiring
  every old/context line at its exact hunk position and zero offset/fuzz. All
  seven reconstructed byte strings equal current Q. No tree was modified.
- Final series protocol and carrier trace match their published hashes
  `201fc3ab...fc60e3` and `9290f1f3...57f302`. External runtime environment hash
  independently matches `463cd045...1074a84`.

This establishes declared byte composition, not standalone build or full source
restoration. P/A source kits, actual Nix/Cargo toolchain/cache and environment
remain explicitly external. No original Git-index reconstruction is required.
Raw series/trace are retained in the package outside the compact external bundle.

## Independently executed behavior and reconstruction checks

Read all 16 numbered final series-05 JSON receipts and package-local log bytes.
Every receipt is valid with child exit 0; every log SHA and analysis receipt SHA
matches. Executed package `analyze(rows)` and compared every returned field to
`raw/analysis-final-cut3.json`: exact equality. Six paired on profiles each close
with signed accounting difference zero, without normalization of medians.
Four malformed profile copies—negative elapsed, zero denominator, inconsistent
accounting difference and duplicate bucket—each raise ValueError. Tool-session
outputs retain the command results; no new evidence files were fabricated.

Independently grouped raw `recompute-trace-cut3/carrier-0.json`, without invoking
its recomputation analyzer: lifecycle counts are 400 Carrier, 400 Provider,
200 Evaluator and 72 Outer. Providers have 204 recorded request keys; 132
repeated groups cover 328 executions and 196 nominal repeats. Eight repeated
keys produce multiple outputs. All 200 evaluator groups have exactly two calls,
distinct recorded requests and equal recorded outputs. These results reproduce
the published negative conclusion. Equal recorded keys are demonstrably not a
complete effective-input authority; equal outputs cannot establish role,
mutation, validation or ownership interchangeability.

Command recovery is bounded and honest: series protocol/receipts provide exact
runner argv, CPU0, cwd, schedule, timeouts, environment input and exits. Build,
fmt and focused-test argv are explicitly reconstructed from execution records,
not falsely claimed to be printed in logs. Detailed capture is a separate
observer-off/hash-on execution, not part of the timing evidence. Development
series and failed custody/primary-worktree invocations remain distinct from
final-Q evidence. Retained exact-Q compact test log reports 3/3 PASS; I did not
rerun those Rust tests or infer invalid-physics solver coverage from them.

## Review closure, inventory and requirement legitimacy

ATTR-A01/QA-ATTR-01 is correctly resolved as a disposition correction, not a
green lint result. Both retained P/Q Clippy logs end in compiler failure; matched
source-attributed diagnostics support no observed new issue but cannot replace
the frozen warnings-denied requirement. Reviewers' matched-diagnostic comparison
is reused at its stated strength; no count-subtraction qualification is made.
The required gate remains FAIL and prevents package completion.

ATTR-A02 and QA-ATTR-03 corrections are visible in the final report: 4.20% is
only named runtime validation/reseal/acceptance, while common experiment-only
audit overhead remains unmeasured inside honest remainders. Off/on isolates the
new compact observer, not every audit. QA-ATTR-02's command/dependency receipt
now supplies the missing execution and external-custody distinctions.

Initial inventory lacked final-disposition.md; parent supplied it during this
verification. I reread it and the artifact index: the final record explicitly
says EXECUTED HOLD / NO NEW PROTOTYPE, identifies the failed Clippy gate, and
retains measurement/recovery limits. Both independent reviews and dispositions
are present; independent verifier A is a separate required record, not presumed
by this B verdict. Final catalog/handoff verification-status bookkeeping and
scoped commit remain executor responsibilities.

Non-deferral explicitly checked: a failed current QA requirement has not been
retrospectively narrowed or converted into inherited PASS. No broad lint cleanup
is authorized merely to evade HOLD. Conversely, the conditional new prototype
requires proven distinct interchangeable computation and >=5% removable cost;
neither is demonstrated. Not implementing it is the lawful gate result, not an
unexecuted authorized implementation. Established F is not renamed as new work.
The result does not prove no future opportunity exists or that any large named
region is removable. Production, scale, memory and full-workspace qualification
are not claimed. Primary status is package-only; no primary Rust/science changes
were observed. `git diff --check` passed tracked changes.

## Verdict

**PASS for terminal evidence integrity, declared recovery and the truthful
EXECUTED HOLD / NO NEW PROTOTYPE handoff.** No new B blocker remains.
**Clippy remains FAIL; package and production remain HOLD, not complete.**
This verification grants no cache/prototype, new architecture authority or push.
Substantive source/evidence/claim changes reopen their affected assurance.
