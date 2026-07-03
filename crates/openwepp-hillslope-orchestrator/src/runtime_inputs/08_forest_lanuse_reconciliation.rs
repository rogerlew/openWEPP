// Forest `lanuse` authority reconciliation manifest (DFF-WS1 Increment-2).
//
// Ties the openWEPP-native forest `.man` scenario to the paired disturbed
// `.sol` `DisturbedPolicy` under ADR-0034 / the management-`lanuse` authority
// contract (`LANUSE-AUTH-6` single-source-of-truth). A forest management must
// be backed by a disturbed soil policy whose `luse` matches the `.man` forest
// class; otherwise the run fails closed rather than silently pairing a forest
// management with a mismatched or non-disturbed soil.
//
// Scope (Increment-2): the load-bearing `.man` forest_class ↔ `.sol`
// `DisturbedPolicy` leg. The additional legs against the authoritative
// `(texture × class)` lookup table and the `openwepp-disturbed.json`
// class→management binding are a follow-on (`MAN-GAP-005`); soil texture
// (`stext`) stays soil-authoritative and is intentionally not carried in the
// `.man` (`LANUSE-AUTH-3`).

/// Reconciliation failure between a forest `.man` class and the paired soil
/// disturbed policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ForestLanuseReconciliationError {
    /// A forest management scenario is present but the paired soil profile
    /// carries no `DisturbedPolicy` to authorize the forest physics.
    MissingDisturbedPolicy { forest_class: String },
    /// The `.man` forest class does not match any `.sol` disturbed-policy `luse`.
    ClassMismatch {
        forest_class: String,
        policy_luse: String,
    },
}

impl ForestLanuseReconciliationError {
    /// Stable diagnostic code for the reconciliation failure class.
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::MissingDisturbedPolicy { .. } => "LANUSE-RECON-E-001",
            Self::ClassMismatch { .. } => "LANUSE-RECON-E-002",
        }
    }
}

impl fmt::Display for ForestLanuseReconciliationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::MissingDisturbedPolicy { forest_class } => write!(
                f,
                "{}: forest management class '{forest_class}' has no paired soil DisturbedPolicy",
                self.code()
            ),
            Self::ClassMismatch {
                forest_class,
                policy_luse,
            } => write!(
                f,
                "{}: forest management class '{forest_class}' does not match soil DisturbedPolicy luse '{policy_luse}'",
                self.code()
            ),
        }
    }
}

impl Error for ForestLanuseReconciliationError {}

/// Reconcile the forest `lanuse` authority for a hillslope: every forest
/// management class must be backed by a disturbed soil policy whose `luse`
/// matches it. Cropland-only managements reconcile trivially (`Ok`).
///
/// # Errors
///
/// Returns [`ForestLanuseReconciliationError`] when a forest management is not
/// backed by a matching disturbed soil policy (fail-closed, `LANUSE-AUTH-6`).
pub fn reconcile_forest_lanuse_authority(
    management: &ManagementParseOutput,
    soil: &SoilProfile,
) -> Result<(), ForestLanuseReconciliationError> {
    let forest_classes: Vec<&str> = management
        .registries
        .plants
        .iter()
        .filter_map(|plant| match &plant.data {
            PlantScenarioData::Forest(forest) => Some(forest.forest_class.as_str()),
            PlantScenarioData::Cropland(_) => None,
        })
        .collect();
    if forest_classes.is_empty() {
        return Ok(());
    }

    let policy_luses: Vec<&str> = soil
        .ofes
        .iter()
        .filter_map(|ofe| ofe.policy.as_ref().map(disturbed_policy_luse))
        .collect();

    for forest_class in forest_classes {
        if policy_luses.is_empty() {
            return Err(ForestLanuseReconciliationError::MissingDisturbedPolicy {
                forest_class: forest_class.to_string(),
            });
        }
        let normalized_class = normalize_lanuse_class_key(forest_class);
        let matched = policy_luses
            .iter()
            .any(|luse| normalize_lanuse_class_key(luse) == normalized_class);
        if !matched {
            return Err(ForestLanuseReconciliationError::ClassMismatch {
                forest_class: forest_class.to_string(),
                policy_luse: policy_luses[0].to_string(),
            });
        }
    }

    Ok(())
}

/// The `luse` (class) token carried by a disturbed soil policy, independent of
/// policy version.
fn disturbed_policy_luse(policy: &DisturbedPolicy) -> &str {
    match policy {
        DisturbedPolicy::V9002 { luse, .. }
        | DisturbedPolicy::V9003 { luse, .. }
        | DisturbedPolicy::V9005 { luse, .. } => luse.as_str(),
    }
}

