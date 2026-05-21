# wepp-forest kernel requirements extract

Evidence: Static
Ran evidence: none

## Extracted requirements

### WF-KR-001 Typed process state contracts
- Static: [DIRECT] `watbal_process_types` defines explicit timestep/geometry/input/storage/flux/closure/status records (`/home/workdir/wepp-forest/fpm-src/watbal_process_types.f90:59`, `:71`, `:80`, `:122`, `:166`, `:180`, `:189`).
- Static: [INFERENCE] openWEPP should require typed state carriers per subsystem phase and prohibit implicit cross-phase mutation via globals.

### WF-KR-002 Stage-oriented kernel pipeline
- Static: [DIRECT] `watbal_process_kernels` exports staged kernels `wbk01..wbk09` plus domain-specific helpers, indicating explicit phase decomposition (`/home/workdir/wepp-forest/fpm-src/watbal_process_kernels.f90:21`, `:29`, `:31`).
- Static: [INFERENCE] openWEPP should define stable phase order contracts (normalize -> bounds -> ET -> percolation -> lateral -> drainage -> reconcile -> closure).

### WF-KR-003 Runtime status taxonomy
- Static: [DIRECT] status objects include `ok`, `finite_ok`, `domain_ok`, `boundary_class`, `clamp_class`, and `message_id` (`/home/workdir/wepp-forest/fpm-src/watbal_process_types.f90:189`, `:196`).
- Static: [INFERENCE] openWEPP should standardize a typed status schema across hillslope and watershed kernels with machine-readable message IDs.

### WF-KR-004 Geometry and finite checks are first-class
- Static: [DIRECT] kernels explicitly reject invalid geometry/non-finite values and set status fields (`/home/workdir/wepp-forest/fpm-src/watbal_process_kernels.f90:142`, `:159`, `:160`, `:163`).
- Static: [INFERENCE] openWEPP should make finite/domain checks mandatory gates before authoritative writeback.

### WF-KR-005 Orchestrator authority and hard-stop policy
- Static: [DIRECT] `wshdrv`/`wshrun` abort execution on kernel dispatch failures rather than silently continuing (`/home/workdir/wepp-forest/src/wshdrv.f90:885`, `:937`, `:1266`, `/home/workdir/wepp-forest/src/wshrun.f90:179`, `:313`).
- Static: [INFERENCE] openWEPP should preserve explicit fault surfacing; no default-value fallback for non-finite/domain failures.

### WF-KR-006 Adapter isolation for compatibility bridges
- Static: [DIRECT] HBP bridge modules encapsulate pass-file reads/writes and return explicit success/message tuples (`/home/workdir/wepp-forest/src/hbp_mode2_bridge.f90:16`, `:21`, `:57`, `/home/workdir/wepp-forest/src/hbp_legacy_bridge.f90:33`, `:35`).
- Static: [INFERENCE] openWEPP should isolate legacy compatibility into adapter crates, not kernel crates.

### WF-KR-007 Topology validation prior to runtime dispatch
- Static: [DIRECT] watershed topology checks are enforced in `wshinp` with immediate stop conditions (`/home/workdir/wepp-forest/src/wshinp.for:214`, `:217`, `:265`, `:273`).
- Static: [INFERENCE] openWEPP should require topology validation and graph closure checks before entering simulation loops.

### WF-KR-008 Summary accumulation kernelization
- Static: [DIRECT] impoundment daily/monthly/yearly/EOS summary paths are routed through typed `wbk_imp_06_summary_accumulator_execute` before legacy writers (`/home/workdir/wepp-forest/src/wshdrv.f90:1259`, `:1315`, `:1372`, `:1436`).
- Static: [INFERENCE] openWEPP should kernelize summary accumulation separately from reporting sinks.

### WF-KR-009 Mode-aware scheduler context
- Static: [DIRECT] timestep context carries scheduler/requested/effective mode strings (`/home/workdir/wepp-forest/fpm-src/watbal_process_types.f90:66`, `:67`, `:68`).
- Static: [INFERENCE] openWEPP should explicitly carry runtime mode context (daily/hourly/watershed) into kernels for deterministic auditability.

### WF-KR-010 Canonical WEPP symbol continuity at kernel boundaries
- Static: [DIRECT] legacy symbols remain visible in orchestration/writeback (`runoff`, `runvol`, `sbrunf`, `drainq`, `sep`, `st`, `frzw`, `frozen`) while kernel structs provide typed wrappers (`/home/workdir/wepp-forest/src/wshrun.f90:124`, `:148`, `:173`, `/home/workdir/wepp-forest/fpm-src/watbal_process_types.f90:166`, `:172`).
- Static: [INFERENCE] openWEPP should keep canonical symbol names in science-contract tables and map them to Rust field names through explicit alias metadata.

## Priority extraction for openWEPP architecture
- Static: [INFERENCE] Highest priority requirements: WF-KR-001, WF-KR-002, WF-KR-003, WF-KR-005.
- Static: [INFERENCE] Secondary requirements: WF-KR-006, WF-KR-007, WF-KR-008, WF-KR-009, WF-KR-010.
