# Direct Authority Policy Inputs

These compact JSON inputs belong to direct science-contract admission and
external-authority anti-evasion checks. They are not planner inputs, execution
plans, CI admission, or lifecycle authority.

The retained `ADR-0039` identifiers are historical schema identities. Live
entries are `SCHEMA_ONLY_NONBLOCKING`: a registry match is information only
and has no prospective planner effect. Retired execution, lifecycle, and
planner authority rows remain absent.

Historical generation-17 policy verification uses the immutable Git object
named by `gate-policy/history/adr0039-generation17.json`; it never derives
historical identity from these live direct-authority inputs.
