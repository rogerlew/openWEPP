use crate::snow_stage3_v11_attachment::{
    DirectSnowStage3V11AttachmentError, DirectSnowStage3V11CommittedState,
    DirectSnowStage3V11PreparedDay, DirectSnowStage3V11ShadowAttachment,
    DirectSnowStage3V11StaticContext,
};

use super::{DirectDayFrame, DirectPublicationDayInput, DirectRunFrame, DirectRuntimeError};

impl DirectRunFrame {
    pub fn configure_snow_stage3_v11_attachment(
        &mut self,
        static_context: DirectSnowStage3V11StaticContext,
        committed: DirectSnowStage3V11CommittedState,
    ) -> Result<(), DirectRuntimeError> {
        let attachment = DirectSnowStage3V11ShadowAttachment::new(static_context, committed)
            .map_err(attachment_runtime_error("configure"))?;
        self.snow_stage3_v11_attachment = Some(Box::new(attachment));
        Ok(())
    }

    pub fn prepare_snow_stage3_v11_day(
        &mut self,
        prepared: DirectSnowStage3V11PreparedDay,
    ) -> Result<(), DirectRuntimeError> {
        let attachment = self.snow_stage3_v11_attachment.as_deref_mut().ok_or(
            DirectRuntimeError::DirectKernelGuardFailure {
                phase: "snow_stage3_v11.prepare",
                detail: "constitutive attachment is not installed".into(),
            },
        )?;
        attachment
            .stage_prepared_day(prepared)
            .map_err(attachment_runtime_error("prepare"))
    }

    pub(crate) fn stage_snow_stage3_shadow(
        &mut self,
        day_input: &DirectPublicationDayInput,
        day_frame: &DirectDayFrame,
    ) -> Result<(), DirectRuntimeError> {
        #[cfg(test)]
        if let Some(mut attachment) = self.snow_stage3_shadow.take().map(|value| *value) {
            let result = attachment.stage_after_live_day(self, day_input, day_frame);
            self.snow_stage3_shadow = Some(Box::new(attachment));
            return result;
        }
        #[cfg(not(test))]
        let _ = (day_input, day_frame);
        // In production the constitutive attachment is staged only from its
        // sealed 48-support capability. It never reads the completed day
        // frame as a physical beginning state.
        Ok(())
    }

    pub(crate) fn commit_snow_stage3_shadow(&mut self) -> Result<(), DirectRuntimeError> {
        #[cfg(test)]
        if let Some(mut attachment) = self.snow_stage3_shadow.take().map(|value| *value) {
            let result = attachment.commit_after_live_day(self);
            self.snow_stage3_shadow = Some(Box::new(attachment));
            return result;
        }
        if let Some(attachment) = self.snow_stage3_v11_attachment.as_deref_mut() {
            attachment
                .commit_staged_day()
                .map_err(attachment_runtime_error("commit"))?;
        }
        Ok(())
    }
}

fn attachment_runtime_error(
    phase: &'static str,
) -> impl Fn(DirectSnowStage3V11AttachmentError) -> DirectRuntimeError {
    move |error| DirectRuntimeError::DirectKernelGuardFailure {
        phase: "snow_stage3_v11",
        detail: format!("{phase}: {error}"),
    }
}
