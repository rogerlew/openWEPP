# Workflow Cost Containment

Ran: 2026-07-18 PDT / 2026-07-19 UTC.

- Authenticated GitHub CLI account: `rogerlew`; repository and workflow scopes
  present.
- Hosted run `29666277883`, triggered by commit
  `dba80a184dbf9246eb37f0f3974aafd8c622221f`, was canceled. Provider terminal
  state: `completed / cancelled` at `2026-07-19T00:04:36Z`.
- `testgate-shadow` provider workflow state changed from `active` to
  `disabled_manually` before host provisioning.
- `release-gates` remained `disabled_manually`; it was not enabled or run.
- Repository runner inventory remained empty after containment.

The previously recorded failed runs were `29647587353`, `29658898454`, and
`29663821156`. Each reached `GATE-CARGO-METADATA` with a cold-cache dependency
missing while offline mode was required.

Cutover acceptance run `29671679629` failed in 26 seconds before gate execution
and therefore incurred no repeated heavy suite. Its always-reporting verifier
and aggregate failed closed. The runner image was rebuilt for the two launch
defects recorded in `host-capacity-security.md`; it was not blindly rerun.
