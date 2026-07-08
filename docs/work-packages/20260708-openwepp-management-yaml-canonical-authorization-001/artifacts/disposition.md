# Disposition

Status: scaffold disposition complete.

The package scaffold accepts the YAML strategy:

- canonical management YAML is a first-class input surface, not a sidecar;
- flat `.man` remains source-only for native producer evolution;
- no native flat management writer is required;
- typed schema/parser/validator work is in scope;
- producers emit lowercase `.yaml` only, with `.man.yaml` as informal default
  naming for migrated flat `.man` inputs;
- consumers accept `.yaml`, `.YAML`, `.yml`, and `.YML` input paths but still
  validate schema content;
- a dedicated publishable `crates/openwepp-management-schema` crate is the
  recommended long-term schema owner;
- real runtime consumer-path proof is required before production-readiness
  closure.

No unresolved scaffold findings remain.
