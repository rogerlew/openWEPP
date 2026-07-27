# Resume Intent

Status: `PROSPECTIVE / REVIEW REQUIRED`

Evidence class: `Static`

Authenticated restart base:
`dd3b2a59018bd9a39999f9b263b07351afc34290`.

The resumed increment executes the existing frozen CAL-04B command DAG without
changing calibration domains, observations, operators, objectives, acceptance,
stopping behavior, or Harvard custody. Production and predecessor science are
read-only.

The existing `/home/workdir/cal04b-objects/` tree contains retained native
hold-lift evidence. It will be moved byte-preservingly to a uniquely named
pre-resume archive. A new empty `/home/workdir/cal04b-objects/` becomes the sole
attempt-005 execution root. The observed executor must restart at `prepare`.

Pre-execution acceptance:

- native hold-lift, assurance identity, and TESTGATE packages are `COMPLETE`;
- exact full workspace run `7e79049d-0871-4142-a9f7-86ac7ac714be` passed
  2,301/2,301;
- active kickoff is byte-identical to the archived accepted prompt;
- source, binary, command, repository-head, and prerequisite identities are
  rebound before result-bearing work;
- scaffold and executor validators, executor tests, warnings-denied Clippy,
  cargo-deny, documentation lint, and diff hygiene pass;
- two independent read-only reviewers approve the fresh restart;
- Harvard remains `SEALED`.

Heavy execution ownership remains with the authorized
`comparator_suite_runner`. Freeze verification and terminal review/verification
remain independent and read-only.
