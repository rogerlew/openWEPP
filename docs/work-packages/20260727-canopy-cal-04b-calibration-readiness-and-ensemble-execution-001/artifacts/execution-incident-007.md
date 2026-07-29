# Execution Incident 007 — Pre-Open Custody Corrections

Evidence class: `Ran + Static`

Status: `CLOSED BEFORE HOLDOUT OPEN`

Harvard remained sealed throughout every failure described here. No failed
verifier invocation created a PASS receipt, and no holdout command ran until a
fresh freeze received two independent PASS receipts and passed the formal
barrier.

## Fail-Closed Sequence

1. Freeze construction rejected a duplicate `direct_execution_plan` identity
   in `freeze-custody-controls.csv`. The duplicate insertion was removed from
   `tools/freeze.py`.
2. Verifier A rejected a publication-overlay lookup for
   `calibration-forcing-authority-resolution.md`. `tools/validate_preopen.py`
   now resolves the authenticated manifest path below the repository root and
   rejects traversal.
3. Verifier A found 0–3 representable-step differences between Rust and Python
   when summing 36 already bit-exact annual MSE operands. The frozen
   `tools/validate.py` permits four local ULP widths only for that
   cross-language aggregate comparison. Components, annual MSEs, acceptance
   threshold, membership, and hashes remain exact. Terminal review later noted
   that local-width scaling is broader than four representable steps at a
   binade boundary; the separately added post-freeze exact-rank audit proves
   the actual 1,598 finite results have step histogram
   `{0: 986, 1: 576, 2: 35, 3: 1}`.
4. Verifier A rejected the retained-trace receipt because its
   `exact_command` omitted `--execution-root`, then rejected the corrected
   receipt because executable and script paths were relative. `tools/retain.py`
   now emits the exact absolute command rendered by the canonical plan.

## Preservation And Rebuild

Rejected freeze states are retained at:

- `/home/workdir/cal04b-freeze-archive-incident007`
- `/home/workdir/cal04b-freeze-archive-incident008`
- `/home/workdir/cal04b-freeze-archive-incident009`
- `/home/workdir/cal04b-freeze-archive-incident010`

The retained trace was deterministically reissued from the unchanged raw trace
and dual reconstruction. All three compressed objects have SHA-256
`ff78edbe12a8b6e2434687c5304a7cd70281af4b0cada87f672da39df8aa2ab0`.

The accepted freeze has digest
`6066be76a584386b33ea2a9c5ce774588f0b5b8190261f9c0523daebd0e7349d`
and 177 transitive members. Verifiers A and B independently passed the exact
pre-open semantic command before the barrier and one-time opening.
