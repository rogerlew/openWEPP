// Physical-custody error projection for the V11 real consumer.

impl DirectV11RealConsumerError {
    pub(crate) fn from_stage3_physical_custody(error: &DirectSnowStage3V11AttachmentError) -> Self {
        match error {
            DirectSnowStage3V11AttachmentError::Precipitation(detail) => {
                Self::Stage3PrecipitationCustody(detail)
            }
            DirectSnowStage3V11AttachmentError::SnowSoilHeat(detail) => {
                Self::Stage3SnowSoilHeatCustody(detail)
            }
            _ => Self::Identity("Stage-3 physical-custody attachment"),
        }
    }
}
