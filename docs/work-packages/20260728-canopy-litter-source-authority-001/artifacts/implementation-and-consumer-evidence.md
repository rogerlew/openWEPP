# Implementation and Consumer Evidence

Status: `PASS`

Evidence class: `Ran + Static`

The native-management schema now accepts independent needle and fine-woody
records only through path-based parsing. Every `complete` tissue binds:

- prescribed-scenario or exhaustive measured-daily mode;
- exact support, calendar, material and functional class;
- dry-mass state, horizontal-area basis, and
  `kg_dry_mass_m2_day`;
- site/OFE binding;
- separately authenticated vegetation classification plus source and
  executable SHA-256 identities; and
- fine-wood diameter and bark treatment.

`not_represented` and authority-backed `not_applicable` are explicit states
and cannot carry numeric payloads. Interval and derived objects are rejected
by this identity-only increment until a successor admits typed transformation
inputs/digests and temporal authority. Noncanonical bytes, invalid
dates/masses, path escape, digest drift, incompatible material, site, or OFE
fail closed.

The verified forcing survives management parsing, schedule/crop projection,
daily date lookup, and the real decomposition input. Research output publishes
leaf, needle, fine-woody, total, per-tissue status/mode, and before/after
surface/interrill/rill operands.

Unrepresented/inapplicable tissue publishes a null operand, not numeric zero,
and aggregate source completeness remains explicit.

The real native-forest execution fixture supplies day-1 needle `0.002` and
fine-woody `0.003 kg/m2/day`. Its test independently reconstructs:

```text
Q = L_leaf + 0.002 + 0.003
S_next = (S_before + Q) * f
I_next = (I_before + Q) * f
R_next = (R_before + Q) * f
```

for the no-action fixture. It independently reconstructs weighted ground mass,
interrill/rill/composite cover, and residue depth, then proves that the exact
interrill/rill cover operands are read by active erosion and exact depth is
read by frost. A source guard proves the sum enters the real decomposition
input once with no downstream re-addition. The run also proves outside-support
failure and null publication for unrepresented tissue.

Ran:

- `cargo nextest run -p openwepp-runner
  native_forest_yaml_executes_through_the_direct_production_consumer`
  — 1 passed;
- combined terminal focused run of that consumer plus the native source guard
  — 2 passed.
