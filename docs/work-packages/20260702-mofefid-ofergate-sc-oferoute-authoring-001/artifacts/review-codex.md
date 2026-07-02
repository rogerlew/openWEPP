# Codex Review - MOFEFID-OFERGATE SC-OFEROUTE-001

Date: 2026-07-02
Reviewer: Codex
Branch/worktree: `worktree-mofefid-ofergate` / `.claude/worktrees/mofefid-ofergate`
Reviewed range: `45da5530..HEAD`

## Outcome

Ratified after amendment.

The scientific direction is accepted: `SC-OFEROUTE-001` is the correct
top-down home for opt-in hillslope OFE-by-OFE overland-flow routing, distinct
from watershed-channel `SC-ROUTE-001`, and it is sufficient to unblock D4
single-OFE KWE/TVD solver work. D5 remains gated by the explicit runon
ownership hold in `GAP-OFEROUTE-003`.

No production code or tests were changed.

## Evidence Classes

Static:
- Reviewed `docs/work-packages/20260702-mofefid-ofergate-sc-oferoute-authoring-001/package.md`.
- Reviewed `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md`.
- Reviewed `docs/specifications/science-contracts/index.md`.
- Reviewed governing schemas in `docs/specifications/science-contract-spec.md` and `docs/specifications/science-contracts/kernel-process-contract-profile.md`.

Ran:
- `bash tools/release/check_sc_unit_compliance.sh docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` (incorrect CLI form; usage failure, not a gate result).
- `bash tools/release/check_sc_unit_compliance.sh --path docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` (pre-amendment failed: missing `Symbol Alias Map`; post-amendment PASS).
- `python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` (PASS-DEFERRED: 4 rows, 2 science-review-follow-on rows).
- `bash tools/release/check_authority_suite_antievasion.sh` (PASS).
- `cargo nextest run --test auth11_required_suite_obligation_guards_contract` (2/2 passed).
- `cargo fmt --check` (PASS).
- `git diff --check` (PASS).
- `.venv/bin/python - <<'PY' ...` section-audit helper failed because this worktree has no `.venv`; reran with `python3 - <<'PY' ...` (missing sections `[]`, ordered `True`).

I did not rerun the full suite, clippy, or deny for this contract-text-only
package. The load-bearing gates here are the contract/unit/BEI and authority
guards above.

## Findings

| Candidate | Verdict | Evidence | Disposition |
|---|---|---|---|
| CX-001: initial contract did not satisfy the new kernel-process contract profile. Required state-surface, branch/guard, obligation, symbol-alias, unit-governance, tolerance, and test-vector sections were absent; invariant rows also lacked explicit guard/failure-posture fields. | Accepted | Static: required schema in `docs/specifications/science-contract-spec.md`; profile in `docs/specifications/science-contracts/kernel-process-contract-profile.md`. Ran: unit lint failed pre-amendment on missing `Symbol Alias Map`. | Fixed in-review. `SC-OFEROUTE-001` now has the required sections and ordered profile content at `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:103`, `:129`, `:146`, `:171`, `:181`, `:190`, `:203`, `:218`, `:229`, `:242`; unit lint now passes. |
| CX-002: D4/D5 over-authorization risk. | Rejected after amendment | Static: D4 solver authority is limited to `INV-OFEROUTE-005..007` at `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:152`; D5 cascade/runon remains held by `INV-OFEROUTE-008..009` and `GAP-OFEROUTE-003` at `:155`, `:156`, `:267`, `:268`, `:277`; package closure repeats the D5 block at `docs/work-packages/20260702-mofefid-ofergate-sc-oferoute-authoring-001/package.md:52`. | No blocker. D4 is unblocked; D5 is not. |
| CX-003: frozen-library references could overstate primary authority. | Rejected | Static: secondary-via-R-63/KINEROS posture is explicit for Shen & Li, Hirsch, Woolhiser, and TVD numerics at `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:73`, `:74`, `:78`, `:80`; gaps preserve the evidence limits at `:275` and `:276`; D-val obligations carry empirical confirmation before activation at `:246` and `:247`. | No blocker. The contract is honest about secondary evidence and makes D-val the executable confirmation gate. |
| CX-004: Binding Exposure Index is not fully consolidated. | Deferred by design | Ran: BEI lint reports `PASS-DEFERRED`, not `PASS`. Static: the two deferred rows are the prospective D4 solver and D5 cascade bindings at `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:267` and `:268`; the narrative explicitly says these bindings do not exist yet at `:257`. | Acceptable for this gate. Deferral is visible and owned; it does not satisfy D4/D5 completion by itself. |
| CX-005: default-path safety / opt-in scope. | Rejected | Static: opt-in/default-off scope appears at `docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md:54`, out-of-scope default activation at `:64`, default branch guard at `:142`, invariant at `:157`, byte-exact tolerance note at `:231`, and default-path test-vector obligation at `:249`. | No blocker. Contract does not authorize default activation. |

## Ratification Decision

`SC-OFEROUTE-001` is ratified as an active, approved D4 prerequisite. The
registry row was updated at `docs/specifications/science-contracts/index.md:55`,
and the package was closed at
`docs/work-packages/20260702-mofefid-ofergate-sc-oferoute-authoring-001/package.md:3`.

D4 may proceed under `INV-OFEROUTE-005..007`. D5 may not close until
`GAP-OFEROUTE-003` is designed, implemented, and guarded against DC01 double
counting.
