# Rust Line-Count Disposition

Status: `WARN ACCEPTED`

Evidence class: `Static`

| File | Prospective expansion base | Corrected implementation | Threshold |
|---|---:|---:|---|
| `crates/openwepp-gate-planner/src/pre_heavy.rs` | 2,052 | 2,762 | 2,000 `WARN`; 3,000 refactor |
| `crates/openwepp-gate-planner/src/resume.rs` | 2,065 | 2,119 | 2,000 `WARN`; 3,000 refactor |

Both files remain below the 3,000-line closure threshold.
`pre_heavy.rs` keeps the existing pathname APIs and transition-only
bound-ledger variants adjacent so reviewers can directly verify parity,
unchanged public behavior, and the absence of transition pathname reopens.
Splitting those variants during the security correction would enlarge the
write set and obscure that audit.

`resume.rs` adds only the retained-byte entry point beside the existing
pathname entry point and funnels both through the same internal parser. Keeping
that small compatibility seam adjacent makes the unchanged recovery selection
logic directly auditable.

Follow-on split intent: the gate-planner maintainers must prepare prospective
mechanical-refactor packages before further substantive growth:

- split ledger admission, chain I/O, and bound-handle tests from
  `pre_heavy.rs` behind unchanged exports;
- split recovery-ledger parsing/candidate selection from `resume.rs` behind the
  existing pathname and retained-text entry points.

Each split must preserve comparator/gate behavior, carry exact export and test
parity, and return its source file below 2,000 lines. Reaching 3,000 lines
remains a hard closure blocker; these WARN dispositions do not approve an
exception to that threshold.
