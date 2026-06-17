# PERFIDX02 Worker Handoff

Status: complete 2026-06-16
Evidence mode: **Static** + **Ran**

PERFIDX02 completed ADR-0022 Stage 2.

What changed:

- Added `IndexedSurface` and `IndexedWritebackSurface` as sparse sorted
  working-set stores in `openwepp-kernel-contract`.
- Added an env-gated indexed shadow report hook in the hillslope runner.
- Tightened production symbol registry enumeration for PL indexed cut/grazing
  families from parsed reachable dimensions.
- Kept BTreeMap surfaces authoritative.

What is proved:

- Sparse working-set clone is a real H2637-scale win: `69.882x` without UI and
  `54.096x` with UI.
- Shadow round-trip equality passed on H2637 both UI variants and OFE1-OFE5.
- Tightened registry completeness passed on H2637 both UI variants and OFE1-OFE5.
- Bit identity and determinism passed.
- Full cargo gates passed.

Next package:

```text
PERFIDX03-indexed-surface-authority-001
```

Stage 3 must still design and prove the authority flip. It must not assume that
the compact-value candidate from PERFIDX02 is a complete representation; the
complete Stage 2 shadow representation is sparse sorted id/value pairs.
