# Worker Handoff

Status: complete
Evidence mode: static

Static: summary

- HPHYS0255 makes MOFE storage projection semantics explicit.
- Contracts now require separating aggregate `Area` from WB11/WB13 storage
  lineage.
- Runner manifests now publish
  `wb13_publication.storage_lineage_policy = "single-runtime-wb11-state"`.

Static: next work

- Do not implement area-weighted WB13 storage from static soil rows.
- If true MOFE aggregate storage is needed, scaffold a new contract-first
  package for per-OFE dynamic hydrology state migration.
- Use the HPHYS0255 asymmetric test as a guard against accidental OFE2/OFE3
  leakage into active unqualified WB11 aliases.
