# Conservative Rollback Contract

Evidence class: `Static`

The new `.github/workflows/testgate-shadow.yml` is nonblocking and independent
of `.github/workflows/release-gates.yml`. The latter retains its
`workspace-validation` job and direct call to
`tools/release/run_release_candidate_gates.sh`; no current command, trigger, or
artifact upload is removed.

Before provider-side cutover, repository rollback may remove or disable the
entire nonrequired shadow workflow, including its observational aggregate. No
conservative-runner restoration command is needed because that runner remains
present and authoritative throughout observation. Retaining a failing new
aggregate is required only after provider-side cutover has made that context
required; that future rollback also requires authenticated repository-rule
changes and is outside this package.

Automatic rollback triggers remain those in the canonical strategy: any missed
required obligation, unsafe reuse, certificate trust failure, ledger lost
update, or required aggregate context disappearance. The stable
`testgate-shadow-observation` aggregate is not a protected merge context in
this package.
