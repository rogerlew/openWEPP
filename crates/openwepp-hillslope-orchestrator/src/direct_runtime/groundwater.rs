use super::{DirectRuntimeError, validate_finite, validate_nonnegative_direct_m};

const GROUNDWATER_STORAGE_ROUNDOFF_TOLERANCE_M3: f64 = 1.0e-9;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DirectGroundwaterAuthority {
    Disabled,
    LinearReservoir {
        initial_storage_depth_m: f64,
        baseflow_coeff_per_day: f64,
        deep_seepage_coeff_per_day: f64,
        baseflow_threshold_area_ha: f64,
    },
}

impl DirectGroundwaterAuthority {
    #[must_use]
    pub const fn disabled() -> Self {
        Self::Disabled
    }

    pub fn linear_reservoir(
        initial_storage_depth_m: f64,
        baseflow_coeff_per_day: f64,
        deep_seepage_coeff_per_day: f64,
        baseflow_threshold_area_ha: f64,
    ) -> Result<Self, DirectRuntimeError> {
        validate_nonnegative_direct_m(
            "groundwater.initial_storage_depth_m",
            initial_storage_depth_m,
        )?;
        validate_nonnegative_direct_m(
            "groundwater.baseflow_coeff_per_day",
            baseflow_coeff_per_day,
        )?;
        validate_nonnegative_direct_m(
            "groundwater.deep_seepage_coeff_per_day",
            deep_seepage_coeff_per_day,
        )?;
        validate_nonnegative_direct_m(
            "groundwater.baseflow_threshold_area_ha",
            baseflow_threshold_area_ha,
        )?;
        Ok(Self::LinearReservoir {
            initial_storage_depth_m,
            baseflow_coeff_per_day,
            deep_seepage_coeff_per_day,
            baseflow_threshold_area_ha,
        })
    }

