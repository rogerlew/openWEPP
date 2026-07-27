# Required Reading Map

Status: `COMPLETE`

Evidence class: `Static: complete-file reading; Ran: instruction discovery,
source identity, and checksum checks`

Applicable instructions:

- `AGENTS.md`
- `docs/work-packages/AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `docs/standards/AGENTS.md`
- `docs/standards/prompt-wording-guidance.md`
- `docs/standards/testing-and-gate-strategy.md`

Package and calibration authority:

- this package and kickoff prompt;
- CANOPY-CAL-04 final disposition, intent plan, objective/operator, and search
  plan;
- CANOPY-CAL-04/05 admission ledger, authority-gap ledger, partition/objective,
  and exact timing windows;
- CAL-03 pre-calibration protocol and observation ledger;
- `SC-PLANT-001`, especially CP-GSI02 and `INV-PLANT-033..037`;
- canopy assurance roadmap.

Source and implementation bindings:

- retained Hubbard EDI 51.16 EML and CSV;
- canopy observation and authority-admission checksum manifests;
- Daymet V4 daily-data guide and Single Pixel service documentation;
- native runner day-input builder and saturation-vapor-pressure function;
- native plant-phenology photoperiod implementation;
- protected Hubbard `.cli` and its fixture manifest, comparison-only.

The combined admitted timing ledger, including Harvard role metadata, was read
to select only `CALIBRATION` rows. No Harvard interval was joined or analyzed,
and the raw Harvard source, modeled results, and downstream consumer results
were not read for analysis or selection.
