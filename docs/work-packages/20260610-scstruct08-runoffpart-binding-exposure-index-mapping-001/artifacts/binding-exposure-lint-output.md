# SCSTRUCT08 Binding Exposure Lint Output

Evidence: Ran
Date: 2026-06-11

## Default Mode

Command:

```bash
python3 tools/check_sc_binding_exposure.py docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md
```

Output:

```text
PASS-DEFERRED docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md: 15 binding exposure row(s), 13 science-review-follow-on row(s) not yet consolidated
```

Exit: `0`

## Strict Mode

Command:

```bash
python3 tools/check_sc_binding_exposure.py --strict docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md
```

Output:

```text
PASS-DEFERRED docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md: 15 binding exposure row(s), 13 science-review-follow-on row(s) not yet consolidated
strict mode: deferred rows are not-consolidated; failing completion gate
exit=1
```

Exit: `1` by lint contract design for `PASS-DEFERRED` in strict mode. This is
not a malformed-row or gamed-gate failure; it confirms that SCSTRUCT09 remains
required before full consolidation can be claimed.
