# Contract Implementation Evidence

Status: `complete`.

Evidence class: Static.

Created and registered
`docs/specifications/science-contracts/contracts/SC-SNOWENERGY-001.md`.
Version 1 binds:

- corrected Dilley-O'Brien plus Unsworth-Monteith atmospheric longwave;
- daily clearness cloud inference and fail-closed polar-night behavior;
- `f_sky=(1-C)^1.6` from effective canopy cover;
- complementary sky/canopy incoming longwave;
- effective-unity emissivity and outgoing snow longwave;
- positive-toward-snow net-longwave sign;
- typed state, unit, guard, alias, and consumer obligations; and
- explicit EB-03 runtime prerequisites.

The contract is registered as `in_review / draft / static`. No executable
surface was changed and no production behavior is claimed.
