use openwepp_kernel_contract::{
    BoundarySymbol, KernelRunResponse, KernelWritebackPayload, WatershedChannelFluxField,
    WatershedChannelStateField, WatershedImpoundmentFluxField, WatershedImpoundmentStateField,
    WatershedKernel, WatershedKernelRequest, WatershedProductionFluxSymbol,
    WatershedProductionStateSymbol, WritebackField,
};
use openwepp_sim_contract::status::{BoundaryClass, SimulationPhase, SimulationStatus};

include!("constants.rs");
include!("types.rs");
include!("helpers.rs");
include!("routing.rs");
include!("diagnostics.rs");
include!("validation.rs");

impl WatershedKernel for Ws10ChannelImpoundmentKernel {
    fn run_watershed_node(&mut self, request: &WatershedKernelRequest<'_>) -> KernelRunResponse {
        let response = match request.node_kind {
            "channel" => Self::run_channel_node(request),
            "impoundment" => Self::run_impoundment_node(request),
            _ => Err(Self::domain_violation(
                Ws10NodeClass::Channel,
                BoundarySymbol::from("node_kind"),
                -1.0,
            )),
        };

        match response {
            Ok(response) => response,
            Err(error) => KernelRunResponse::new(
                Self::status_from_guard_error(&error),
                KernelWritebackPayload::empty(),
            ),
        }
    }
}
