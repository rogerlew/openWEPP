# Review Agent QA

Evidence class: `Static + Ran`

Final recommendation: `PASS-WITH-NOTES`.

Accepted QA findings covered stale assurance identity, v126/v127 and generation
prose, exact write-set/receipt narration, and operator/Binding Exposure test
strength. All substantive findings closed at generation `34f2f80e`; the final
test further replaced suffix counting with exact full-row equality.

The remaining terminal note was to distinguish tool affected paths from actual
byte changes and enumerate the terminal diff. Package narration now states that
the unrelated groundwater/forest locks were included in the affected set but
their bytes and roots did not change. The exact path inventory is recorded at
closure.

Ran: focused tests, formatting, focused Clippy, Markdown lint, `cargo deny`,
assurance validation, strict Binding Exposure, and diff checks.
