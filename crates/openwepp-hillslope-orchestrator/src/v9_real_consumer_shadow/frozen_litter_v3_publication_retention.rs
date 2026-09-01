//! Canonical frozen-litter V3 publication retention seam.

use openwepp_coupled_time::{Digest32, digest_bytes};
use serde::{Deserialize, Serialize};

// The successor runtime adopter is a separate manifest-owned slice. Keep this
// canonical seam warning-clean while it remains deliberately unwired.
#[allow(dead_code)]
const FROZEN_LITTER_V3_PUBLICATION_SUPPORT_SCHEMA: &str =
    "OPENWEPP_FROZEN_LITTER_V3_PUBLICATION_SUPPORT_V1";

#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
struct FrozenLitterV3PublicationReceiptFrameV1 {
    ordinal: u32,
    model_version: String,
    model_definition_sha256: String,
    lse_configuration_sha256: String,
    receipt_sha256: String,
    canonical_receipt_bytes: Vec<u8>,
}

/// Additive successor publication value for one accepted frozen-litter
/// support. V1 publication bytes remain unchanged; a later runtime adopter
/// must retain this value beside, never inside, the V1 support.
#[derive(Clone, Debug, Deserialize, PartialEq, Serialize)]
#[serde(deny_unknown_fields)]
#[allow(dead_code)]
pub(crate) struct FrozenLitterV3PublicationSupportV1 {
    schema: String,
    surface_configuration_sha256: String,
    projection_sha256: String,
    projection_receipt_chain_sha256: String,
    canonical_projection_bytes: Vec<u8>,
    ordered_litter_phase_receipts: Vec<FrozenLitterV3PublicationReceiptFrameV1>,
    publication_sha256: Digest32,
}

#[derive(Deserialize)]
#[allow(dead_code)]
struct FrozenLitterV3ProjectionRetentionView {
    receipt_chain_sha256: String,
    litter_phase_receipt_bytes: Vec<Vec<u8>>,
    projection_sha256: String,
}

#[allow(dead_code)]
impl FrozenLitterV3PublicationSupportV1 {
    pub(crate) fn try_new(
        configuration: &crate::SurfaceLiquidConfigurationV2,
        projection: &crate::SurfaceLiquidCompleteOwnerProjectionV3,
        ordered_receipts: &[openwepp_land_surface_energy::LitterPhaseReceipt],
    ) -> Result<Self, crate::DirectSurfaceLiquidError> {
        let canonical_projection_bytes = projection.canonical_bytes(configuration)?;
        let ordered_litter_phase_receipts = ordered_receipts
            .iter()
            .enumerate()
            .map(|(ordinal, receipt)| {
                let ordinal = u32::try_from(ordinal).map_err(|_| {
                    crate::DirectSurfaceLiquidError::Identity(
                        "frozen-litter V3 publication receipt ordinal width",
                    )
                })?;
                let canonical_receipt_bytes =
                    openwepp_land_surface_energy::litter_phase_receipt_json(receipt).map_err(
                        |_| {
                            crate::DirectSurfaceLiquidError::Identity(
                                "invalid frozen-litter V3 publication receipt",
                            )
                        },
                    )?;
                Ok(FrozenLitterV3PublicationReceiptFrameV1 {
                    ordinal,
                    model_version: receipt.identity.model_version.clone(),
                    model_definition_sha256: receipt.identity.model_definition_sha256.to_string(),
                    lse_configuration_sha256: receipt.identity.lse_configuration_sha256.to_string(),
                    receipt_sha256: receipt.receipt_sha256.to_string(),
                    canonical_receipt_bytes,
                })
            })
            .collect::<Result<Vec<_>, crate::DirectSurfaceLiquidError>>()?;
        let mut value = Self {
            schema: FROZEN_LITTER_V3_PUBLICATION_SUPPORT_SCHEMA.into(),
            surface_configuration_sha256: configuration.configuration_sha256().into(),
            projection_sha256: projection.projection_sha256().into(),
            projection_receipt_chain_sha256: projection.identity().receipt_chain_sha256.clone(),
            canonical_projection_bytes,
            ordered_litter_phase_receipts,
            publication_sha256: Digest32::zero(),
        };
        value.publication_sha256 = value.recomputed_sha256()?;
        value.validate(configuration)?;
        Ok(value)
    }

    pub(crate) fn canonical_bytes(
        &self,
        configuration: &crate::SurfaceLiquidConfigurationV2,
    ) -> Result<Vec<u8>, crate::DirectSurfaceLiquidError> {
        self.validate(configuration)?;
        serde_json::to_vec(self).map_err(|_| {
            crate::DirectSurfaceLiquidError::Schema(
                "frozen-litter V3 publication support serialization",
            )
        })
    }

    #[must_use]
    pub(crate) const fn publication_sha256(&self) -> &Digest32 {
        &self.publication_sha256
    }

    pub(crate) fn from_canonical_bytes(
        configuration: &crate::SurfaceLiquidConfigurationV2,
        bytes: &[u8],
    ) -> Result<Self, crate::DirectSurfaceLiquidError> {
        let value: Self = serde_json::from_slice(bytes).map_err(|_| {
            crate::DirectSurfaceLiquidError::Schema("frozen-litter V3 publication support parsing")
        })?;
        value.validate(configuration)?;
        if serde_json::to_vec(&value).map_err(|_| {
            crate::DirectSurfaceLiquidError::Schema(
                "frozen-litter V3 publication support canonicalization",
            )
        })? != bytes
        {
            return Err(crate::DirectSurfaceLiquidError::Schema(
                "noncanonical frozen-litter V3 publication support bytes",
            ));
        }
        Ok(value)
    }

