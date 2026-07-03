# Codex Review and Finding Disposition

Evidence classes: Static + Ran. Review type: local Codex adversarial review.
No delegated subagent review was used in this execution pass.

## Findings

### High - Source-intent guard rejected valid saturated p313 input

Static + Ran: The first p313 direct-runtime run failed because the new evaluator
rejected layer water storage greater than upper-limit storage. That was not a
source-intent guard: `REF-SUBHYD-KSATADJ-INTENT` caps derived `avsat`; it does
not reject saturated initialized storage before forming the top-two average.

Disposition: accepted / fixed. The over-strict `theta <= upper_limit` and
`field_capacity <= upper_limit` rejects were removed, and a regression vector
now proves saturated storage is capped rather than rejected.

### Medium - Touched runner builder exceeded the 3000-line closure threshold

Static + Ran: The WB14 insertion point was the pre-existing
`00_builders_and_authority.rs` monolith at 4144 lines. Keeping the change there
would violate work-package line-count governance.

Disposition: accepted / fixed. The direct-publication helper was mechanically
split into focused include files. The original file is now 1923 lines, and all
touched helper files are below 2000 lines.

### Medium - Existing source guards assumed the pre-refactor monolith path

Ran: Full nextest initially failed snow-density and Paradigm-2 source guards
because selector and trace markers moved from `00_builders_and_authority.rs` to
split helper files.

Disposition: accepted / fixed. Guard read targets were updated to the new
source ownership without weakening marker assertions. Cross-file guards that
need constructor-plus-selector proof now read the combined relevant sources.

## Accepted Checks

- Static: The evaluator implements the ratified source-intent algorithm, not the
  deleted symbol-map implementation as authority where they differ.
- Static + Ran: 9001, 9002+, 9003, inactive, missing-operand, missing-layer, and
  saturated-storage vectors are covered.
- Static + Ran: Active `ksatadj` overrides the base WB14 fallback path while
  remaining limited by a lower positive frost cap.
- Ran: p313 disturbed-burn forest high-severity loam runs end-to-end through the
  direct runtime, produces HBP/loss/manifest outputs, and its manifest counter
  proves the production `ksatadj` evaluator runs only when the fixture's
  soil-side flag is active.
- Ran: Full workspace nextest passed after the source-guard path updates.

No remaining correctness findings are open from this local review.
