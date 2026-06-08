/// Boundary-symbol physical dimension class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum DimensionClass {
    Area,
    Count,
    Density,
    Depth,
    Direction,
    Fraction,
    HydraulicConductivity,
    RadiationDaily,
    RadiationHourly,
    Rate,
    Temperature,
    Time,
    Unitless,
    VolumetricWaterContent,
    WindSpeed,
}

impl DimensionClass {
    #[must_use]
    pub const fn is_dimensionless(self) -> bool {
        matches!(self, Self::Count | Self::Fraction | Self::Unitless)
    }
}

/// Boundary-symbol domain class.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum DomainClass {
    AnyFinite,
    CountNonNegative,
    DirectionDegrees,
    NonNegativeFinite,
    PositiveFinite,
    SignedFinite,
    UnitInterval,
}

/// Required typed-boundary posture for a symbol.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum TypedBoundaryRequirement {
    TypedAvailable,
    TypedRequired,
    ScalarException,
    FollowUpRequired,
}

/// Canonical unit metadata for one runtime/publication boundary symbol family.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct BoundaryUnitEntry {
    canonical_symbol: &'static str,
    boundary_aliases: &'static [&'static str],
    unit_label: &'static str,
    dimension_class: DimensionClass,
    domain_class: DomainClass,
    producer_scope: &'static str,
    consumer_scope: &'static str,
    contract_id: &'static str,
    invariant_id: &'static str,
    typed_boundary: TypedBoundaryRequirement,
    scalar_exception: Option<&'static str>,
    publication_aliases: &'static [&'static str],
}

impl BoundaryUnitEntry {
    #[allow(clippy::too_many_arguments)]
    #[must_use]
    pub const fn new(
        canonical_symbol: &'static str,
        boundary_aliases: &'static [&'static str],
        unit_label: &'static str,
        dimension_class: DimensionClass,
        domain_class: DomainClass,
        producer_scope: &'static str,
        consumer_scope: &'static str,
        contract_id: &'static str,
        invariant_id: &'static str,
        typed_boundary: TypedBoundaryRequirement,
        scalar_exception: Option<&'static str>,
        publication_aliases: &'static [&'static str],
    ) -> Self {
        Self {
            canonical_symbol,
            boundary_aliases,
            unit_label,
            dimension_class,
            domain_class,
            producer_scope,
            consumer_scope,
            contract_id,
            invariant_id,
            typed_boundary,
            scalar_exception,
            publication_aliases,
        }
    }

    #[must_use]
    pub const fn canonical_symbol(&self) -> &'static str {
        self.canonical_symbol
    }

    #[must_use]
    pub const fn boundary_aliases(&self) -> &'static [&'static str] {
        self.boundary_aliases
    }

    #[must_use]
    pub const fn unit_label(&self) -> &'static str {
        self.unit_label
    }

    #[must_use]
    pub const fn dimension_class(&self) -> DimensionClass {
        self.dimension_class
    }

    #[must_use]
    pub const fn domain_class(&self) -> DomainClass {
        self.domain_class
    }

    #[must_use]
    pub const fn producer_scope(&self) -> &'static str {
        self.producer_scope
    }

    #[must_use]
    pub const fn consumer_scope(&self) -> &'static str {
        self.consumer_scope
    }

    #[must_use]
    pub const fn contract_id(&self) -> &'static str {
        self.contract_id
    }

    #[must_use]
    pub const fn invariant_id(&self) -> &'static str {
        self.invariant_id
    }

    #[must_use]
    pub const fn typed_boundary(&self) -> TypedBoundaryRequirement {
        self.typed_boundary
    }

    #[must_use]
    pub const fn scalar_exception(&self) -> Option<&'static str> {
        self.scalar_exception
    }

    #[must_use]
    pub const fn publication_aliases(&self) -> &'static [&'static str] {
        self.publication_aliases
    }
}

/// Authority class for output publication unit metadata.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum OutputUnitAuthority {
    BoundaryRegistry {
        boundary_alias: &'static str,
    },
    PublicationOnly {
        rationale: &'static str,
        contract_id: &'static str,
        invariant_id: &'static str,
    },
}

/// Canonical unit metadata for one output schema column.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub struct OutputUnitEntry {
    schema_id: &'static str,
    column_name: &'static str,
    unit_label: &'static str,
    authority: OutputUnitAuthority,
}

impl OutputUnitEntry {
    #[must_use]
    pub const fn boundary_registry(
        schema_id: &'static str,
        column_name: &'static str,
        unit_label: &'static str,
        boundary_alias: &'static str,
    ) -> Self {
        Self {
            schema_id,
            column_name,
            unit_label,
            authority: OutputUnitAuthority::BoundaryRegistry { boundary_alias },
        }
    }

    #[must_use]
    pub const fn publication_only(
        schema_id: &'static str,
        column_name: &'static str,
        unit_label: &'static str,
        rationale: &'static str,
        contract_id: &'static str,
        invariant_id: &'static str,
    ) -> Self {
        Self {
            schema_id,
            column_name,
            unit_label,
            authority: OutputUnitAuthority::PublicationOnly {
                rationale,
                contract_id,
                invariant_id,
            },
        }
    }

    #[must_use]
    pub const fn schema_id(&self) -> &'static str {
        self.schema_id
    }

    #[must_use]
    pub const fn column_name(&self) -> &'static str {
        self.column_name
    }

    #[must_use]
    pub const fn unit_label(&self) -> &'static str {
        self.unit_label
    }

    #[must_use]
    pub const fn authority(&self) -> OutputUnitAuthority {
        self.authority
    }
}
