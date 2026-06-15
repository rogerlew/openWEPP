# Error Surface Equivalence

Static: production changes preserve the `HillslopeRuntimeInputError` enum
definition and all variant fields.

Static: production changes preserve the original `self.code()` use inside
display formatting. Format strings and argument ordering were copied into
private family helpers without intentional text edits.

Ran: added characterization test:

```text
runtime_inputs::tests::hillslope_runtime_input_error_codes_and_display_are_stable
```

Ran: the characterization covers all 64 current
`HillslopeRuntimeInputError` variants with exact expected `HS-RUNTIME-E-*` code
strings and exact `Display` strings.

Ran: focused runtime-input tests after characterization and refactor passed:

```text
67 passed; 0 failed; 69 filtered out
```

Disposition: error-code and display-string surface is frozen and equivalent.