    #[must_use]
    pub const fn is_enabled(self) -> bool {
        matches!(self, Self::LinearReservoir { .. })
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectGroundwaterDayOutput {
    pub enabled: bool,
    pub recharge_m3: f64,
    pub storage_before_m3: f64,
    pub storage_after_m3: f64,
    pub storage_delta_m3: f64,
    pub baseflow_m3: f64,
    pub deep_seepage_m3: f64,
    pub baseflow_threshold_area_ha: Option<f64>,
}

impl DirectGroundwaterDayOutput {
    #[must_use]
    pub const fn zero() -> Self {
        Self {
            enabled: false,
            recharge_m3: 0.0,
            storage_before_m3: 0.0,
            storage_after_m3: 0.0,
            storage_delta_m3: 0.0,
            baseflow_m3: 0.0,
            deep_seepage_m3: 0.0,
            baseflow_threshold_area_ha: None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DirectGroundwaterRunState {
    pub authority: DirectGroundwaterAuthority,
    pub storage_m3: f64,
    pub previous_baseflow_m3: f64,
    pub previous_deep_seepage_m3: f64,
    pub initialized_area_m2: Option<f64>,
}

impl DirectGroundwaterRunState {
    #[must_use]
    pub const fn disabled() -> Self {
        Self {
            authority: DirectGroundwaterAuthority::Disabled,
            storage_m3: 0.0,
            previous_baseflow_m3: 0.0,
            previous_deep_seepage_m3: 0.0,
            initialized_area_m2: None,
        }
    }

    pub fn from_authority(
        authority: DirectGroundwaterAuthority,
        total_area_m2: f64,
    ) -> Result<Self, DirectRuntimeError> {
        validate_positive_area(total_area_m2)?;
        match authority {
            DirectGroundwaterAuthority::Disabled => Ok(Self::disabled()),
            DirectGroundwaterAuthority::LinearReservoir {
                initial_storage_depth_m,
                ..
            } => {
                let storage_m3 = initial_storage_depth_m * total_area_m2;
                validate_nonnegative_direct_m("groundwater.initial_storage_m3", storage_m3)?;
                Ok(Self {
                    authority,
                    storage_m3,
                    previous_baseflow_m3: 0.0,
                    previous_deep_seepage_m3: 0.0,
                    initialized_area_m2: Some(total_area_m2),
                })
            }
        }
    }

    #[must_use]
    pub const fn is_enabled(&self) -> bool {
        self.authority.is_enabled()
    }

    pub fn run_day(
        &mut self,
        recharge_m3: f64,
        total_area_m2: f64,
    ) -> Result<DirectGroundwaterDayOutput, DirectRuntimeError> {
        validate_positive_area(total_area_m2)?;
        validate_nonnegative_direct_m("groundwater.recharge_m3", recharge_m3)?;
        let Some(initialized_area_m2) = self.initialized_area_m2 else {
            if self.authority.is_enabled() {
                return Err(DirectRuntimeError::MissingDirectUpstream {
                    upstream: "groundwater initialized area",
                });
            }
            return Ok(DirectGroundwaterDayOutput::zero());
        };
        if (initialized_area_m2 - total_area_m2).abs()
            > GROUNDWATER_STORAGE_ROUNDOFF_TOLERANCE_M3.max(total_area_m2.abs() * 1.0e-12)
        {
            return Err(DirectRuntimeError::DirectDomainViolation {
                field: "groundwater.total_area_m2",
            });
        }

        match self.authority {
            DirectGroundwaterAuthority::Disabled => Ok(DirectGroundwaterDayOutput::zero()),
            DirectGroundwaterAuthority::LinearReservoir {
                baseflow_coeff_per_day,
                deep_seepage_coeff_per_day,
                baseflow_threshold_area_ha,
                ..
            } => {
                let storage_before_m3 = self.storage_m3;
                let mut storage_after_m3 = storage_before_m3 + recharge_m3
                    - self.previous_baseflow_m3
                    - self.previous_deep_seepage_m3;
                validate_finite("groundwater.storage_after_m3", storage_after_m3)?;
                if storage_after_m3 < 0.0 {
                    if storage_after_m3.abs() <= GROUNDWATER_STORAGE_ROUNDOFF_TOLERANCE_M3 {
                        storage_after_m3 = 0.0;
                    } else {
                        return Err(DirectRuntimeError::NegativeDirectValue {
                            field: "groundwater.storage_after_m3",
                        });
                    }
                }
                let baseflow_m3 = baseflow_coeff_per_day * storage_after_m3;
                let deep_seepage_m3 = deep_seepage_coeff_per_day * storage_after_m3;
                validate_nonnegative_direct_m("groundwater.baseflow_m3", baseflow_m3)?;
                validate_nonnegative_direct_m("groundwater.deep_seepage_m3", deep_seepage_m3)?;
                if baseflow_m3 + deep_seepage_m3
                    > storage_after_m3 + GROUNDWATER_STORAGE_ROUNDOFF_TOLERANCE_M3
                {
                    return Err(DirectRuntimeError::DirectKernelGuardFailure {
                        phase: "groundwater_linear_reservoir",
                        detail: format!(
                            "generated groundwater exports {} m3 exceed accepted storage {} m3",
                            baseflow_m3 + deep_seepage_m3,
                            storage_after_m3
                        ),
                    });
                }
                self.storage_m3 = storage_after_m3;
                self.previous_baseflow_m3 = baseflow_m3;
                self.previous_deep_seepage_m3 = deep_seepage_m3;
                Ok(DirectGroundwaterDayOutput {
                    enabled: true,
                    recharge_m3,
                    storage_before_m3,
                    storage_after_m3,
                    storage_delta_m3: storage_after_m3 - storage_before_m3,
                    baseflow_m3,
                    deep_seepage_m3,
                    baseflow_threshold_area_ha: Some(baseflow_threshold_area_ha),
                })
            }
        }
    }
}

fn validate_positive_area(total_area_m2: f64) -> Result<(), DirectRuntimeError> {
    validate_finite("groundwater.total_area_m2", total_area_m2)?;
    if total_area_m2 <= 0.0 {
        return Err(DirectRuntimeError::DirectDomainViolation {
            field: "groundwater.total_area_m2",
        });
    }
    Ok(())
}
