/// Compatibility behavior policy for legacy adapter boundaries.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum CompatibilityPolicy {
    /// Strict mode rejects legacy aliases and unknown edge artifacts.
    #[default]
    Strict,
    /// Compat mode accepts selected legacy forms and emits typed warnings.
    Compat,
}

impl CompatibilityPolicy {
    #[must_use]
    pub const fn allows_legacy(self) -> bool {
        matches!(self, Self::Compat)
    }
}
