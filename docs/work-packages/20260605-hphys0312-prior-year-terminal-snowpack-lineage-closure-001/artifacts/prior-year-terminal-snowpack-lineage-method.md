# Prior-Year Terminal Snowpack Lineage Method

Status: complete

Evidence mode: ran

Static:

- Input ledger: HPHYS0311 source-line parity ledger.
- Baseline evidence: HPHYS0305 fixed-comparator `H305_S_OUT` observe lane.
- openWEPP evidence: HPHYS0305 `post_wb13` trace rows.
- Material threshold: `0.0005 m` depth or `0.5 kg m^-3` density.
- Source-line evidence must be present before ledger generation.

Ran:

- Filtered HPHYS0311 to six `prior-year-terminal-state-hold` groups.
- Scanned each prior calendar year from day 1 hour 1 through terminal day hour
  24, preserving the first material paired divergence and the preceding
  within-tolerance state when present.
- Verified terminal deltas match HPHYS0311 inherited terminal deltas.
- Classified rows without authorizing production or downstream compensation.
