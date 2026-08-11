# Direct Authority Policy Inputs

These compact JSON inputs belong only to direct science-contract admission.
The external-authority anti-evasion guard and its contract are independent
direct owners and do not route through this map. These files are not planner
inputs, execution plans, CI admission, or lifecycle authority.

The retained `ADR-0039` identifiers are historical schema identities. Live
entries are `SCHEMA_ONLY_NONBLOCKING`: a registry match is information only
and has no prospective planner effect. Retired execution, lifecycle, and
planner authority rows remain absent.

Historical generation-17 policy verification uses the immutable Git object
named by `gate-policy/history/adr0039-generation17.json`; it never derives
historical identity from these live direct-authority inputs.

Direct admission accepts multiple atomic one-contract entries for a shared
science path and validates every matched authority and covering A1 gate.
Duplicate-contract or multi-contract entries fail closed. For an uncommitted
increment, `check_science_contract_admission.sh --base-ref <commit> --worktree`
includes tracked and untracked paths and reports a stable fingerprint of the
complete authority-input surface.
