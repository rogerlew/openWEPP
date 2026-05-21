# wepp-forest subsystem ownership patterns

Evidence: Static
Ran evidence: none

## Source set
- `/home/workdir/wepp-forest/src/main.for`
- `/home/workdir/wepp-forest/src/contin.for`
- `/home/workdir/wepp-forest/src/infile.for`
- `/home/workdir/wepp-forest/src/wshinp.for`
- `/home/workdir/wepp-forest/src/wshdrv.f90`
- `/home/workdir/wepp-forest/src/wshrun.f90`
- `/home/workdir/wepp-forest/src/hbp_mode2_bridge.f90`
- `/home/workdir/wepp-forest/src/hbp_legacy_bridge.f90`

## Observed ownership patterns
- Static: [DIRECT] `main.for` owns top-level execution mode switching: hillslope loop (`call contin`) then watershed dispatch (`call wshdrv`) with explicit HBP mode-2 open/close lifecycle handling (`/home/workdir/wepp-forest/src/main.for:357`, `:406`, `:423`, `:516`, `:519`).
- Static: [DIRECT] `main.for` and `infile.for` both own sidecar acquisition at orchestration boundaries (`gwcoeff.txt`, `phosphorus.txt`, `wepp_ui.txt`, irrigation sidecars, PMET, snow, frost) rather than physics kernels (`/home/workdir/wepp-forest/src/main.for:476`, `:499`, `/home/workdir/wepp-forest/src/infile.for:1544`, `:1566`, `:1612`, `:2060`, `:2136`).
- Static: [DIRECT] `contin.for` declares a very broad common-block mutation/read surface and acts as the hillslope continuous simulation coordinator (`/home/workdir/wepp-forest/src/contin.for:57`, `:59`, `:64`, `:71`, `:217`).
- Static: [DIRECT] `infile.for` owns watershed and hillslope file contracts for structure/channel/impoundment/management/slope/climate/soil plus version checks (`/home/workdir/wepp-forest/src/infile.for:373`, `:380`, `:409`, `:441`, `:480`, `:1653`, `:1707`, `:1874`).
- Static: [DIRECT] `wshinp.for` owns watershed topology and sidecar-based routing feature gates (`tcr.txt`, `lcwb.txt`, optional `chan.inp`), and hard-stops on invalid graph conditions (`/home/workdir/wepp-forest/src/wshinp.for:183`, `:199`, `:246`, `:269`, `:472`).
- Static: [DIRECT] `wshdrv.f90` owns watershed orchestration call ordering (`infile` -> `input` -> `wshinp` -> `wshini`) and embeds kernel-first dispatch gates (`wbk_imp_05`, `wbk_route_08`, `wbk_imp_06`) with fail-stop behavior on non-authoritative/non-OK returns (`/home/workdir/wepp-forest/src/wshdrv.f90:308`, `:312`, `:386`, `:391`, `:879`, `:885`, `:928`, `:937`, `:1259`, `:1266`).
- Static: [DIRECT] `wshrun.f90` uses typed route kernel interfaces, writes fluxes back into legacy storage fields, and hard-stops on route kernel failure (`/home/workdir/wepp-forest/src/wshrun.f90:15`, `:17`, `:141`, `:165`, `:167`, `:179`, `:313`).
- Static: [DIRECT] `hbp_mode2_bridge.f90` and `hbp_legacy_bridge.f90` isolate pass-file I/O adapters and surface explicit success/failure status messages to orchestrators (`/home/workdir/wepp-forest/src/hbp_mode2_bridge.f90:10`, `:20`, `:73`, `/home/workdir/wepp-forest/src/hbp_legacy_bridge.f90:20`, `:33`, `:35`).
- Static: [INFERENCE] wepp-forest ownership is primarily orchestration-centric with shared mutable global state; newer kernelized segments introduce typed status/flux boundaries but still write back to legacy storage containers.

## Subsystem ownership map

| subsystem | entrypoints | current owner | state ownership class | dependency direction |
| --- | --- | --- | --- | --- |
| top-level run shell | `main.for` | monolithic program shell | global mutable + mode flags | shell -> hillslope/watershed orchestrators |
| hillslope simulation orchestration | `contin.for` | hillslope coordinator | broad common blocks | orchestrator -> parser + kernels + outputs |
| parser/input layer | `infile.for`, `input`, `wshinp.for` | file-contract routines | common blocks + file handles + sidecars | parser -> state surfaces consumed by orchestrators |
| watershed orchestration | `wshdrv.f90` | watershed coordinator | global state + typed kernel inputs/status | orchestrator -> kernel dispatch + legacy compatibility routines |
| routing/impoundment kernels | `watbal_route_*`, `wbk_imp_*`, `wbk_route_*` via `wshdrv`/`wshrun` | typed kernel modules | typed flux/state/status structs | kernel -> authoritative fluxes -> orchestrator writeback |
| HBP adapter boundary | `hbp_mode2_bridge.f90`, `hbp_legacy_bridge.f90` | bridge adapter routines | explicit status + buffered shard data | orchestrator <-> bridge <-> pass file storage |

## Architecture implications for openWEPP
- Static: [INFERENCE] Keep parser/file/sidecar ownership out of physics kernels; parser and orchestration crates should own optional sidecar discovery and validation.
- Static: [INFERENCE] Treat typed status objects and explicit fail-stop semantics as transferable; avoid direct global mutable cross-subsystem sharing.
- Static: [INFERENCE] Preserve explicit orchestration sequencing points (`input`/`topology init`/`per-step kernel dispatch`/`summary accumulation`) as first-class architecture boundaries.
