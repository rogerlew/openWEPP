# SVG And Transaction Security Review

Status: complete / pass

Evidence class: Static + Ran

The final retained-SVG path parses XML with `quick-xml 0.41.0` and rejects:

- scripts, animation, foreign content, images, style and metadata elements;
- event handlers, active/data URLs, nonfragment links, `xml:base`, namespace
  aliases, prefixed elements, CSS escapes, and unsupported style declarations;
- external or protocol-relative resources, declarations after sanitization,
  internal DTD subsets, unmatched/multiple roots, and text or CDATA outside the
  root;
- malformed XML, absent accessibility text, symlinks, special files, hash
  drift, path escape, and output collisions.

Only the exact inert Matplotlib default line-cap/join style is converted to
equivalent inherited presentation attributes. Style attributes use a finite
property/value allowlist. Four focused parser/sanitizer tests pass, including
adversarial CSS, namespace, declaration, and document-shape cases. The real
eight-figure CAL-09 assembly passes.

Admission uses exact conventional manifest spelling and exact nested
repository-relative source spelling. Regular-file confinement rejects symlink
and special-file inputs. `--check` now uses atomically created, invocation-owned
OS temporary directories and cannot create activatable recovery state.
`--apply` retains compare-and-swap, isolated candidate validation, atomic
exchange, installed-state verification, and rollback.

`cargo deny check` passes after upgrading from vulnerable `quick-xml 0.38.4`
to patched `0.41.0`; the repository's preexisting unmatched MIT-0 allowance is
the only warning.
