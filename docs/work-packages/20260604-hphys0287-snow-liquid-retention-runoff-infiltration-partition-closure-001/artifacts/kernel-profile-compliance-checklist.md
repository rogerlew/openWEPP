# Kernel-Profile Compliance Checklist

Status: complete
Evidence mode: Static + Ran

Checks:
- [x] Package scoped under `docs/work-packages/`.
- [x] Contract-first sequencing followed.
- [x] Canonical `SC-*` contracts amended before production code.
- [x] Contract-derived tests added and pre-implementation failing gate recorded.
- [x] Production code uses typed guards, not silent canonicalize-and-proceed.
- [x] Dual review artifacts completed.
- [x] Review findings dispositioned.
- [x] Dual verification artifacts completed.
- [x] `cargo fmt --check` passed.
- [x] `cargo clippy --workspace --all-targets -- -D warnings` passed.
- [x] `cargo test --workspace` passed.
- [x] `cargo deny check` passed with existing warnings.
- [ ] Global `tools/release/check_sc_unit_compliance.sh` passed.

Open compliance item:
- SC unit compliance reports 219 broad registry/alias findings. HPHYS0287 remains `executed-hold`; this global backlog is not claimed closed.
