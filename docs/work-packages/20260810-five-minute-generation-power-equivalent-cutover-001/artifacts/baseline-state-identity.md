# Baseline State Identity

Status: `PASS — protected pre-edit identities frozen`

Evidence mode: `Ran + Static`

## Source and execution identity

- Source `HEAD`: `c9f28a7dbe7adf69d8e6d54ebd8da57568af5552`.
- The worktree contained only the predecessor lifecycle evidence and this
  successor's package-local diagnostic/documentation increment; no production
  Rust, contract, schema, or integration-test edit existed at freeze time.
- Release runner SHA-256:
  `ac8790faf32a5b98993427b636084c04ba468955458c4fc18f3874cea709c4c3`.
- Fresh sidecar source identity: `c9f28a7dbe7adf69d8e6d54ebd8da57568af5552`;
  sidecar SHA-256:
  `019e29af8f089ec052a9a08269320d2017061d663f7f98ed5a70626fd17e4a7b`.
- Toolchain: Rust/Cargo 1.92.0 and cargo-nextest 0.9.138.
- Execution: release/default features, compatibility sidecar policy, no legacy
  discovery, no relevant environment overrides, selected
  `direct-production-executor`, authoritative 3,600-second scheduler.

The external immutable evidence root is
`/home/workdir/openwepp-five-minute-baseline-c9f28a7d`. Compact hashes and
numeric summaries are in `baseline-output-hash-manifest.json`.

## Protected surfaces

Ran: the current peak authority passed 4/4 (nextest run
`31a508fe-64c1-4581-9de8-ab47f72a5e5a`). The focused p61 and p102 erosion
consumers passed 2/2 (run `5f838d29-2971-4177-b8b1-ed67855877aa`).

The external root freezes byte hashes and canonically ordered logical-row
hashes for WAT and PASS, HBP bytes, loss JSON, run manifests, public peak and
runoff/erosion summaries, and the p102 24-slot multi-OFE carry total. The
amplified p61 fixture is the existing contract-test mutation and has nonzero
erosion; p102 is the protected multi-OFE non-adoption control.

Ran: six focused synthetic authority cases cover dry supply, constant
nonponding input, delayed ponding, high-intensity ponding, saturation return,
and tiny positive WB14 supply. They passed under nextest runs
`a179b56d-8752-45de-b4a1-616c2644276b` and
`73662dee-2492-4d95-aeda-0c79a4336d1c`. Their authoritative source files are
content-addressed in the manifest. Two persistent rill-width tests passed under
run `383819bd-20ce-477e-8790-3be75c968e9d`, proving prior-width carry and the
rill-spacing cap before edits.

## Conditional branch disposition

The frozen Topanga baseline plus `Ksat +1%` erosion mutation was not executed.
The prospective constitutive study had already frozen
`NO_FIXED_EXPONENT_ADMITTED` / erosion `NO_ADOPTION`, and the outcome embargo
prohibits opening result-bearing Topanga mutations. Such a result cannot alter
the independent water-output decision. Input/plan identities remain frozen in
`topanga-plan-identity.json`; no erosion result-bearing identity was created.

These baselines are strict noninterference anchors. They are not erosion
magnitude targets and do not authorize an erosion candidate or cutover.
