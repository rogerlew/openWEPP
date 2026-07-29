use super::{
    ForestSurfaceLitterForcing, PlantForestCommunity, PlantForestDecomposition, PlantForestGrowth,
    PlantForestPhenology, RoutingCoefficientExtension,
};

/// Typed openWEPP-native forest management data.
#[derive(Debug, Clone, PartialEq)]
pub struct PlantForestData {
    /// Disturbed/forest class key joining management to its authoritative
    /// lookup row, disturbed binding, and soil policy.
    pub forest_class: String,
    pub growth: PlantForestGrowth,
    /// Present only for native YAML. Flat forest compatibility inputs cannot
    /// infer phenology authority and deliberately retain `None`.
    pub phenology: Option<PlantForestPhenology>,
    /// Authenticated exact-day external ground-deposition boundary.
    pub surface_litter_forcing: Option<Box<ForestSurfaceLitterForcing>>,
    /// Flat residue-cover equation coefficient (m^2/kg).
    pub cf: f64,
    /// Mean stem/branch diameter at maturity (m).
    pub diam: f64,
    pub decomposition: PlantForestDecomposition,
    pub community: PlantForestCommunity,
    pub routing: Option<RoutingCoefficientExtension>,
}
