# Independent Review B

Status: `complete`

Evidence class: `Static`

Disposition: `HOLD`

## Scope

Independently reviewed the terminal draft for narrative shape and register,
self-contained links, coefficient-guide usability, calibration order,
downstream-compensation safeguards, reference style, DOC/ASSURE separation,
and write-set discipline. Read the root and work-package instructions, the
package contract, and the usersum authoring guide. Inspected the active
management-schema field definitions and residue-depth projection path where
needed to test reader-facing configuration claims.

## Findings

### B-01 — Major: coefficient contract is not complete in the public guide or ledger

The package requires the public guide *and* authority ledger to record, for
every coefficient, a user-facing label, equation location, value status,
calibration target and minimum observation needs, identifiability warnings,
and transfer/downstream-compensation cautions
(`package.md:116-129`). The public tables
(`usersum/openwepp-canopy-phenology.md:149-188`) provide field, meaning,
units, hard domain, range class, and a short effect, but do not explicitly
record per-field value status or equation location. Calibration targets and
identifiability are discussed later by coefficient groups, but the guide does
not bind all of those requirements back to every row.

The CSV header
(`artifacts/coefficient-authority-ledger.csv:1`) likewise has no explicit
columns for user-facing label, equation location, minimum observation needs,
or scale. Several concepts are partially embedded in `model_role`,
`calibration_target`, or `warning`, but that is not the explicit, auditable
inventory the coefficient-guide contract requires.

Required disposition: add the missing ledger fields and populate every row;
then make the public guide explicitly bind each listed coefficient to its
value status and equation/calibration guidance. Compact group-level notes are
acceptable where they unambiguously cover every row.

### B-02 — Major: the residue mass-to-depth entry is not actionable or exact

The coefficient contract specifically calls for the residue mass-to-depth
conversion when user-configurable and exact active field names
(`package.md:98-110`, `package.md:116-120`). The ledger instead uses the
placeholder `<derived residue mass-to-depth conversion>`
(`artifacts/coefficient-authority-ledger.csv:18`). The public text says the
conversion comes from “management initial-condition conversion lineage” and
asks users to check mass, cover, material class, and depth
(`usersum/openwepp-canopy-phenology.md:190-194`), but it does not name the
actual user inputs or clarify that native YAML has no standalone residue-depth
coefficient.

The active path derives initial depth and then the conversion from the
native-forest initial-condition and plant inputs (including surface-residue
mass/cover plus cover-factor and diameter lineage). As written, a user cannot
locate the controlling YAML inputs and could reasonably infer that depth is a
directly entered native-forest field.

Required disposition: replace the placeholder with the exact input and
derived runtime field lineage, and revise the public paragraph to distinguish
user-entered fields from the derived
`residue_depth_conversion_m_per_kg_m2`. State which observations must be
coherent without implying that a standalone YAML depth value exists.

### B-03 — Minor: two reader-facing terms remain internal or undefined

“OFE” appears without expansion in the litter-forcing and calibration sections
(`usersum/openwepp-canopy-phenology.md:201`, `:220`). Forest managers are part
of the declared audience, so define “overland-flow element (OFE)” at first use.
The literal status token `not_represented` at line 203 is useful only if the
reader knows it is a YAML status value; introduce it as such.

Required disposition: expand OFE once and identify `not_represented` as the
configuration status token.

### B-04 — Minor: one bibliography entry is not APA-style

The usersum guide requires an APA-style reference list. The Lim entry uses
“Lim, H., et al.” in the bibliography
(`usersum/openwepp-canopy-phenology.md:307-309`), which is an in-text
abbreviation rather than an APA-style author list.

Required disposition: replace `et al.` with the published author list in the
reference entry, following the repository's chosen APA convention.

## Passing observations

- The document has a clear narrative argument: the compatibility limitation
  frames the native-forest need, then weather-to-state causality leads to
  downstream consequences, calibration, and interpretation.
- The audience/version lines, revision log, restrained formatting, and closing
  interpretation conform to the usersum narrative shape.
- The six timing ranges and mature-LAI interval are tightly scoped to their
  named evidence. Examples and search domains are not promoted to defaults or
  physiological ranges.
- Calibration follows the required sequence and explicitly prohibits tuning
  canopy coefficients to hide snow, frost, runoff, erosion, sediment,
  litter-source, or decomposition residuals
  (`usersum/openwepp-canopy-phenology.md:211-252`).
- Harvard and tropical-dry-forest limitations remain qualitative. The draft
  does not duplicate claim-bearing assurance tables, reproduction procedures,
  or quantitative evaluation results.
- The only narrative cross-link resolves within `usersum`
  (`usersum/openwepp-canopy-phenology.md:122-123`).
- The inspected working-tree paths are within the declared write set; no
  production, schema, contract, prior-evidence, or assurance-publication
  surface is modified. `git diff --check` passed during this review.

## Closure judgment

The narrative is strong and the calibration/assurance boundary is legitimate,
but B-01 and B-02 are direct failures of the package's mandatory
coefficient-guide contract. CANOPY-DOC-01 should remain on `HOLD` until those
findings are corrected and independently verified.
