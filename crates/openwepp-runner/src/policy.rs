use openwepp_input_contract::parsers::climate::{
    CompatibilityOptions, ParserMode as ClimateParserMode,
};
use openwepp_input_contract::parsers::frost::ParseMode as FrostParseMode;
use openwepp_input_contract::parsers::management::ParseMode as ManagementParseMode;
use openwepp_input_contract::parsers::pmetpara::ParseMode as PmetparaParseMode;
use openwepp_input_contract::parsers::slope::SlopeParserOptions;
use openwepp_input_contract::parsers::snow::{ParseMode as SnowParseMode, SnowParseOptions};
use openwepp_input_contract::parsers::soil::ParserMode as SoilParserMode;
use openwepp_input_contract::parsers::wepp_ui::WeppUiParserMode;
use openwepp_legacy_bridge::policy::CompatibilityPolicy;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidecarPolicy {
    Compat,
}

impl SidecarPolicy {
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        "compat"
    }

    #[must_use]
    pub const fn as_legacy_bridge_policy(self) -> CompatibilityPolicy {
        CompatibilityPolicy::Compat
    }

    #[must_use]
    pub const fn as_soil_parser_mode(self) -> SoilParserMode {
        SoilParserMode::Compatibility
    }

    #[must_use]
    pub const fn as_slope_parser_options(self) -> SlopeParserOptions {
        SlopeParserOptions::compatibility()
    }

    #[must_use]
    pub const fn as_management_parser_mode(self) -> ManagementParseMode {
        ManagementParseMode::Compatibility
    }

    #[must_use]
    pub fn as_climate_parser_mode(self) -> ClimateParserMode {
        ClimateParserMode::Compatibility(CompatibilityOptions::default())
    }

    #[must_use]
    pub const fn as_snow_parse_options(self) -> SnowParseOptions {
        SnowParseOptions {
            mode: SnowParseMode::Compatibility,
        }
    }

    #[must_use]
    pub const fn as_frost_parse_mode(self) -> FrostParseMode {
        FrostParseMode::Compatibility
    }

    #[must_use]
    pub const fn as_wepp_ui_parse_mode(self) -> WeppUiParserMode {
        WeppUiParserMode::Compatibility
    }

    #[must_use]
    pub const fn as_pmetpara_parse_mode(self) -> PmetparaParseMode {
        PmetparaParseMode::Compatibility
    }
}

impl std::str::FromStr for SidecarPolicy {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "compat" => Ok(Self::Compat),
            _ => Err(format!(
                "unsupported sidecar policy '{value}' (expected compat)"
            )),
        }
    }
}
