# Rust Line-Count Disposition

Status: `WARN ACCEPTED`

Evidence class: `Static`

| File | Prospective expansion base | Corrected implementation | Threshold |
|---|---:|---:|---|
| `crates/openwepp-gate-planner/src/pre_heavy.rs` | 2,052 | 2,762 | 2,000 `WARN`; 3,000 refactor |

The file remains below the 3,000-line closure threshold. This correction keeps
the existing pathname APIs and the transition-only bound-ledger variants
adjacent so reviewers can directly verify parity, unchanged public behavior,
and the absence of transition pathname reopens. Splitting those variants during
the security correction would enlarge the write set and obscure that audit.

Follow-on split intent: the gate-planner maintainers must prepare a prospective
mechanical-refactor package before further substantive growth of
`pre_heavy.rs`, extracting ledger admission, chain I/O, and bound-handle tests
behind unchanged exports. The split must preserve comparator/gate behavior,
carry exact export and test parity, and return the source file below 2,000
lines. Reaching 3,000 lines remains a hard closure blocker; this WARN does not
approve an exception to that threshold.
