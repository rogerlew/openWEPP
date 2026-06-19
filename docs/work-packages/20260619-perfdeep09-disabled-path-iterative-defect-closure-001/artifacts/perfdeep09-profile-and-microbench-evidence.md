# PERFDEEP09 Profile and Micro-Benchmark Evidence

Status: complete.
Evidence class: Static + Ran.

No new `perf record` run was required before the retained edit because
PERFDEEP04 already supplied matched default-path profile attribution and the
PERFDEEP09 same-machine control reproduced the blocker.

Prior profile evidence used:

- PERFDEEP04 default profile:
  `ensure_no_overflow_indexed_symbols_for_decomposition` accounted for
  `9.18%` children / `7.72%` self.
- Same default profile also showed the path as map/string/symbol-heavy, with
  `BTreeMap::insert` and symbol-state lookups in the hot set.

Static attribution confirmed:

- The old guard built one prefix per root and scanned every `state_surface` key
  once per root.
- Perennial decomposition modes checked seven roots:
  `cutday`, `gday`, `gend`, `animal`, `bodywt`, `area`, `digest`.
- The retained candidate reduces seven full scans to one slot/crop-filtered
  scan, stores first overflow per root, and preserves old root-order error
  precedence.

Rejected attribution:

- Candidate 1 targeted `SymbolRegistry::id_of` by changing the private reverse
  lookup to `HashMap`. It screened slower (`689.30 s`) and was reverted.

Why evidence was sufficient:

- Mechanism was ranked by a prior default-path profile, reproduced by static
  source inspection, in-package, guard-only, and verifiable with focused tests
  plus endpoint timing.
