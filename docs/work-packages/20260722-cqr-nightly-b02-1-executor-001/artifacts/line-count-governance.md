# Line-Count Governance

Status: `WARN-DISPOSITIONED`.

Ran: `executor.rs` is 2,996 lines at terminal implementation source. It remains
below the 3,000-line blocker but has only a four-line margin and is subject to
the mandatory 2,000-line warning.

Rationale: this package extracted three cohesive validation guards while
preserving their private locality and exact execution order. A larger module
split would exceed the one-function, one-quality-dimension authority.

Follow-on split intent: before any future net source growth in `executor.rs`,
the owning executor maintenance package must extract a cohesive executor
responsibility into a private submodule and reduce the host file materially.