/// Normalize a disturbed/forest class key for comparison across the `.man`
/// (underscore form, e.g. `forest_high_sev_fire`) and the `.sol` / lookup
/// (space form, e.g. `forest high sev fire`): lowercase, treat `_` as a space,
/// and collapse whitespace.
fn normalize_lanuse_class_key(raw: &str) -> String {
    raw.to_ascii_lowercase()
        .replace('_', " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

#[cfg(test)]
mod forest_lanuse_reconciliation_tests {
    use super::*;
    use openwepp_input_contract::parsers::management::{
        ParseMode, parse_management_from_str,
    };
    use openwepp_input_contract::parsers::soil::{SoilParserOptions, parse_soil};

    const FOREST_MAN: &str = "ow-lanuse-1
1
1
1
Forest_High_Severity_Fire
d1
d2
d3
3 # Landuse - <Forest>
forest_high_sev_fire
14.0 3.0 0.0 2.0 0.45
17.0 0.2 0.42 0.0 0.5
20.0 0.1 90.0 0.33 0.2
2.0 0.3 1.0 1.0
5.0 0.005
0.0 0.0
-5.0 5.0 0.2 0.1
0.0 0.0 0.0 0.0
0.0 0.0 0.0 0.0
0.02 2.0 8.0 500.0
0
1
Forest_Initial
d1
d2
d3
3 # Landuse - <Forest>
0.4 0.3 0.3 0.06
1
2
0.0 0.0
0.2 0.2
0
0
0
1
Forest_Year
d1
d2
d3
3 # Landuse - <Forest>
1
0
0
0
2
0
0
0
0.0
3
Forest_Management
d1
d2
d3
1
1
1
1
1
1
";

    fn disturbed_soil(luse: &str) -> String {
        format!(
            "9002
Disturbed soil profile
1 1
1 {luse} silt_loam 0.20 0.001
SOIL_B CLAY_LOAM 2 0.20 0.55 900000 0.005 4.2 10.5
100 1.25 15.0 1.20 0.30 0.15 35 25 2.0 15 5 0.05 0.45 0.02 1.40 120 0.16 0.31
250 1.30 8.0 1.10 0.28 0.14 33 27 1.8 14 7 0.06 0.43 0.03 1.35 110 0.15 0.30
1 500 0.8
"
        )
    }

    fn parsed_management(
        text: &str,
    ) -> openwepp_input_contract::parsers::management::ManagementParseOutput {
        parse_management_from_str(text, ParseMode::Strict).expect("management parses")
    }

    fn parsed_soil(text: &str) -> SoilProfile {
        parse_soil(text, SoilParserOptions::default()).expect("soil parses")
    }

    #[test]
    fn matching_forest_class_and_policy_luse_reconciles() {
        let management = parsed_management(FOREST_MAN);
        // `.sol` policy luse in space form matches the `.man` underscore form.
        let soil = parsed_soil(&disturbed_soil("'forest high sev fire'"));
        reconcile_forest_lanuse_authority(&management, &soil)
            .expect("matching forest class and policy luse should reconcile");
    }

    #[test]
    fn mismatched_policy_luse_fails_closed() {
        let management = parsed_management(FOREST_MAN);
        let soil = parsed_soil(&disturbed_soil("'young forest'"));
        let error = reconcile_forest_lanuse_authority(&management, &soil)
            .expect_err("mismatched policy luse must fail closed");
        assert_eq!(error.code(), "LANUSE-RECON-E-002");
    }

    #[test]
    fn missing_disturbed_policy_fails_closed() {
        let management = parsed_management(FOREST_MAN);
        // A plain 97.5 soil profile carries no DisturbedPolicy.
        let plain_soil = "97.5
Plain soil profile
1 1
SOIL_A SILT_LOAM 2 0.23 0.60 1200000 0.004 3.5 12.0
150 40 20 2.5 12 5
300 38 22 2.2 10 8
";
        let soil = parsed_soil(plain_soil);
        let error = reconcile_forest_lanuse_authority(&management, &soil)
            .expect_err("forest management without a disturbed soil policy must fail closed");
        assert_eq!(error.code(), "LANUSE-RECON-E-001");
    }

    #[test]
    fn cropland_only_management_reconciles_trivially() {
        let cropland = parsed_management(
            "98.4
1
1
1
Crop
d1
d2
d3
1 # Landuse - <Cropland>
WeppWillSet
14.0 3.0 0.0 2.0 5.0 5.0 0.0 0.3 1.0 0.005
0.5 1.0 0.45 0.99 17.0 0.0 0.42 0.2
2
0.0 0.0 20.0 0.1 0.5 0.3 0.33 0.2 90 40.0
-40.0 2.0 0.0
0
1
Ini
d1
d2
d3
1 # Landuse - <Cropland>
1.1 0.4 330 1000 0.0 0.3
1
2
400.0 0.06 0.3 0.06 0.0
1
0.0 0.0 0.0 0.0 0.0
0.2 0.2
0
0
0
1
Year
d1
d2
d3
1 # Landuse - <Cropland>
1
0
0
0
2
0
0
0
0.0
3
Manage
d1
d2
d3
1
1
1
1
1
1
",
        );
        let soil = parsed_soil(&disturbed_soil("'forest high sev fire'"));
        reconcile_forest_lanuse_authority(&cropland, &soil)
            .expect("cropland-only management reconciles trivially");
    }
}
