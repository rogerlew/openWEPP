from pathlib import Path
import re
p=Path('/workdir/openwepp-experiments/b01-wb14-cadence/native-context-restoration-reader-20260916/crates/openwepp-hillslope-orchestrator/src/v9_real_consumer_shadow_publication_retention.rs')
s=p.read_text();a=s.index('    #[test]\n    fn archived_restorer_uses_replayed_terminal_event_tail_and_rejects_wire_poisons()');b=s.index('    fn support(',a);old=s[a:b]
positive=old[:old.index('        // Day-zero')]
no_terminal=old[old.index('        // Day-zero'):old.index('        let terminal_indexes')].replace('        let final_tick = last_support.support.end_ns();\n','')
structural=old[old.index('        let mut omitted'):old.rfind('    }')].replace('let mut wrong_count = multiple_wire;','let mut wrong_count = multiple_wire.clone();')
new=positive+'''        check_no_terminal_event(&one_event_wire);
        check_terminal_event_substitutions(&multiple_wire, &multiple);
        check_structural_archive_poisons(&multiple_wire);
        let mut noncanonical = one;
        noncanonical.push(b'\\n');
        assert_archive_identity(
            restart_authority_restore_archived_publication_day_v1(&noncanonical),
            "archived publication day wire",
        );
    }

    fn check_no_terminal_event(one_event_wire: &Stage3AcceptedPublicationDayEvidenceWireV1) {
'''+no_terminal+'''    }

    fn check_terminal_event_substitutions(multiple_wire: &Stage3AcceptedPublicationDayEvidenceWireV1, multiple: &[u8]) {
        let final_tick = multiple_wire.supports.last().expect("last support").support.end_ns();
        let terminal_indexes = multiple_wire.event_handoffs.iter().enumerate()
            .filter_map(|(index,event)| (event.tick()==final_tick).then_some(index)).collect::<Vec<_>>();
        let [first_terminal, second_terminal] = terminal_indexes.as_slice() else { panic!("exactly two terminal events"); };
        let first_event = &multiple_wire.event_handoffs[*first_terminal];
        let second_event = &multiple_wire.event_handoffs[*second_terminal];
        let mut reordered = multiple_wire.clone();
        reordered.event_handoffs.swap(*first_terminal, *second_terminal);
        assert_archive_identity(restart_authority_restore_archived_publication_day_v1(&serde_json::to_vec(&reordered).expect("canonical reordered wire")), "accepted publication incremental event chronology");
        let foreign = multiple_wire.event_handoffs.iter().find(|event| event.parent_transaction_id()!=first_event.parent_transaction_id()).expect("foreign parent");
        let foreign_value = serde_json::to_value(foreign).expect("foreign event value");
        let second_value = serde_json::to_value(second_event).expect("second event value");
        let fields = [
            ("tick", (final_tick.get()+1).to_string(), "accepted publication incremental restore chronology", "accepted publication incremental restore chronology"),
            ("parent_transaction_id", foreign_value["parent_transaction_id"].as_str().expect("parent").to_owned(), "accepted publication incremental event seal", "accepted publication incremental event chronology"),
            ("begin_owner_set", second_value["begin_owner_set"].as_str().expect("beginning owner").to_owned(), "accepted publication incremental event seal", "accepted publication incremental event chronology"),
        ];
        for resealed in [false, true] {
            for (field, replacement, raw_expected, sealed_expected) in &fields {
                let wire = if resealed {
                    resealed_event_field_substitution(multiple, first_event, field, replacement)
                } else { canonical_event_field_substitution(multiple, first_event, field, replacement) };
                if resealed { assert_resealed_event_is_valid(&wire, *first_terminal); }
                println!("terminal event substitution: {field}; independently sealed={resealed}");
                assert_archive_identity(restart_authority_restore_archived_publication_day_v1(&wire), if resealed { sealed_expected } else { raw_expected });
            }
        }
    }

    fn check_structural_archive_poisons(multiple_wire: &Stage3AcceptedPublicationDayEvidenceWireV1) {
'''+structural+'    }\n\n'
s=s[:a]+new+s[b:]
s=s.replace('        const HEADER: &[u8] = b"OPENWEPP_STAGE3_COMMITTED_DAY_EVIDENCE_V1\\0";\n','')
i=s.index('    fn archived_publication_wire_v1(');s=s[:i]+'    const HEADER: &[u8] = b"OPENWEPP_STAGE3_COMMITTED_DAY_EVIDENCE_V1\\0";\n\n'+s[i:]
s=s.replace('result: Result<RestoredStage3PublicationDayEvidenceV1, DirectV11RealConsumerError>,','result: &Result<RestoredStage3PublicationDayEvidenceV1, DirectV11RealConsumerError>,').replace('assert_eq!(actual, expected);','assert_eq!(*actual, expected);')
s=re.sub(r'(assert_archive_identity\(\s*)(restart_authority_restore_archived_publication_day_v1)',r'\1&\2',s)
p.write_text(s)
