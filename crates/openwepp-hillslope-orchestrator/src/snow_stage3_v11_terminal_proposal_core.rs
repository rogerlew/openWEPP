fn terminal_group_topology_set_digest(
    lane_topologies: &[(u32, Digest32)],
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    if lane_topologies.is_empty() {
        return Err(DirectSnowStage3V11AttachmentError::Identity(
            "terminal group topology set empty",
        ));
    }
    let mut members = Vec::with_capacity(lane_topologies.len());
    let mut previous_lane = None;
    for (lane_id, topology) in lane_topologies {
        if previous_lane.is_some_and(|previous| previous >= *lane_id) {
            return Err(DirectSnowStage3V11AttachmentError::Identity(
                "terminal group topology set order",
            ));
        }
        previous_lane = Some(*lane_id);
        let mut member = Vec::with_capacity(36);
        member.extend_from_slice(&lane_id.to_be_bytes());
        member.extend_from_slice(topology.as_bytes());
        members.push(member);
    }
    let fields = members
        .iter()
        .map(|member| FramedField {
            tag: "lane_topology",
            value: member,
        })
        .collect::<Vec<_>>();
    Ok(framed_sha256(
        "stage3-v11-terminal-event-group-topology-set-v1",
        &fields,
    )?)
}

fn terminal_event_proposal_core(
    configuration: &DirectSurfaceLiquidConfiguration,
    group: &Stage3V11TerminalEventGroupV1,
    parent_transaction_id: Digest32,
    enclosing_support: TimeSupport,
    physical_child_ordinal: u32,
) -> Result<Digest32, DirectSnowStage3V11AttachmentError> {
    let lane_topologies = group
        .candidates
        .iter()
        .map(|candidate| {
            Ok((
                candidate.lane_id,
                terminal_receiver_topology(configuration, candidate.lane_id)?.digest,
            ))
        })
        .collect::<Result<Vec<_>, DirectSnowStage3V11AttachmentError>>()?;
    let topology_set = terminal_group_topology_set_digest(&lane_topologies)?;
    let schema = 1_u32.to_be_bytes();
    let child = physical_child_ordinal.to_be_bytes();
    let ordinal = u32::try_from(group.ordinal)
        .map_err(|_| DirectSnowStage3V11AttachmentError::Identity("terminal event ordinal width"))?
        .to_be_bytes();
    let mut candidates = u32::try_from(group.candidates.len())
        .map_err(|_| DirectSnowStage3V11AttachmentError::Identity("terminal candidate count"))?
        .to_be_bytes()
        .to_vec();
    for value in &group.candidates {
        let mut member = Vec::new();
        member.extend_from_slice(&value.lane_id.to_be_bytes());
        member.extend_from_slice(value.event_result_digest.as_bytes());
        member.extend_from_slice(value.terminal_state_sha256.as_bytes());
        member.extend_from_slice(&value.event.terminal_liquid_kg_m2.to_bits().to_be_bytes());
        member.extend_from_slice(
            &value
                .event
                .terminal_unallocated_energy_j_m2
                .to_bits()
                .to_be_bytes(),
        );
        candidates.extend_from_slice(&(member.len() as u32).to_be_bytes());
        candidates.extend_from_slice(&member);
    }
    let search = group
        .candidates
        .first()
        .ok_or(DirectSnowStage3V11AttachmentError::Identity(
            "terminal proposal-core candidates",
        ))?
        .support;
    let proposal_core = framed_sha256(
        "stage3-v11-terminal-event-proposal-core",
        &[
            FramedField {
                tag: "schema",
                value: &schema,
            },
            FramedField {
                tag: "parent_transaction",
                value: parent_transaction_id.as_bytes(),
            },
            FramedField {
                tag: "enclosing_start",
                value: &enclosing_support.start_ns().get().to_be_bytes(),
            },
            FramedField {
                tag: "enclosing_end",
                value: &enclosing_support.end_ns().get().to_be_bytes(),
            },
            FramedField {
                tag: "search_start",
                value: &search.start_ns().get().to_be_bytes(),
            },
            FramedField {
                tag: "search_end",
                value: &search.end_ns().get().to_be_bytes(),
            },
            FramedField {
                tag: "event_tick",
                value: &group.tick.get().to_be_bytes(),
            },
            FramedField {
                tag: "child_ordinal",
                value: &child,
            },
            FramedField {
                tag: "event_ordinal",
                value: &ordinal,
            },
            FramedField {
                tag: "forcing",
                value: group.candidates[0].shortened_forcing_sha256.as_bytes(),
            },
            FramedField {
                tag: "topology",
                value: topology_set.as_bytes(),
            },
            FramedField {
                tag: "candidates",
                value: &candidates,
            },
        ],
    )?;
    Ok(proposal_core)
}

#[cfg(test)]
mod terminal_proposal_core_regression_tests {
    use super::*;

    #[test]
    fn distinct_lane_receiver_topologies_form_one_ordered_group_identity() {
        let lane_topologies = [
            (1, digest_bytes(b"ofe-1-terminal-receiver-topology")),
            (2, digest_bytes(b"ofe-2-terminal-receiver-topology")),
            (3, digest_bytes(b"ofe-3-terminal-receiver-topology")),
        ];
        let group = terminal_group_topology_set_digest(&lane_topologies)
            .expect("three-OFE terminal group topology");
        assert!(
            lane_topologies
                .iter()
                .all(|(_, topology)| *topology != group)
        );
        assert_eq!(
            terminal_group_topology_set_digest(&lane_topologies)
                .expect("deterministic three-OFE terminal group topology"),
            group,
        );

        let mut topology_poison = lane_topologies;
        topology_poison[1].1 = digest_bytes(b"ofe-2-substituted-terminal-receiver-topology");
        assert_ne!(
            terminal_group_topology_set_digest(&topology_poison)
                .expect("topology substitution remains a well-formed set"),
            group,
        );

        let mut order_poison = lane_topologies;
        order_poison.swap(0, 1);
        assert!(terminal_group_topology_set_digest(&order_poison).is_err());
        let duplicate_poison = [lane_topologies[0], lane_topologies[0]];
        assert!(terminal_group_topology_set_digest(&duplicate_poison).is_err());
    }
}
