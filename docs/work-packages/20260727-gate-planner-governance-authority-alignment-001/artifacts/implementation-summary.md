# Implementation Summary

Evidence class: `Static`.

The operative governance now implements ADR-0043:

- agents select and execute applicable validation requirements directly;
- the planner and TESTGATE are frozen historical tooling with no prospective
  permission, execution, evidence, lifecycle, CI, runner, or closeout role;
- the optional advisory linter is read-only, non-authoritative, nonblocking,
  and absent from CI;
- independently binding correctness, science, security, assurance, review,
  package, campaign, and release obligations remain direct requirements;
- unknown production impact still receives conservative escalation;
- coverage/CRAP remains observational under ADR-0041 except for explicitly
  owned metric packages.

The legacy 1,303-line executor source-literal guard and its Cargo registration
were deleted. The retained alignment guard now checks ADR-0043, direct
governance, historical-object identity, removed planner-admission rows,
preserved AUTH11 rows, and frozen package status.
The snow-density runtime-confinement allowlist no longer permits the deleted
executor-guard path.

Generation 17 is preserved by immutable Git blob
`ab8fe3e4db61df6691a96a11fa2034b90036bfb2` with SHA-256
`74203b294dcea4c7f3ecb5fe4110a425d938d2ec75bde60cfc646a54fea3f5e9`.
The live generation-18 impact map binds the rewritten strategy digest and
contains no planner-policy, planner-lifecycle, or planner-authority admission
row. Its v1 `ADR-0039` identifier remains a schema-compatibility field until
roadmap Order 3.

Four incomplete planner prerequisite packages are frozen and superseded by
ADR-0043. The completed ledger-bootstrap package remains complete and is
explicitly historical.
