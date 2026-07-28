# Frozen Cohort And Snapshots

Evidence class: Static.

The cohort was frozen before trials. Each package is evaluated in `pre-edit`,
`working-tree`, and `terminal` mode, producing 18 paired cases.

| ID | Class | Package | Scaffold snapshot | Terminal snapshot |
| --- | --- | --- | --- | --- |
| DOC-1 | documentation | `20260727-gate-planner-advisory-linter-roadmap-001` | `57f5f6f1f1649022d47124de856108c6a11cc483` | `bac26bface8e6a224545137ede030a0e2931afff` |
| DOC-2 | documentation/governance | `20260727-gate-planner-governance-authority-alignment-001` | `d7a6f8aeabc0ecfd737acae46d26ec20b6c9e4be` | `0f851bc89a82044cc8a5fccef64be912403cceaf` |
| NKR-1 | non-kernel Rust | `20260608-owcmp01-comparator-cli-implementation-001` | `ff4f44b9e403f37a6cc1190f2e6ed26f076213a6` | `6b6acf77d5fd258cdad71ba111fa75eb3bd87bb5` |
| NKR-2 | non-kernel Rust test | `20260727-assurance-v2-amendment-contract-clippy-line-disposition-001` | `1eb56a4dfcf2cb91046d053d8562ef67049aacf0` | `86f14da61997f1b5d29f2eee5f7f8cf470a4c94e` |
| KER-1 | kernel/science | `20260727-cal04b-native-gsi-canopy-height-coherence-hold-lift-001` | `f4b3db6c17f25d9dfe969825c672309443963ac4` | `dd3b2a59018bd9a39999f9b263b07351afc34290` |
| CAL-1 | calibration science | `20260727-canopy-cal-04b-calibration-readiness-and-ensemble-execution-001` | `28d842c445d0e21d21e62b82e6f28762e788e7d9` | `72e433d16b4f9c35f2bb05cee8c7d92b1e16108d` |

For `pre-edit`, use the scaffold snapshot. For `working-tree`, clone the
scaffold snapshot and apply the exact scaffold-to-terminal binary diff without
committing. For `terminal`, use the terminal snapshot. The manual and linter
arms of a pair use separately created but byte-equivalent repositories.

The current linter source is invoked by absolute path from outside the trial
repository. It is not copied, installed, or committed into historical
snapshots. If the historical snapshot lacks a required current policy input,
that unavailable analysis is measured as tool behavior; the manual route
continues.
