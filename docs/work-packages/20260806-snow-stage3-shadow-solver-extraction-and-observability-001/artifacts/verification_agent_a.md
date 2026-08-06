# Verification Agent A

Evidence class: `Static + Ran`.

Status: PASS at exact clean commit
`7efbed024751d54fcbf29375545cf0df76e06f57`; no findings.

- Baseline diff and manifest match exactly at 95 paths; the write set includes
  the declared review- and gate-expanded paths.
- The post-heavy delta from `56f85c3a` is exactly five package Markdown files,
  so clean heavy evidence remains current.
- Review findings, v128 bounded authority, CoE sole ownership, schema/default/
  public-output isolation, line counts, and all retained campaign holds pass.
- Assurance validates at generation `221f8e51`: three DRAFT, zero public,
  empty events, and null authority/approval/release/publication roots.
- Archived prompt is byte-identical: 3,590 bytes, SHA-256
  `313a5500776577c5dd4ed12e94974ee6a480b9d28028c4f0409343a54bc81533`.
- Roadmaps/catalog name only the frozen four-site audit next.

Ran: focused runtime/contract `36/36`, runner evaluation `5/5`, strict Binding
Exposure 10 rows, unit compliance, assurance validation, format, Markdown,
package validation, diff hygiene, and clean-head checks all pass. One malformed
positional filter initially selected zero tests; the corrected expression ran
`5/5`, so this was command selection rather than a product failure.
