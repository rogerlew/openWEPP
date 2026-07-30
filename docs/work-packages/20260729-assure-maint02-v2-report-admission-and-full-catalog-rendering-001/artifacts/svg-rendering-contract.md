# Retained SVG Rendering Contract

Status: frozen before production edits

Evidence class: Static

## Typed Representation

The existing `linear_magnitude_bars` figure remains unchanged. The added
`retained_svg` variant requires:

- ordinary figure identity, ownership, generation procedure, caption, and
  alternative text;
- empty result and value-binding lists unless the figure itself bears declared
  strict-result values;
- `research_object_id` naming one declared `public_safe` SVG object; and
- `ancillary_object_id` naming one declared `public_safe` Markdown sidecar.

The schema is an explicit union of complete generated and retained variants;
existing generated-figure source bytes acquire no placeholder fields.

The SVG and sidecar remain independently content-identified research objects.
Rendering counts both as used so exact-use validation remains fail closed.

## SVG Admission

The assembler reads the identified source bytes and parses XML locally. It
requires:

- UTF-8 XML with one root element in the SVG namespace named `svg`;
- nonempty `title` and `desc` descendants and `role="img"` on the root;
- no `script`, `foreignObject`, embedded HTML, processing instruction, DTD,
  entity declaration, event-handler attribute, or animation/link element;
- only fragment-local `href`/`xlink:href` references, no URL-bearing style,
  CSS import, or external-resource attribute;
- no absolute path or external scheme in any attribute value; and
- exact staged bytes equal to the declared identified source.

The parser never fetches, renders, or executes SVG content.

## Output

The assembler verifies the exact identified source object, removes only a
leading XML declaration, external DOCTYPE, and non-rendering metadata block,
then injects `role="img"` and escaped authored `title`/`desc` elements before
validating the resulting standalone SVG. This bounded deterministic
sanitization is required because the retained Matplotlib artifacts carry the
standard SVG 1.1 external DOCTYPE and RDF metadata but no embedded accessible
title or description; those declarations are not admitted to reader-facing
output. The assembler writes the sanitized bytes to
`figures/<figure-id>.svg`, emits portable Markdown image
syntax with the authored alternative, emits the escaped caption, and emits a
visible ancillary-information link to the staged sidecar in
`research-objects/`. Internal subsets, declarations elsewhere, output
collisions, or second rendering fail. Named and all-report builds use the same
function and must be byte equal.

CAL-09 assigns F1, F3, F4, F5, F6, and F8 to the main manuscript and F2 and F7
to the supplement. The existing transfer-range bar figure remains in the main
report. No figure science or source SVG bytes change.
