struct TerminalReceiverTopologyV1 {
    destination_ofe: OfeId,
    receiver_destinations: Vec<DirectSnowStage3V11TerminalReceiverDestinationV1>,
    digest: Digest32,
}

fn terminal_receiver_topology(
    configuration: &DirectSurfaceLiquidConfiguration,
    lane_id: u32,
) -> Result<TerminalReceiverTopologyV1, DirectSnowStage3V11AttachmentError> {
    let destination_ofe = configuration
        .ofe_bindings
        .iter()
        .find(|binding| binding.production_lane_id == lane_id)
        .map(|binding| binding.ofe_id.clone())
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "terminal receiver lane binding",
        ))?;
    let records = configuration
        .records
        .iter()
        .filter(|record| record.key.ofe_id == destination_ofe)
        .collect::<Vec<_>>();
    let fraction_sum = records
        .iter()
        .map(|record| record.tile_fraction)
        .sum::<f64>();
    if records.is_empty() || (fraction_sum - 1.0).abs() > 1.0e-12 {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "terminal receiver topology closure",
        ));
    }
    let mut topology_bytes = Vec::new();
    for record in &records {
        topology_bytes.extend_from_slice(record.key.ofe_id.as_str().as_bytes());
        topology_bytes.push(0);
        topology_bytes.extend_from_slice(record.key.tile_id.as_str().as_bytes());
        topology_bytes.extend_from_slice(&record.tile_fraction.to_bits().to_be_bytes());
    }
    Ok(TerminalReceiverTopologyV1 {
        destination_ofe,
        receiver_destinations: records
            .iter()
            .map(|record| DirectSnowStage3V11TerminalReceiverDestinationV1 {
                destination_ofe_id: record.key.ofe_id.to_string(),
                destination_tile_id: record.key.tile_id.as_str().to_owned(),
                destination_fraction: record.tile_fraction,
            })
            .collect(),
        digest: digest_bytes(&topology_bytes),
    })
}
