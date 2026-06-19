# PERFDEEP05 Active Prompt

In `/home/workdir/openWEPP`, execute
`docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/`
end to end.

Remove the measured PERFDEEP03 lane-dense resynchronization hotspot. The
PERFDEEP03 opt-in daily H2637 hot loop must not call full
`sync_from_writeback_surface`; transfer input should update lane-dense state
directly, hot slot metadata should be cached, and dense writeback application
should avoid symbol-registry lookup where indexed payloads are available.

Do not activate by default. Do not expand the island or rewrite hydrology kernel
bodies unless this package is explicitly amended before implementation.

Required reading:

- `AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/package.md`
- `docs/work-packages/20260619-perfdeep04-profile-perfdeep03-lane-dense-no-go-001/artifacts/perfdeep04-profile-results.md`
- `docs/work-packages/20260619-perfdeep04-profile-perfdeep03-lane-dense-no-go-001/artifacts/perfdeep04-next-package-recommendation.md`
- `docs/work-packages/20260619-perfdeep03-persistent-lane-owned-dense-state-001/artifacts/review-claude-independent.md`

Required closeout:

- focused tests for dense transfer update and dirty-slot behavior;
- H2637 identity evidence;
- real H2637 endpoint and RSS;
- profile evidence that the PERFDEEP04 sync hotspot is gone;
- full Rust gates;
- markdown lint over package and touched docs.
