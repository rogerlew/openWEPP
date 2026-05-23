# PL11 Event Runtime Projection Contract

Status: `complete`
Evidence mode: `Static + Ran`

## Implemented Projection Authority

Static:
- Annual/fallow branch projects deterministic slot/crop symbols for:
  - `resmgt`
  - `jdherb`, `jdburn`, `jdslge`, `jdcut`, `jdmove`
  - `fbrnag`, `fbrnog`, `frcut`, `frmove`
- Perennial branch projects deterministic slot/crop symbols for:
  - `mgtopt`, `ncut`, `ncycle`
  - indexed cuts: `cutday_{event:04}`
  - indexed grazing cycles: `gday_{cycle:04}`, `gend_{cycle:04}`, `animal_{cycle:04}`, `bodywt_{cycle:04}`, `area_{cycle:04}`, `digest_{cycle:04}`
- Primary merged-surface aliases are emitted for first-slot annual controls (`jdherb/jdburn/jdslge/jdcut/jdmove/fbrnag/fbrnog/frcut/frmove`).

## Typed Guard / Failure Posture

Static:
- Day domain validation (`HS-RUNTIME-E-046`) enforces `1..366` with explicit `0` allowance only where contract-authorized.
- Annual extension mismatch validation (`HS-RUNTIME-E-047`) enforces `resmgt` to extension payload compatibility.
- Cardinality validation (`HS-RUNTIME-E-048`) enforces non-empty required families (`ncut` for `mgtopt=1`, `ncycle` for `mgtopt=2`).
- Grazing window validation (`HS-RUNTIME-E-049`) enforces `gday < gend` per cycle.
- Numeric domain validation (`HS-RUNTIME-E-050`) enforces fraction and positive-value domains.
- Unsupported payload combinations (`HS-RUNTIME-E-051`) hard-fail invalid branch/payload shape combinations.

Ran:
- PL10b conformance tests that previously failed now pass with this failure posture in place.
