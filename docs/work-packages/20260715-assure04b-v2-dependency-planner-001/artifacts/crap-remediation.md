# ASSURE-04B CRAP Remediation

Status: PASS; focused remediation and two independent reviews complete

Evidence classes: Static and Ran

## Finding And Disposition

The second independent heavy attempt passed formatting, workspace Clippy, full
Nextest at 2,001/2,001 with three skipped, and dependency policy. Fresh CRAP
then failed with raw/adjudicated/actionable counts of 3/2/1. The sole actionable
row was `crates/openwepp-assurance/src/cli.rs::execute`: cyclomatic complexity
27, coverage 75.5102%, and CRAP 37.7074. The finding is **accepted**; no
adjudication or exception is proposed.

## Remediation

The CLI dispatcher now contains only the command match and delegates to four
command-specific helpers. A structural `cargo crap` scan without credit for
coverage reports dispatcher complexity 6 instead of 27. The most complex new
helper is the plan path at 11; all four helpers retain the same typed guards and
consumer calls.

The planner integration suite now executes successful zero-public `build
--all --output-root <scratch>` and `check --all` CLI paths in addition to the
existing report-specific and format negative paths. The scratch build cannot
write the tracked public tree.

## Current Focused Evidence

- formatting: PASS;
- assurance all-target Clippy with warnings denied: PASS;
- assurance crate tests: 6/6 PASS;
- three assurance integration suites: 35/35 PASS;
- focused instrumented execution of the same three suites: 35/35 PASS; and
- `git diff --check`: PASS.

The prior failed CRAP bundle remains truthful HOLD evidence. Fresh full-source
CRAP must be recollected after independent review; the complexity scan is
diagnostic only and makes no closure claim.

Reviewer A independently returned PASS with no finding. Its instrumented
diagnostic measured `execute` at complexity 6, 80% coverage, and CRAP 6.288;
`execute_plan` at complexity 11, 85% coverage, and CRAP 11.408; and
`parse_options` as the highest remaining `cli.rs` row at CRAP 18.807. The
reviewer also proved the scratch build left repository status unchanged.

Reviewer B independently returned PASS with no finding and reproduced the same
coverage-backed CRAP values. It measured the other new helpers at CRAP 3–7,
probed guard routing for validate, plan, build, check, and unknown commands,
and confirmed that the successful build wrote only the two expected files below
a temporary root. Protected hashes, the `usersum/**` aggregate, line counts,
scope, and diff checks remained exact.
