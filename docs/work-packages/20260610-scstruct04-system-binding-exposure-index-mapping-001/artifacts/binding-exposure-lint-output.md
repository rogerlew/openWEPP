# Binding Exposure Lint Output

Evidence: Ran
Date: 2026-06-10

## Default Mode

Command:

```console
python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md
```

Output:

```console
PASS-DEFERRED docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md: 27 binding exposure row(s), 27 science-review-follow-on row(s) not yet consolidated
```

Exit code: `0`

## Strict Mode

Command:

```console
python3 tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md
```

Output:

```console
PASS-DEFERRED docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md: 27 binding exposure row(s), 27 science-review-follow-on row(s) not yet consolidated
strict mode: deferred rows are not-consolidated; failing completion gate
```

Exit code: `1`

Strict mode nonzero is expected for this package because SCSTRUCT04 routes every
unresolved row to SCSTRUCT05 and does not claim full consolidation.
