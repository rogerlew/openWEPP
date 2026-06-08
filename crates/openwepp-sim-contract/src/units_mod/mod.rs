mod boundary_catalog;
mod output_catalog;
mod registries;
mod types;

pub use boundary_catalog::{canonical_boundary_unit_entries, hphys0274_required_boundary_aliases};
pub use output_catalog::canonical_output_unit_entries;
pub use registries::{
    BoundaryUnitRegistry, BoundaryUnitRegistryError, OutputUnitRegistry, OutputUnitRegistryError,
    validate_output_schema_unit,
};
pub use types::{
    BoundaryUnitEntry, DimensionClass, DomainClass, OutputUnitAuthority, OutputUnitEntry,
    TypedBoundaryRequirement,
};
