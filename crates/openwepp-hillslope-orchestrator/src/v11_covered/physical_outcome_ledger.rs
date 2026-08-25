//! Independent post-candidate Stage-3 snow mass and energy accounting.
//!
//! No API in this module converts ledger fields into solver operands.

use super::{OfeId, Stage3LaneAreaBasisV1};
use openwepp_coupled_time::{Digest32, TimeSupport, digest_bytes};

/// Event-integrated lower-boundary custody for a lane whose accepted ending
/// snow owner is dormant.  Unlike the persistent receipt, this wire has no
/// ending snow temperature: dormancy is proved by the ending owner identity.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct TerminalSnowSoilHeatReceiptV1 {
    pub support: TimeSupport,
    pub lane_id: u32,
    pub ofe_id: OfeId,
    pub beginning_snow_owner_sha256: Digest32,
    pub ending_dormant_snow_owner_sha256: Digest32,
    pub ending_soil_owner_sha256: Digest32,
    pub limiting_boundary_receipt_sha256: Digest32,
    pub snow_heat_j_m2: f64,
    pub soil_heat_j_m2: f64,
    pub receipt_sha256: Digest32,
}

impl TerminalSnowSoilHeatReceiptV1 {
    pub(crate) fn seal(mut self) -> Result<Self, Stage3PhysicalOutcomeLedgerError> {
        self.receipt_sha256 = Digest32::zero();
        if self.beginning_snow_owner_sha256 == Digest32::zero()
            || self.ending_dormant_snow_owner_sha256 == Digest32::zero()
            || self.ending_soil_owner_sha256 == Digest32::zero()
            || self.limiting_boundary_receipt_sha256 == Digest32::zero()
            || !self.snow_heat_j_m2.is_finite()
            || self.soil_heat_j_m2.to_bits() != (-self.snow_heat_j_m2).to_bits()
        {
            return Err(Stage3PhysicalOutcomeLedgerError::Identity(
                "terminal snow-soil custody",
            ));
        }
        self.receipt_sha256 = self.digest();
        Ok(self)
    }

    pub(crate) fn validate(&self) -> Result<(), Stage3PhysicalOutcomeLedgerError> {
        let mut unsealed = self.clone();
        unsealed.receipt_sha256 = Digest32::zero();
        let expected = unsealed.seal()?;
        if &expected != self {
            return Err(Stage3PhysicalOutcomeLedgerError::Identity(
                "terminal snow-soil receipt seal",
            ));
        }
        Ok(())
    }

    fn digest(&self) -> Digest32 {
        let mut bytes = b"OPENWEPP_TERMINAL_SNOW_SOIL_HEAT_RECEIPT_V1".to_vec();
        bytes.extend_from_slice(&self.support.start_ns().get().to_le_bytes());
        bytes.extend_from_slice(&self.support.end_ns().get().to_le_bytes());
        bytes.extend_from_slice(&self.lane_id.to_le_bytes());
        append_str(&mut bytes, self.ofe_id.as_str());
        bytes.extend_from_slice(self.beginning_snow_owner_sha256.as_bytes());
        bytes.extend_from_slice(self.ending_dormant_snow_owner_sha256.as_bytes());
        bytes.extend_from_slice(self.ending_soil_owner_sha256.as_bytes());
        bytes.extend_from_slice(self.limiting_boundary_receipt_sha256.as_bytes());
        bytes.extend_from_slice(&self.snow_heat_j_m2.to_bits().to_le_bytes());
        bytes.extend_from_slice(&self.soil_heat_j_m2.to_bits().to_le_bytes());
        digest_bytes(&bytes)
    }
}

pub(crate) fn ledger_set_digest(
    ledgers: &std::collections::BTreeMap<u32, Stage3LanePhysicalOutcomeLedgerV1>,
) -> Digest32 {
    let mut bytes = b"OPENWEPP_STAGE3_PHYSICAL_OUTCOME_LEDGER_SET_V1".to_vec();
    for (lane, ledger) in ledgers {
        bytes.extend_from_slice(&lane.to_be_bytes());
        bytes.extend_from_slice(ledger.receipt_sha256.as_bytes());
    }
    digest_bytes(&bytes)
}

