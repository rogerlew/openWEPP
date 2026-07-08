# Worker Handoff

Status: complete.

## Next Package

Scaffold and execute the runtime/producer implementation package for rev-49
canonical native datver enforcement.

## First Actions

1. Update openWEPP default/active Lane D eligibility to inspect scheduled
   management datver authority, not only coefficient presence.
2. Add tests for:
   - all-native complete coefficients -> Lane D active/default;
   - all-legacy datvers -> legacy/off protected output path;
   - native missing coefficients -> fail closed;
   - mixed native/legacy datvers -> fail closed;
   - optional sidecar attempt -> unsupported/fail closed;
   - explicit disable on native-complete input -> protected legacy/off identity.
3. Update diagnostics to distinguish all-legacy fallback from malformed native
   production inputs.
4. Coordinate the wepppy producer change so Disturbed/native management files
   are emitted as `ow-lanuse-1` with embedded `routing_coefficients`.

## Constraints

- Do not project coefficients from legacy fields.
- Do not add optional sidecar coefficient authority.
- Do not delete legacy datver support.
- Preserve protected all-legacy/off output identity.
- Keep Lane D active/default production fail-closed on missing or mixed
  authority.
