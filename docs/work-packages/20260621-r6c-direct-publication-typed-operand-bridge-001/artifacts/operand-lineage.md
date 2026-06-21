# Operand Lineage

Evidence mode: Static.

R6C must record the field-level lineage for:

- HBP: event date, duration, peak runoff, detachment, deposition, sediment
  concentration, contributor count.
- WAT: daily water-balance fields, calendar identity, OFE/lane identity,
  climate/runoff/subsurface/storage/projection operands.
- PASS: daily pass volume basis, lateral/subsurface volume basis, peak,
  sediment, run/OFE identity.
- Loss: static run geometry fields, event totals, warning metadata.
- Manifest: schema ID, output policy, direct runtime counters, checksums,
  provenance, warning metadata.

For each accepted field, execution must record:

- direct producer;
- direct frame field;
- unit and basis;
- rejected compatibility alias classes;
- anti-alias fixture;
- independent reconstruction rule.

## Execution Disposition

R6C could not accept any output-family operand because the production climate
lifecycle does not retain direct day/publication producers.

Blocked producer families:

- HBP/PASS peak runoff, duration, detachment, deposition, and concentration:
  no retained production direct erosion/publication handoff.
- WAT/PASS water-balance operands: direct runtime has typed shadow/projection
  structs, but production climate execution does not retain them as publication
  authority.
- Loss and manifest operands: static inputs exist, but cutover still requires a
  direct publication execution provenance/checksum projection linked to retained
  production direct counters and output policy.

R6C therefore preserves the R6 ledger and stops before accepting wrapped
compatibility aliases.
