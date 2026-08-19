# Gate Results

## Explicit owner-boundary increment

The GSI/forcing boundary increment passes the focused 7/7 forcing contract,
both complete 48-interval repository-derived V10 provider days, explicit
static cursor restoration, and exact downstream rollback. See
`artifacts/gsi-owner-boundary.md`. Persisted restart and package terminal review
remain pending and no package-complete claim is made.

Latest exact-tree rerun after sealing the raw provider entry point and adding
constructor-time owner validation:

- forcing contract: PASS, 7/7, Nextest run
  `db7e472b-79c2-4243-8745-9555d14b42f8`;
- repository projection, zero-radiation day, positive-radiation day, and
  downstream rollback: PASS, 4/4, Nextest run
  `c12a6798-b717-42e9-99ab-788c0a8ed2b0`;
- affected five-crate all-target Clippy with warnings denied: PASS;
- canonical formatting and `git diff --check`: PASS.

Disposition: `PHYSICS PASS / OWNERSHIP-RESTART HOLD`. Earlier green evidence
below is retained as increment evidence, not terminal proof.

Ran on the exact implementation bytes:

- adapter authority contract: `3/3 PASS`;
- forcing authority contract: `4/4 PASS`;
- strict climate parser contract: `31/31 PASS`;
- sealed parser/provider/Child-4 forcing-type projection: `1/1 PASS` at the
  earlier boundary;
- affected four-crate quick profile: `778/778 PASS`, run
  `c17992c9-bb0d-4fdb-9442-a900fb33185e`;
- strict all-target Clippy for meteorology, input-contract, and hillslope
  orchestrator: PASS;
- `cargo fmt --all`: PASS;
- `git diff --check`: PASS.

No full-workspace gate is claimed here; campaign-strength full correctness is
retained for the later dependency/campaign terminal sequence.

Fresh exact consumer evidence:

- ordinary strict climate parsing again rejects `timem > 24`; explicit
  `SnowFreeHalfHourProvider` mode alone admits support through 48 hours: PASS;
- adapter authority contract after prepare/commit cursor conversion: `3/3 PASS`;
- workspace `cargo check`: PASS;
- actual sealed zero-radiation provider day entering the public
  `DirectV10RealConsumerShadow::execute_day`: PASS, 48/48 with V10/LSE-V2
  successor state committed;
- actual sealed realistic positive-radiation provider day entering the same
  public transaction: PASS, 48/48 including respiration-dominated interval 8.

Live GSI custody and provider-cursor persisted atomic restart remain
load-bearing HOLDs. No terminal PASS is claimed.
