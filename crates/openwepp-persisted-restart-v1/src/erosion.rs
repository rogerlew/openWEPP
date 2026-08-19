use crate::HexF64;
use openwepp_hillslope_orchestrator::{
    DirectErosionConsolidationCarry, DirectErosionDownstreamOperands, DirectErosionInflowIntake,
    DirectErosionRuntimeCarry, DirectPublicationErosionOperands, ErosionIfrostCarry,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;
#[derive(Debug, Error, PartialEq, Eq)]
pub enum ErosionRestartError {
    #[error("{field} violates erosion domain")]
    Domain { field: &'static str },
    #[error("ifrost must be 0..=2")]
    Ifrost,
    #[error("erosion publication authority and payload posture disagree")]
    PublicationAuthority,
}
fn nn(field: &'static str, v: &HexF64) -> Result<f64, ErosionRestartError> {
    let x = v.to_f64();
    (x.is_finite() && x >= 0.0)
        .then_some(x)
        .ok_or(ErosionRestartError::Domain { field })
}
fn opt(field: &'static str, v: &Option<HexF64>) -> Result<Option<f64>, ErosionRestartError> {
    v.as_ref().map(|v| nn(field, v)).transpose()
}
fn arr<const N: usize>(v: [f64; N]) -> [HexF64; N] {
    v.map(HexF64::from_f64)
}
fn unarr<const N: usize>(
    field: &'static str,
    v: &[HexF64; N],
) -> Result<[f64; N], ErosionRestartError> {
    v.iter()
        .map(|v| nn(field, v))
        .collect::<Result<Vec<_>, _>>()?
        .try_into()
        .map_err(|_| unreachable!())
}
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectErosionRuntimeCarryRestartV1 {
    pub rfcum_m: HexF64,
    pub daydis: HexF64,
    pub ifrost: u8,
    pub rill_width_m: HexF64,
}
impl DirectErosionRuntimeCarryRestartV1 {
    pub fn project(v: &DirectErosionRuntimeCarry) -> Self {
        let DirectErosionRuntimeCarry {
            consolidation,
            ifrost,
            rill_width_m,
        } = v;
        let DirectErosionConsolidationCarry { rfcum_m, daydis } = *consolidation;
        let ErosionIfrostCarry(ifrost) = *ifrost;
        Self {
            rfcum_m: HexF64::from_f64(rfcum_m),
            daydis: HexF64::from_f64(daydis),
            ifrost,
            rill_width_m: HexF64::from_f64(*rill_width_m),
        }
    }
    pub fn restore(&self) -> Result<DirectErosionRuntimeCarry, ErosionRestartError> {
        if self.ifrost > 2 {
            return Err(ErosionRestartError::Ifrost);
        }
        Ok(DirectErosionRuntimeCarry {
            consolidation: DirectErosionConsolidationCarry {
                rfcum_m: nn("erosion.rfcum_m", &self.rfcum_m)?,
                daydis: nn("erosion.daydis", &self.daydis)?,
            },
            ifrost: ErosionIfrostCarry(self.ifrost),
            rill_width_m: nn("erosion.rill_width_m", &self.rill_width_m)?,
        })
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectErosionInflowIntakeRestartV1 {
    pub hourly_qout_m2_s: [HexF64; 24],
    pub hourly_qsout_kg_m_s: [HexF64; 24],
    pub prior_slpend: HexF64,
    pub prior_cnslp: HexF64,
    pub prior_end_shear: [HexF64; 3],
    pub prior_end_transport: [HexF64; 3],
    pub exit_fractions: [HexF64; 5],
}
impl DirectErosionInflowIntakeRestartV1 {
    pub fn project(v: &DirectErosionInflowIntake) -> Self {
        let DirectErosionInflowIntake {
            hourly_qout_m2_s,
            hourly_qsout_kg_m_s,
            prior_slpend,
            prior_cnslp,
            prior_end_shear,
            prior_end_transport,
            exit_fractions,
        } = *v;
        Self {
            hourly_qout_m2_s: arr(hourly_qout_m2_s),
            hourly_qsout_kg_m_s: arr(hourly_qsout_kg_m_s),
            prior_slpend: HexF64::from_f64(prior_slpend),
            prior_cnslp: HexF64::from_f64(prior_cnslp),
            prior_end_shear: arr([prior_end_shear.0, prior_end_shear.1, prior_end_shear.2]),
            prior_end_transport: arr([
                prior_end_transport.0,
                prior_end_transport.1,
                prior_end_transport.2,
            ]),
            exit_fractions: arr(exit_fractions),
        }
    }
    pub fn restore(&self) -> Result<DirectErosionInflowIntake, ErosionRestartError> {
        let shear = unarr("erosion.prior_end_shear", &self.prior_end_shear)?;
        let transport = unarr("erosion.prior_end_transport", &self.prior_end_transport)?;
        let fractions = unarr("erosion.exit_fractions", &self.exit_fractions)?;
        if (fractions.iter().sum::<f64>() - 1.0).abs() > 1e-12 {
            return Err(ErosionRestartError::Domain {
                field: "erosion.exit_fractions",
            });
        }
        Ok(DirectErosionInflowIntake {
            hourly_qout_m2_s: unarr("erosion.hourly_qout_m2_s", &self.hourly_qout_m2_s)?,
            hourly_qsout_kg_m_s: unarr("erosion.hourly_qsout_kg_m_s", &self.hourly_qsout_kg_m_s)?,
            prior_slpend: nn("erosion.prior_slpend", &self.prior_slpend)?,
            prior_cnslp: nn("erosion.prior_cnslp", &self.prior_cnslp)?,
            prior_end_shear: (shear[0], shear[1], shear[2]),
            prior_end_transport: (transport[0], transport[1], transport[2]),
            exit_fractions: fractions,
        })
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PublicationErosionRestartV1 {
    pub peak_runoff_rate_m_s: Option<HexF64>,
    pub runoff_duration_s: Option<HexF64>,
    pub total_detachment_kg: Option<HexF64>,
    pub total_deposition_kg: Option<HexF64>,
    pub hbp_total_detachment_kg: Option<HexF64>,
    pub hbp_total_deposition_kg: Option<HexF64>,
    pub hbp_sediment_concentration_kg_m3: Option<HexF64>,
    pub sediment_concentration_kg_m3: Option<[HexF64; 5]>,
    pub hourly_runoff_fraction: Option<[HexF64; 24]>,
    pub hourly_sediment_mass_kg: Option<[HexF64; 24]>,
    pub enrichment_ratio: Option<HexF64>,
}
fn enc(v: Option<f64>) -> Option<HexF64> {
    v.map(HexF64::from_f64)
}
impl PublicationErosionRestartV1 {
    fn project(v: &DirectPublicationErosionOperands) -> Self {
        let DirectPublicationErosionOperands {
            peak_runoff_rate_m_s,
            runoff_duration_s,
            total_detachment_kg,
            total_deposition_kg,
            hbp_total_detachment_kg,
            hbp_total_deposition_kg,
            hbp_sediment_concentration_kg_m3,
            sediment_concentration_kg_m3,
            hourly_runoff_fraction,
            hourly_sediment_mass_kg,
            enrichment_ratio,
        } = *v;
        Self {
            peak_runoff_rate_m_s: enc(peak_runoff_rate_m_s),
            runoff_duration_s: enc(runoff_duration_s),
            total_detachment_kg: enc(total_detachment_kg),
            total_deposition_kg: enc(total_deposition_kg),
            hbp_total_detachment_kg: enc(hbp_total_detachment_kg),
            hbp_total_deposition_kg: enc(hbp_total_deposition_kg),
            hbp_sediment_concentration_kg_m3: enc(hbp_sediment_concentration_kg_m3),
            sediment_concentration_kg_m3: sediment_concentration_kg_m3.map(arr),
            hourly_runoff_fraction: hourly_runoff_fraction.map(arr),
            hourly_sediment_mass_kg: hourly_sediment_mass_kg.map(arr),
            enrichment_ratio: enc(enrichment_ratio),
        }
    }
    fn restore(&self) -> Result<DirectPublicationErosionOperands, ErosionRestartError> {
        Ok(DirectPublicationErosionOperands {
            peak_runoff_rate_m_s: opt("erosion.peak_runoff_rate_m_s", &self.peak_runoff_rate_m_s)?,
            runoff_duration_s: opt("erosion.runoff_duration_s", &self.runoff_duration_s)?,
            total_detachment_kg: opt("erosion.total_detachment_kg", &self.total_detachment_kg)?,
            total_deposition_kg: opt("erosion.total_deposition_kg", &self.total_deposition_kg)?,
            hbp_total_detachment_kg: opt(
                "erosion.hbp_total_detachment_kg",
                &self.hbp_total_detachment_kg,
            )?,
            hbp_total_deposition_kg: opt(
                "erosion.hbp_total_deposition_kg",
                &self.hbp_total_deposition_kg,
            )?,
            hbp_sediment_concentration_kg_m3: opt(
                "erosion.hbp_sediment_concentration_kg_m3",
                &self.hbp_sediment_concentration_kg_m3,
            )?,
            sediment_concentration_kg_m3: self
                .sediment_concentration_kg_m3
                .as_ref()
                .map(|v| unarr("erosion.sediment_concentration_kg_m3", v))
                .transpose()?,
            hourly_runoff_fraction: self
                .hourly_runoff_fraction
                .as_ref()
                .map(|v| unarr("erosion.hourly_runoff_fraction", v))
                .transpose()?,
            hourly_sediment_mass_kg: self
                .hourly_sediment_mass_kg
                .as_ref()
                .map(|v| unarr("erosion.hourly_sediment_mass_kg", v))
                .transpose()?,
            enrichment_ratio: opt("erosion.enrichment_ratio", &self.enrichment_ratio)?,
        })
    }
}
#[derive(Clone, Debug, Deserialize, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields)]
pub struct DirectErosionDownstreamRestartV1 {
    pub publication_authority: bool,
    pub publication: PublicationErosionRestartV1,
}
impl DirectErosionDownstreamRestartV1 {
    pub fn project(v: &DirectErosionDownstreamOperands) -> Self {
        let DirectErosionDownstreamOperands {
            publication_authority,
            publication,
        } = v;
        Self {
            publication_authority: *publication_authority,
            publication: PublicationErosionRestartV1::project(publication),
        }
    }
    pub fn restore(&self) -> Result<DirectErosionDownstreamOperands, ErosionRestartError> {
        let core = [
            self.publication.peak_runoff_rate_m_s.as_ref(),
            self.publication.runoff_duration_s.as_ref(),
            self.publication.total_detachment_kg.as_ref(),
            self.publication.total_deposition_kg.as_ref(),
            self.publication.hbp_total_detachment_kg.as_ref(),
            self.publication.hbp_total_deposition_kg.as_ref(),
            self.publication.hbp_sediment_concentration_kg_m3.as_ref(),
        ];
        if self.publication_authority {
            if core.iter().any(|value| value.is_none())
                || self.publication.sediment_concentration_kg_m3.is_none()
            {
                return Err(ErosionRestartError::PublicationAuthority);
            }
            if self.publication.hourly_runoff_fraction.is_some()
                != self.publication.hourly_sediment_mass_kg.is_some()
            {
                return Err(ErosionRestartError::PublicationAuthority);
            }
        } else {
            let core_is_zero = core
                .iter()
                .all(|value| value.is_some_and(|value| value.to_f64() == 0.0));
            let classes_are_zero = self
                .publication
                .sediment_concentration_kg_m3
                .as_ref()
                .is_some_and(|values| values.iter().all(|value| value.to_f64() == 0.0));
            if !core_is_zero
                || !classes_are_zero
                || self.publication.hourly_runoff_fraction.is_some()
                || self.publication.hourly_sediment_mass_kg.is_some()
                || self.publication.enrichment_ratio.is_some()
            {
                return Err(ErosionRestartError::PublicationAuthority);
            }
        }
        Ok(DirectErosionDownstreamOperands {
            publication_authority: self.publication_authority,
            publication: self.publication.restore()?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn erosion_runtime_inflow_and_downstream_round_trip_exactly() {
        let runtime = DirectErosionRuntimeCarry {
            consolidation: DirectErosionConsolidationCarry {
                rfcum_m: -0.0,
                daydis: 4.0,
            },
            ifrost: ErosionIfrostCarry(2),
            rill_width_m: 0.03,
        };
        let dto = DirectErosionRuntimeCarryRestartV1::project(&runtime);
        assert_eq!(
            DirectErosionRuntimeCarryRestartV1::project(&dto.restore().unwrap()),
            dto
        );
        let intake = DirectErosionInflowIntake {
            hourly_qout_m2_s: std::array::from_fn(|i| i as f64 / 1000.0),
            hourly_qsout_kg_m_s: std::array::from_fn(|i| i as f64 / 2000.0),
            prior_slpend: 0.2,
            prior_cnslp: 0.1,
            prior_end_shear: (-0.0, 0.2, 0.3),
            prior_end_transport: (0.4, 0.5, 0.6),
            exit_fractions: [0.1, 0.2, 0.3, 0.15, 0.25],
        };
        let dto = DirectErosionInflowIntakeRestartV1::project(&intake);
        assert_eq!(
            DirectErosionInflowIntakeRestartV1::project(&dto.restore().unwrap()),
            dto
        );
        let downstream = DirectErosionDownstreamOperands::zero();
        let dto = DirectErosionDownstreamRestartV1::project(&downstream);
        assert_eq!(
            DirectErosionDownstreamRestartV1::project(&dto.restore().unwrap()),
            dto
        )
    }
    #[test]
    fn erosion_domains_and_cardinality_reject() {
        let mut dto =
            DirectErosionRuntimeCarryRestartV1::project(&DirectErosionRuntimeCarry::inert());
        dto.ifrost = 3;
        assert_eq!(dto.restore(), Err(ErosionRestartError::Ifrost));
        dto.ifrost = 0;
        dto.rill_width_m = HexF64::from_f64(f64::NAN);
        assert!(matches!(
            dto.restore(),
            Err(ErosionRestartError::Domain { .. })
        ));
        let intake = DirectErosionInflowIntake {
            hourly_qout_m2_s: [0.0; 24],
            hourly_qsout_kg_m_s: [0.0; 24],
            prior_slpend: 0.0,
            prior_cnslp: 0.0,
            prior_end_shear: (0.0, 0.0, 0.0),
            prior_end_transport: (0.0, 0.0, 0.0),
            exit_fractions: [0.1; 5],
        };
        let dto = DirectErosionInflowIntakeRestartV1::project(&intake);
        assert_eq!(
            dto.restore(),
            Err(ErosionRestartError::Domain {
                field: "erosion.exit_fractions"
            })
        );
        let mut publication =
            DirectErosionDownstreamRestartV1::project(&DirectErosionDownstreamOperands::zero());
        publication.publication_authority = true;
        publication.publication.total_detachment_kg = None;
        assert_eq!(
            publication.restore(),
            Err(ErosionRestartError::PublicationAuthority)
        );
    }
}