    pub(crate) fn complete_owner_projection(
        &self,
        configuration: &crate::SurfaceLiquidConfigurationV2,
    ) -> Result<crate::SurfaceLiquidCompleteOwnerProjectionV3, crate::DirectSurfaceLiquidError>
    {
        self.validate(configuration)?;
        crate::SurfaceLiquidCompleteOwnerProjectionV3::from_canonical_bytes(
            configuration,
            &self.canonical_projection_bytes,
        )
    }

    pub(crate) fn ordered_litter_phase_receipts(
        &self,
        configuration: &crate::SurfaceLiquidConfigurationV2,
    ) -> Result<
        Vec<openwepp_land_surface_energy::LitterPhaseReceipt>,
        crate::DirectSurfaceLiquidError,
    > {
        self.validate(configuration)?;
        self.replay_receipts()
    }

    fn validate(
        &self,
        configuration: &crate::SurfaceLiquidConfigurationV2,
    ) -> Result<(), crate::DirectSurfaceLiquidError> {
        let projection = crate::SurfaceLiquidCompleteOwnerProjectionV3::from_canonical_bytes(
            configuration,
            &self.canonical_projection_bytes,
        )?;
        let projection_view: FrozenLitterV3ProjectionRetentionView =
            serde_json::from_slice(&self.canonical_projection_bytes).map_err(|_| {
                crate::DirectSurfaceLiquidError::Schema("frozen-litter V3 retained projection view")
            })?;
        let receipts = self.replay_receipts()?;
        let expected_count = configuration
            .parent()
            .records
            .iter()
            .filter(|record| {
                record.key.surface_class == openwepp_land_surface_energy::SurfaceClass::ForestLitter
            })
            .count();
        if self.schema != FROZEN_LITTER_V3_PUBLICATION_SUPPORT_SCHEMA
            || self.surface_configuration_sha256 != configuration.configuration_sha256()
            || self.projection_sha256 != projection.projection_sha256()
            || self.projection_sha256 != projection_view.projection_sha256
            || self.projection_receipt_chain_sha256 != projection.identity().receipt_chain_sha256
            || self.projection_receipt_chain_sha256 != projection_view.receipt_chain_sha256
            || projection_view.litter_phase_receipt_bytes
                != self
                    .ordered_litter_phase_receipts
                    .iter()
                    .map(|frame| frame.canonical_receipt_bytes.clone())
                    .collect::<Vec<_>>()
            || expected_count != receipts.len()
            || self.publication_sha256 == Digest32::zero()
            || self.publication_sha256 != self.recomputed_sha256()?
        {
            return Err(crate::DirectSurfaceLiquidError::Identity(
                "frozen-litter V3 publication support seal",
            ));
        }
        for ((ordinal, frame), receipt) in self
            .ordered_litter_phase_receipts
            .iter()
            .enumerate()
            .zip(&receipts)
        {
            let configured = configuration
                .parent()
                .records
                .iter()
                .filter(|record| {
                    record.key.surface_class
                        == openwepp_land_surface_energy::SurfaceClass::ForestLitter
                })
                .nth(ordinal)
                .ok_or(crate::DirectSurfaceLiquidError::Identity(
                    "frozen-litter V3 publication receipt order",
                ))?;
            if usize::try_from(frame.ordinal) != Ok(ordinal)
                || frame.model_version != receipt.identity.model_version
                || frame.model_definition_sha256
                    != receipt.identity.model_definition_sha256.as_str()
                || frame.lse_configuration_sha256
                    != receipt.identity.lse_configuration_sha256.as_str()
                || frame.receipt_sha256 != receipt.receipt_sha256.as_str()
                || receipt.identity.transaction_id != projection.identity().transaction_id
                || receipt.identity.support_start_ns != projection.identity().support_start_ns
                || receipt.identity.support_end_ns != projection.identity().support_end_ns
                || receipt.identity.ofe_id != configured.key.ofe_id
                || receipt.identity.tile_id != configured.key.tile_id
            {
                return Err(crate::DirectSurfaceLiquidError::Identity(
                    "frozen-litter V3 publication receipt identity",
                ));
            }
        }
        Ok(())
    }

    fn replay_receipts(
        &self,
    ) -> Result<
        Vec<openwepp_land_surface_energy::LitterPhaseReceipt>,
        crate::DirectSurfaceLiquidError,
    > {
        self.ordered_litter_phase_receipts
            .iter()
            .map(|frame| {
                let receipt = openwepp_land_surface_energy::litter_phase_receipt_from_json(
                    &frame.canonical_receipt_bytes,
                )
                .map_err(|_| {
                    crate::DirectSurfaceLiquidError::Identity(
                        "frozen-litter V3 publication receipt replay",
                    )
                })?;
                if openwepp_land_surface_energy::litter_phase_receipt_json(&receipt).map_err(
                    |_| {
                        crate::DirectSurfaceLiquidError::Identity(
                            "frozen-litter V3 publication receipt canonical replay",
                        )
                    },
                )? != frame.canonical_receipt_bytes
                {
                    return Err(crate::DirectSurfaceLiquidError::Schema(
                        "noncanonical frozen-litter V3 publication receipt bytes",
                    ));
                }
                Ok(receipt)
            })
            .collect()
    }

    fn recomputed_sha256(&self) -> Result<Digest32, crate::DirectSurfaceLiquidError> {
        let mut value = self.clone();
        value.publication_sha256 = Digest32::zero();
        let bytes = serde_json::to_vec(&value).map_err(|_| {
            crate::DirectSurfaceLiquidError::Schema("frozen-litter V3 publication support digest")
        })?;
        Ok(digest_bytes(
            &[
                b"OPENWEPP_FROZEN_LITTER_V3_PUBLICATION_SUPPORT_V1\0".as_slice(),
                bytes.as_slice(),
            ]
            .concat(),
        ))
    }
}
