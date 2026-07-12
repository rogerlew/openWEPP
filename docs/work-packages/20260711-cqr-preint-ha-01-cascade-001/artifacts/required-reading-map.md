# Required Reading Map

Evidence class: **Ran** for instruction discovery; **Static** for applicability.

| Tier | Paths | Rationale / status |
| --- | --- | --- |
| Core | `AGENTS.md`; `crates/AGENTS.md`; `docs/work-packages/AGENTS.md`; package; High-A ExecPlan; campaign execution contract; mechanical/CQR/test-enhancement guides; ADR-0021; prompt standard | Read before scaffold; process and CQR authority |
| Core | `docs/specifications/science-contracts/AGENTS.md`; `SC-OFEROUTE-001`; `cascade.rs` and inline tests | Read before scaffold; science, handoff, conservation, and target authority |
| Conditional | local CI standard; unit governance; pinned baseline | Read on named trigger; local CI already read, other triggers absent |
| On-demand | `kinematic_wave.rs`, `profile.rs`, lane/executor consumers | Consumer-path and focused-test context |

`tools/agents/find-agents` returned root `AGENTS.md` plus `crates/AGENTS.md` for
the Rust target, and root plus `docs/work-packages/AGENTS.md` for package files.
Core byte budget is 326,401 (`WARN`, justified by the large active contract).
