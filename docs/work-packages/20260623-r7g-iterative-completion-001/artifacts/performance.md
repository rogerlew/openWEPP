# Performance

Status: HOLD; endpoint performance is not proven green after fine-layer carry
preservation.

Evidence class: Ran.

Constants:

- H2637 OFE-days: `235961`.
- Legacy WEPP wall time from prior R7G evidence: `9.12 s`.
- `<=10x` wall-time budget: `91.2 s`.

Binary hashes from `/tmp/r7g-cont-h2637/manifests/direct-default7.json`:

- Binary: `fd1886bb36c5295cdcd4baa368d48a2ab7c4d3eb14ce71708f34cf1fb1593bf1`
- Sidecar: `d144412a12b17fb1fe2805bb8a62bc352c7f828cbbaf45b426df4455897266b1`
- Source commit recorded by manifest: `7cfdc0e016b4ea18c43de778befa4779d002dff4`

Same-binary H2637 matrix:

| Mode | Flag | Exit | Seconds | RSS KiB | us/OFE-day | x legacy | Counter summary |
| --- | --- | ---: | ---: | ---: | ---: | ---: | --- |
| default-disabled compatibility | none | `0` | `643.02` | `228604` | `2725.11` | `70.51x` | compatibility; scheduler kernel executed |
| explicit rollback compatibility | `--compatibility-runtime` | `0` | `636.68` | `229708` | `2698.24` | `69.81x` | compatibility; scheduler kernel executed |
| direct default candidate | `--direct-default-candidate` | `0` | `13.21` | `871664` | `55.98` | `1.45x` | `compatibility_edge_invocations=0`; `day_frame_commits=235961` |
| explicit direct production | `--direct-production-executor` | `0` | `13.53` | `870900` | `57.34` | `1.48x` | `compatibility_edge_invocations=0`; `day_frame_commits=235961` |
| direct default recapture | `--direct-default-candidate` | `0` | `13.16` | `871668` | `55.77` | `1.44x` | parity capture rerun |

Active-frost performance remediation loop:

| Label | Seconds | RSS KiB | x legacy | Disposition |
| --- | ---: | ---: | ---: | --- |
| `direct-default-frost5` | `163.88` | `947704` | `17.97x` | red; active frost endpoint with redundant R4A solves |
| `direct-default-frost6` | `122.43` | `948084` | `13.42x` | red; no-material R4A skip helped |
| `direct-default-frost7` | `94.08` | `941936` | `10.32x` | red; frost template shrink nearly closed |
| `direct-default-frost10` | `87.11` | `942324` | `9.55x` | green; zero-prior no-freeze fast path |
| `direct-default-frost11` | `89.88` | `941936` | `9.86x` | green; retained source state |
| `direct-default-frost12` | `101.16` | `941936` | `11.09x` | rejected; no parity improvement |
| `direct-default-frost28` | `107.96` | `941936` | `11.84x` | red; prior-snow ordering trace run |
| `direct-default-frost29` | `188.57` | `941936` | `20.68x` | red; active zero-material fine carry preserved |
| `direct-default-frost30` | `195.27` | `942324` | `21.41x` | red; latest measured endpoint before final no-material consumer safeguard |

Latest retained direct-frost manifest:
`/tmp/r7g-cont-h2637/manifests/direct-default-frost11.json`.

Latest retained direct-frost counters:

- `compatibility_edge_invocations=0`
- `day_frame_commits=235961`
- `phase_span_runs=5191143`
- `direct_compute_operations=5225374`
- `direct_state_mutations=5448916`

Current disposition:

- The earlier `89.88 s` run proved the executor can be fast when fine-layer
  frost carry is not fully preserved.
- Preserving active zero-material fine/shadow carry exposed architecture cost:
  the latest measured endpoint is `195.27 s`, well above the `91.2 s`
  `<=10x` budget.
- The final no-material R4A consumer safeguard was added after the
  `direct-default-frost30` run and has not been full-H2637 measured.
- HOLD requires a follow-up frost stateful sub-solver; performance must be
  re-established there with `compatibility_edge_invocations=0`.