const MASS_TOL: f64 = 1.0e-9;
const ENERGY_TOL: f64 = 1.0e-6;
const LATENT_HEAT_FUSION_J_KG: f64 = 333_600.0;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Stage3LanePhysicalOutcomeExpectationV1 {
    pub support: TimeSupport,
    pub lane_id: u32,
    pub ofe_id: OfeId,
    pub topology_sha256: Digest32,
    pub beginning_snow_owner_sha256: Digest32,
    pub ending_snow_owner_sha256: Digest32,
    pub precipitation_set_sha256: Digest32,
    /// SW, LW, sensible, latent/vapor, snow-soil, interlayer, in that order.
    pub source_receipts_sha256: [Digest32; 6],
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Stage3LanePhysicalOutcomeLedgerV1 {
    pub support: TimeSupport,
    pub lane_id: u32,
    pub ofe_id: OfeId,
    pub area_basis: Stage3LaneAreaBasisV1,
    pub topology_sha256: Digest32,
    pub beginning_snow_owner_sha256: Digest32,
    pub ending_snow_owner_sha256: Digest32,
    pub precipitation_set_sha256: Digest32,
    pub source_receipts_sha256: [Digest32; 6],
    pub beginning_ice_kg_m2: f64,
    pub beginning_liquid_kg_m2: f64,
    pub beginning_cold_content_j_m2: f64,
    pub beginning_enthalpy_j_m2: f64,
    pub ending_ice_kg_m2: f64,
    pub ending_liquid_kg_m2: f64,
    pub ending_cold_content_j_m2: f64,
    pub ending_enthalpy_j_m2: f64,
    pub solid_precipitation_kg_m2: f64,
    pub liquid_precipitation_kg_m2: f64,
    pub precipitation_advection_j_m2: f64,
    pub deposition_kg_m2: f64,
    pub sublimation_kg_m2: f64,
    /// Positive into snow, reconstructed independently from deposition/sublimation.
    pub vapor_transfer_kg_m2: f64,
    pub latent_heat_j_kg: f64,
    pub snow_surface_temperature_k: f64,
    pub vapor_material_enthalpy_j_m2: f64,
    pub melt_kg_m2: f64,
    pub refreeze_kg_m2: f64,
    pub terminal_liquid_kg_m2: f64,
    pub retained_liquid_kg_m2: f64,
    /// All energy terms use positive-into-snow sign convention.
    pub shortwave_j_m2: f64,
    pub longwave_j_m2: f64,
    pub sensible_j_m2: f64,
    pub latent_j_m2: f64,
    pub soil_heat_j_m2: f64,
    pub interlayer_active_conduction_j_m2: f64,
    pub interlayer_lower_conduction_j_m2: f64,
    pub interlayer_conduction_j_m2: f64,
    pub refreeze_fusion_j_m2: f64,
    pub mass_residual_kg_m2: f64,
    pub ice_residual_kg_m2: f64,
    pub liquid_residual_kg_m2: f64,
    pub vapor_residual_kg_m2: f64,
    pub energy_residual_j_m2: f64,
    pub ending_liquid_residual_kg_m2: f64,
    pub receipt_sha256: Digest32,
}

#[derive(Clone, Debug, Eq, PartialEq, thiserror::Error)]
pub(crate) enum Stage3PhysicalOutcomeLedgerError {
    #[error("physical outcome identity mismatch: {0}")]
    Identity(&'static str),
    #[error("physical outcome numeric domain failure: {0}")]
    Numeric(&'static str),
    #[error("physical outcome closure failure: {0}")]
    Closure(&'static str),
}

impl Stage3LanePhysicalOutcomeLedgerV1 {
    pub(crate) fn try_new(
        mut value: Self,
        expected: &Stage3LanePhysicalOutcomeExpectationV1,
    ) -> Result<Self, Stage3PhysicalOutcomeLedgerError> {
        value.receipt_sha256 = Digest32::zero();
        let r = value.reconstruct();
        value.mass_residual_kg_m2 = r[0];
        value.ice_residual_kg_m2 = r[1];
        value.liquid_residual_kg_m2 = r[2];
        value.vapor_residual_kg_m2 = r[3];
        value.energy_residual_j_m2 = r[4];
        value.ending_liquid_residual_kg_m2 = r[5];
        value.receipt_sha256 = value.digest();
        value.validate(expected)?;
        Ok(value)
    }

    pub(crate) fn validate(
        &self,
        expected: &Stage3LanePhysicalOutcomeExpectationV1,
    ) -> Result<(), Stage3PhysicalOutcomeLedgerError> {
        self.validate_bindings(expected)?;
        self.validate_closure()
    }

    fn validate_bindings(
        &self,
        expected: &Stage3LanePhysicalOutcomeExpectationV1,
    ) -> Result<(), Stage3PhysicalOutcomeLedgerError> {
        if self.support != expected.support
            || self.lane_id != expected.lane_id
            || self.ofe_id != expected.ofe_id
            || self.area_basis != Stage3LaneAreaBasisV1::OfeGround
            || self.topology_sha256 != expected.topology_sha256
            || self.beginning_snow_owner_sha256 != expected.beginning_snow_owner_sha256
            || self.ending_snow_owner_sha256 != expected.ending_snow_owner_sha256
            || self.precipitation_set_sha256 != expected.precipitation_set_sha256
            || self.source_receipts_sha256 != expected.source_receipts_sha256
        {
            return Err(Stage3PhysicalOutcomeLedgerError::Identity(
                "expected binding",
            ));
        }
        if self.receipt_sha256 == Digest32::zero() || self.receipt_sha256 != self.digest() {
            return Err(Stage3PhysicalOutcomeLedgerError::Identity("seal"));
        }
        let mut receipts = self.source_receipts_sha256;
        receipts.sort_unstable();
        if receipts.iter().any(|v| *v == Digest32::zero())
            || receipts.windows(2).any(|v| v[0] == v[1])
        {
            return Err(Stage3PhysicalOutcomeLedgerError::Identity(
                "complete unique receipt set",
            ));
        }
        Ok(())
    }

    fn validate_closure(&self) -> Result<(), Stage3PhysicalOutcomeLedgerError> {
        let nonnegative = [
            self.beginning_ice_kg_m2,
            self.beginning_liquid_kg_m2,
            self.beginning_cold_content_j_m2,
            self.ending_ice_kg_m2,
            self.ending_liquid_kg_m2,
            self.ending_cold_content_j_m2,
            self.solid_precipitation_kg_m2,
            self.liquid_precipitation_kg_m2,
            self.deposition_kg_m2,
            self.sublimation_kg_m2,
            self.latent_heat_j_kg,
            self.snow_surface_temperature_k,
            self.melt_kg_m2,
            self.refreeze_kg_m2,
            self.terminal_liquid_kg_m2,
            self.retained_liquid_kg_m2,
        ];
        if nonnegative.iter().any(|v| !v.is_finite() || *v < 0.0) {
            return Err(Stage3PhysicalOutcomeLedgerError::Numeric(
                "nonnegative operand",
            ));
        }
        if self.signed().iter().any(|v| !v.is_finite()) {
            return Err(Stage3PhysicalOutcomeLedgerError::Numeric("signed operand"));
        }
        let vapor = self.deposition_kg_m2 - self.sublimation_kg_m2;
        if !close(vapor, self.vapor_transfer_kg_m2, MASS_TOL)
            || !close(self.latent_j_m2, vapor * self.latent_heat_j_kg, ENERGY_TOL)
        {
            return Err(Stage3PhysicalOutcomeLedgerError::Closure(
                "vapor/latent join",
            ));
        }
        // Refreeze/fusion is an internal exchange between represented ice,
        // liquid, and cold content. The owner-derived material enthalpy
        // already includes it, so its whole-column external contribution must
        // cancel exactly rather than being counted a second time.
        if self.refreeze_fusion_j_m2.to_bits() != 0.0_f64.to_bits() {
            return Err(Stage3PhysicalOutcomeLedgerError::Closure(
                "refreeze/fusion internal cancellation",
            ));
        }
        if !close(
            self.interlayer_active_conduction_j_m2 + self.interlayer_lower_conduction_j_m2,
            self.interlayer_conduction_j_m2,
            ENERGY_TOL,
        ) || self.interlayer_conduction_j_m2.abs() > ENERGY_TOL
        {
            return Err(Stage3PhysicalOutcomeLedgerError::Closure(
                "interlayer equal-and-opposite cancellation",
            ));
        }
        let reconstructed = self.reconstruct();
        let supplied = [
            self.mass_residual_kg_m2,
            self.ice_residual_kg_m2,
            self.liquid_residual_kg_m2,
            self.vapor_residual_kg_m2,
            self.energy_residual_j_m2,
            self.ending_liquid_residual_kg_m2,
        ];
        if reconstructed
            .iter()
            .zip(supplied)
            .any(|(a, b)| a.to_bits() != b.to_bits())
        {
            return Err(Stage3PhysicalOutcomeLedgerError::Identity(
                "reconstructed residual",
            ));
        }
        if supplied[..4].iter().any(|v| v.abs() > MASS_TOL)
            || supplied[4].abs() > ENERGY_TOL
            || supplied[5].abs() > MASS_TOL
        {
            return Err(Stage3PhysicalOutcomeLedgerError::Closure(
                "mass/energy/ending",
            ));
        }
        Ok(())
    }

    fn reconstruct(&self) -> [f64; 6] {
        let vapor = self.deposition_kg_m2 - self.sublimation_kg_m2;
        let ice = self.ending_ice_kg_m2
            - (self.beginning_ice_kg_m2 + self.solid_precipitation_kg_m2 + vapor - self.melt_kg_m2
                + self.refreeze_kg_m2);
        let liquid = self.ending_liquid_kg_m2
            - (self.beginning_liquid_kg_m2 + self.liquid_precipitation_kg_m2 + self.melt_kg_m2
                - self.refreeze_kg_m2
                - self.terminal_liquid_kg_m2);
        let mass = self.ending_ice_kg_m2 + self.ending_liquid_kg_m2
            - (self.beginning_ice_kg_m2
                + self.beginning_liquid_kg_m2
                + self.solid_precipitation_kg_m2
                + self.liquid_precipitation_kg_m2
                + vapor
                - self.terminal_liquid_kg_m2);
        let energy = self.ending_enthalpy_j_m2
            - (self.beginning_enthalpy_j_m2
                + self.shortwave_j_m2
                + self.longwave_j_m2
                + self.sensible_j_m2
                + self.latent_j_m2
                + self.vapor_material_enthalpy_j_m2
                + self.precipitation_advection_j_m2
                + self.soil_heat_j_m2
                + self.interlayer_conduction_j_m2
                + self.refreeze_fusion_j_m2
                // Material enthalpy uses ice at the fusion reference and
                // liquid at +L_f. These boundary phase transports are
                // independent of precipitation sensible advection.
                + LATENT_HEAT_FUSION_J_KG
                    * (self.liquid_precipitation_kg_m2 - self.terminal_liquid_kg_m2));
        [
            mass,
            ice,
            liquid,
            vapor - self.vapor_transfer_kg_m2,
            energy,
            self.ending_liquid_kg_m2 - self.retained_liquid_kg_m2,
        ]
    }

    fn signed(&self) -> [f64; 20] {
        [
            self.beginning_enthalpy_j_m2,
            self.ending_enthalpy_j_m2,
            self.precipitation_advection_j_m2,
            self.vapor_transfer_kg_m2,
            self.shortwave_j_m2,
            self.longwave_j_m2,
            self.sensible_j_m2,
            self.latent_j_m2,
            self.soil_heat_j_m2,
            self.interlayer_active_conduction_j_m2,
            self.interlayer_lower_conduction_j_m2,
            self.interlayer_conduction_j_m2,
            self.refreeze_fusion_j_m2,
            self.vapor_material_enthalpy_j_m2,
            self.mass_residual_kg_m2,
            self.ice_residual_kg_m2,
            self.liquid_residual_kg_m2,
            self.vapor_residual_kg_m2,
            self.energy_residual_j_m2,
            self.ending_liquid_residual_kg_m2,
        ]
    }

    fn digest(&self) -> Digest32 {
        let mut b = b"OPENWEPP_STAGE3_PHYSICAL_OUTCOME_LEDGER_V1".to_vec();
        b.extend_from_slice(&self.support.start_ns().get().to_le_bytes());
        b.extend_from_slice(&self.support.end_ns().get().to_le_bytes());
        b.extend_from_slice(&self.lane_id.to_le_bytes());
        append_str(&mut b, self.ofe_id.as_str());
        b.push(0);
        for d in [
            self.topology_sha256,
            self.beginning_snow_owner_sha256,
            self.ending_snow_owner_sha256,
            self.precipitation_set_sha256,
        ]
        .iter()
        .chain(self.source_receipts_sha256.iter())
        {
            b.extend_from_slice(d.as_bytes());
        }
        for v in self.values() {
            b.extend_from_slice(&v.to_bits().to_le_bytes());
        }
        digest_bytes(&b)
    }

    fn values(&self) -> [f64; 36] {
        [
            self.beginning_ice_kg_m2,
            self.beginning_liquid_kg_m2,
            self.beginning_cold_content_j_m2,
            self.beginning_enthalpy_j_m2,
            self.ending_ice_kg_m2,
            self.ending_liquid_kg_m2,
            self.ending_cold_content_j_m2,
            self.ending_enthalpy_j_m2,
            self.solid_precipitation_kg_m2,
            self.liquid_precipitation_kg_m2,
            self.precipitation_advection_j_m2,
            self.deposition_kg_m2,
            self.sublimation_kg_m2,
            self.vapor_transfer_kg_m2,
            self.latent_heat_j_kg,
            self.snow_surface_temperature_k,
            self.vapor_material_enthalpy_j_m2,
            self.melt_kg_m2,
            self.refreeze_kg_m2,
            self.terminal_liquid_kg_m2,
            self.retained_liquid_kg_m2,
            self.shortwave_j_m2,
            self.longwave_j_m2,
            self.sensible_j_m2,
            self.latent_j_m2,
            self.soil_heat_j_m2,
            self.interlayer_active_conduction_j_m2,
            self.interlayer_lower_conduction_j_m2,
            self.interlayer_conduction_j_m2,
            self.refreeze_fusion_j_m2,
            self.mass_residual_kg_m2,
            self.ice_residual_kg_m2,
            self.liquid_residual_kg_m2,
            self.vapor_residual_kg_m2,
            self.energy_residual_j_m2,
            self.ending_liquid_residual_kg_m2,
        ]
    }
}

fn close(a: f64, b: f64, tolerance: f64) -> bool {
    a.is_finite() && b.is_finite() && (a - b).abs() <= tolerance
}
fn append_str(bytes: &mut Vec<u8>, value: &str) {
    bytes.extend_from_slice(&(value.len() as u64).to_le_bytes());
    bytes.extend_from_slice(value.as_bytes());
}

#[cfg(test)]
mod tests {
    use super::*;
    use openwepp_coupled_time::ModelTimeNs;

    fn d(v: u8) -> Digest32 {
        Digest32::from_bytes([v; 32])
    }
    fn fixture() -> (
        Stage3LanePhysicalOutcomeLedgerV1,
        Stage3LanePhysicalOutcomeExpectationV1,
    ) {
        let support = TimeSupport::new(ModelTimeNs::new(0), ModelTimeNs::new(1_800_000_000_000))
            .expect("support");
        let expected = Stage3LanePhysicalOutcomeExpectationV1 {
            support,
            lane_id: 2,
            ofe_id: OfeId::try_new("ofe-2").expect("OFE"),
            topology_sha256: d(1),
            beginning_snow_owner_sha256: d(2),
            ending_snow_owner_sha256: d(3),
            precipitation_set_sha256: d(4),
            source_receipts_sha256: [d(5), d(6), d(7), d(8), d(9), d(10)],
        };
        let value = Stage3LanePhysicalOutcomeLedgerV1 {
            support,
            lane_id: 2,
            ofe_id: OfeId::try_new("ofe-2").expect("OFE"),
            area_basis: Stage3LaneAreaBasisV1::OfeGround,
            topology_sha256: d(1),
            beginning_snow_owner_sha256: d(2),
            ending_snow_owner_sha256: d(3),
            precipitation_set_sha256: d(4),
            source_receipts_sha256: expected.source_receipts_sha256,
            beginning_ice_kg_m2: 10.0,
            beginning_liquid_kg_m2: 1.0,
            beginning_cold_content_j_m2: 100.0,
            beginning_enthalpy_j_m2: -100.0,
            ending_ice_kg_m2: 10.6,
            ending_liquid_kg_m2: 1.05,
            ending_cold_content_j_m2: 75.0,
            ending_enthalpy_j_m2: 16_606.0,
            solid_precipitation_kg_m2: 0.5,
            liquid_precipitation_kg_m2: 0.2,
            precipitation_advection_j_m2: 2.0,
            deposition_kg_m2: 0.2,
            sublimation_kg_m2: 0.1,
            vapor_transfer_kg_m2: 0.1,
            latent_heat_j_kg: 100.0,
            snow_surface_temperature_k: 273.15,
            vapor_material_enthalpy_j_m2: 0.0,
            melt_kg_m2: 0.1,
            refreeze_kg_m2: 0.1,
            terminal_liquid_kg_m2: 0.15,
            retained_liquid_kg_m2: 1.05,
            shortwave_j_m2: 20.0,
            longwave_j_m2: -5.0,
            sensible_j_m2: -4.0,
            latent_j_m2: 10.0,
            soil_heat_j_m2: 3.0,
            interlayer_active_conduction_j_m2: 2.0,
            interlayer_lower_conduction_j_m2: -2.0,
            interlayer_conduction_j_m2: 0.0,
            refreeze_fusion_j_m2: 0.0,
            mass_residual_kg_m2: 0.0,
            ice_residual_kg_m2: 0.0,
            liquid_residual_kg_m2: 0.0,
            vapor_residual_kg_m2: 0.0,
            energy_residual_j_m2: 0.0,
            ending_liquid_residual_kg_m2: 0.0,
            receipt_sha256: Digest32::zero(),
        };
        (value, expected)
    }

    #[test]
    fn independent_ledger_closes() {
        let (value, expected) = fixture();
        Stage3LanePhysicalOutcomeLedgerV1::try_new(value, &expected).expect("closed ledger");
    }

    #[test]
    fn terminal_snow_soil_receipt_has_no_endpoint_temperature_and_closes_custody() {
        let support =
            TimeSupport::new(ModelTimeNs::new(10), ModelTimeNs::new(20)).expect("support");
        let receipt = TerminalSnowSoilHeatReceiptV1 {
            support,
            lane_id: 7,
            ofe_id: OfeId::try_new("ofe-7").expect("ofe"),
            beginning_snow_owner_sha256: d(1),
            ending_dormant_snow_owner_sha256: d(2),
            ending_soil_owner_sha256: d(3),
            limiting_boundary_receipt_sha256: d(4),
            snow_heat_j_m2: -12.5,
            soil_heat_j_m2: 12.5,
            receipt_sha256: Digest32::zero(),
        }
        .seal()
        .expect("terminal receipt");
        assert_ne!(receipt.receipt_sha256, Digest32::zero());
    }

    #[test]
    fn identity_and_physics_poisons_fail_closed() {
        let (value, expected) = fixture();
        let sealed = Stage3LanePhysicalOutcomeLedgerV1::try_new(value, &expected).expect("ledger");
        let poisons: [fn(&mut Stage3LanePhysicalOutcomeLedgerV1); 14] = [
            |v| v.receipt_sha256 = d(90),
            |v| v.precipitation_set_sha256 = d(91),
            |v| v.beginning_snow_owner_sha256 = d(92),
            |v| v.ending_snow_owner_sha256 = d(93),
            |v| v.source_receipts_sha256[0] = Digest32::zero(),
            |v| v.source_receipts_sha256[5] = v.source_receipts_sha256[4],
            |v| v.shortwave_j_m2 = -v.shortwave_j_m2,
            |v| v.latent_j_m2 = -v.latent_j_m2,
            |v| v.vapor_transfer_kg_m2 = -v.vapor_transfer_kg_m2,
            |v| v.interlayer_conduction_j_m2 = 1.0,
            |v| v.interlayer_lower_conduction_j_m2 += 1.0,
            |v| v.refreeze_fusion_j_m2 = 1.0,
            |v| v.vapor_material_enthalpy_j_m2 += 1.0,
            |v| v.ending_liquid_kg_m2 += 0.01,
        ];
        for poison in poisons {
            let mut candidate = sealed.clone();
            poison(&mut candidate);
            assert!(candidate.validate(&expected).is_err());
        }
    }

    #[test]
    fn resealed_substitution_does_not_replace_external_authority() {
        let (mut value, expected) = fixture();
        let mut substituted_expected = expected.clone();
        substituted_expected.ending_snow_owner_sha256 = d(77);
        value.ending_snow_owner_sha256 = d(77);
        let substituted = Stage3LanePhysicalOutcomeLedgerV1::try_new(value, &substituted_expected)
            .expect("internally valid substituted receipt");
        assert!(substituted.validate(&expected).is_err());
    }

    #[test]
    fn resealed_ending_material_owner_mutation_fails_physical_closure() {
        let (mut value, expected) = fixture();
        value.ending_snow_owner_sha256 = d(77);
        value.ending_liquid_kg_m2 += 0.01;
        value.retained_liquid_kg_m2 += 0.01;
        value.ending_cold_content_j_m2 += 3_336.0;
        value.ending_enthalpy_j_m2 =
            -value.ending_cold_content_j_m2 + 333_600.0 * value.ending_liquid_kg_m2;
        let mut substituted_expected = expected.clone();
        substituted_expected.ending_snow_owner_sha256 = d(77);
        assert!(Stage3LanePhysicalOutcomeLedgerV1::try_new(value, &substituted_expected).is_err());
    }
}
