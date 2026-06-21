# Scope Selection

Status: executed-hold.
Evidence mode: Static + Ran.

## Selected Scope

R6 is selected as a single direct-publication cutover package with a hard
ordering constraint:

1. Promote the PERFDEEP06 publication operand ledger into canonical authority.
2. Cut over HBP to typed direct projection only.
3. Cut over WAT to typed direct projection only.
4. Cut over PASS to typed direct projection only.
5. Cut over loss JSON to typed direct projection only.
6. Cut over run manifest publication/provenance to typed direct projection only.
7. Close package-wide no-compatibility, identity, metadata, reconstruction,
   timing, review, and verification gates.

## Not Selected

- R7 hot compatibility runtime deletion.
- Output schema redesign.
- Physics or conservation equation changes.
- Default activation without R5E authorization.

## Stop Conditions

- Direct publication frame absent after ledger promotion.
- Ledger promotion cannot be made canonical without broader contract authority.
- An output-family identity or metadata residual cannot be adjudicated under
  current authority.
- Independent reconstruction depends on the production projection function under
  test.

## Gate

BLOCKED. The selected R6 scope remains valid, and ledger promotion is complete,
but output-family cutover is not execution-ready until a run-bound direct
publication frame exists.
