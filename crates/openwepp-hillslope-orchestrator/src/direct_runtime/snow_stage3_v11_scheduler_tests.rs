use std::cell::RefCell;
use std::rc::Rc;

use super::*;

fn calendar_input() -> DirectPublicationDayInput {
    DirectPublicationDayInput::calendar_only(DirectPublicationCalendarDay {
        year: 2026,
        julian_day: 1,
        month: 1,
        day_of_month: 1,
        water_year: 2026,
    })
}

fn scheduler_metadata() -> DirectPublicationRunMetadata {
    DirectPublicationRunMetadata {
        run_name: "stage3-scheduler-seam".into(),
        runtime_selection: "production-direct".into(),
        output_policy: "test".into(),
    }
}

fn make_publication_lanes_valid(frame: &mut DirectRunFrame) {
    for lane in &mut frame.lanes {
        lane.area_m2 = 1.0;
        lane.runoff_publication_efflen_m = 1.0;
        lane.runoff_publication_cumulative_length_m = 1.0;
        lane.runoff_publication_ofe_length_m = 1.0;
    }
}

#[test]
fn stage3_scheduler_prepares_once_before_two_lanes_and_commits_after_complete_day() {
    let identity = DirectRunIdentity::new(94, 501, 2, 1).expect("two-lane scheduler identity");
    let mut frame = DirectRunFrame::skeleton(identity).expect("two-lane scheduler frame");
    make_publication_lanes_valid(&mut frame);
    let events = Rc::new(RefCell::new(Vec::<String>::new()));

    DirectFrameExecutor::new(DirectExecutorMode::ProductionDirect)
        .run_publication_stream_with_stage3_day_preparation_and_commit_hook(
            &mut frame,
            scheduler_metadata(),
            {
                let events = Rc::clone(&events);
                move |_frame, day_index| {
                    events.borrow_mut().push(format!("prepare:{day_index}"));
                    Ok(())
                }
            },
            {
                let events = Rc::clone(&events);
                move |_frame, day_index, lane_index| {
                    events
                        .borrow_mut()
                        .push(format!("input:{day_index}:{lane_index}"));
                    Ok(calendar_input())
                }
            },
            {
                let events = Rc::clone(&events);
                move |_row, day_frame| {
                    events.borrow_mut().push(format!(
                        "row:{}:{}",
                        day_frame.day_index, day_frame.lane_index
                    ));
                    Ok(())
                }
            },
            {
                let events = Rc::clone(&events);
                move |hook| {
                    let event = match hook {
                        DirectPublicationDayHook::ProjectedDay {
                            lane_index, frame, ..
                        } => format!("projected:{}:{lane_index}", frame.day_index),
                        DirectPublicationDayHook::CommittedDay => "lane-committed".into(),
                        DirectPublicationDayHook::CompleteDay { day_index } => {
                            format!("complete:{day_index}")
                        }
                    };
                    events.borrow_mut().push(event);
                    Ok(())
                }
            },
            {
                let events = Rc::clone(&events);
                move |_frame, day_index, _publication_inputs| {
                    events
                        .borrow_mut()
                        .push(format!("stage3-commit:{day_index}"));
                    Ok(())
                }
            },
            |_frame, _day_index| Ok(()),
        )
        .expect("two-lane production scheduler");

    assert_eq!(
        events.borrow().as_slice(),
        [
            "prepare:0",
            "input:0:0",
            "projected:0:0",
            "lane-committed",
            "input:0:1",
            "projected:0:1",
            "lane-committed",
            "complete:0",
            "stage3-commit:0",
            "row:0:0",
            "row:0:1",
        ]
    );
}

#[test]
fn stage3_scheduler_builds_48_support_days_just_in_time_across_midnight() {
    use crate::snow_stage3_v11_attachment::{
        STAGE3_V11_DAY_NS, STAGE3_V11_PARENT_SUPPORT_COUNT, STAGE3_V11_PARENT_SUPPORT_NS,
    };

    let identity =
        DirectRunIdentity::new(95, 501, 2, 2).expect("cross-midnight scheduler identity");
    let mut frame = DirectRunFrame::skeleton(identity).expect("cross-midnight frame");
    make_publication_lanes_valid(&mut frame);
    let support_days = Rc::new(RefCell::new(Vec::<Vec<(u128, u128)>>::new()));
    let committed_days = Rc::new(RefCell::new(Vec::<usize>::new()));

    DirectFrameExecutor::new(DirectExecutorMode::ProductionDirect)
        .run_publication_stream_with_stage3_day_preparation_and_commit_hook(
            &mut frame,
            scheduler_metadata(),
            {
                let support_days = Rc::clone(&support_days);
                let committed_days = Rc::clone(&committed_days);
                move |_frame, day_index| {
                    assert_eq!(committed_days.borrow().len(), day_index);
                    let day_start = (day_index as u128)
                        .checked_mul(STAGE3_V11_DAY_NS)
                        .expect("test day start");
                    let supports = (0..STAGE3_V11_PARENT_SUPPORT_COUNT)
                        .map(|support_index| {
                            let start =
                                day_start + (support_index as u128) * STAGE3_V11_PARENT_SUPPORT_NS;
                            (start, start + STAGE3_V11_PARENT_SUPPORT_NS)
                        })
                        .collect::<Vec<_>>();
                    assert_eq!(supports.first().map(|support| support.0), Some(day_start));
                    assert_eq!(
                        supports.last().map(|support| support.1),
                        Some(day_start + STAGE3_V11_DAY_NS)
                    );
                    support_days.borrow_mut().push(supports);
                    Ok(())
                }
            },
            {
                let support_days = Rc::clone(&support_days);
                move |_frame, day_index, _lane_index| {
                    assert_eq!(support_days.borrow().len(), day_index + 1);
                    Ok(calendar_input())
                }
            },
            |_row, _day_frame| Ok(()),
            |_hook| Ok(()),
            {
                let committed_days = Rc::clone(&committed_days);
                move |_frame, day_index, _publication_inputs| {
                    committed_days.borrow_mut().push(day_index);
                    Ok(())
                }
            },
            |_frame, _day_index| Ok(()),
        )
        .expect("cross-midnight production scheduler");

    let support_days = support_days.borrow();
    assert_eq!(support_days.len(), 2);
    assert_eq!(support_days[0].len(), STAGE3_V11_PARENT_SUPPORT_COUNT);
    assert_eq!(support_days[1].len(), STAGE3_V11_PARENT_SUPPORT_COUNT);
    assert_eq!(
        support_days[0].last().map(|support| support.1),
        Some(STAGE3_V11_DAY_NS)
    );
    assert_eq!(
        support_days[1].first().map(|support| support.0),
        Some(STAGE3_V11_DAY_NS)
    );
    assert_eq!(committed_days.borrow().as_slice(), [0, 1]);
}

#[test]
fn stage3_scheduler_prepares_and_commits_every_day_of_a_complete_season_once() {
    use crate::snow_stage3_v11_attachment::{
        STAGE3_V11_DAY_NS, STAGE3_V11_PARENT_SUPPORT_COUNT, STAGE3_V11_PARENT_SUPPORT_NS,
    };

    const SEASON_DAY_COUNT: usize = 365;
    let identity = DirectRunIdentity::new(96, 501, 1, SEASON_DAY_COUNT)
        .expect("complete-season scheduler identity");
    let mut frame = DirectRunFrame::skeleton(identity).expect("complete-season frame");
    make_publication_lanes_valid(&mut frame);
    let prepared_days = Rc::new(RefCell::new(Vec::<usize>::new()));
    let committed_days = Rc::new(RefCell::new(Vec::<usize>::new()));

    DirectFrameExecutor::new(DirectExecutorMode::ProductionDirect)
        .run_publication_stream_with_stage3_day_preparation_and_commit_hook(
            &mut frame,
            scheduler_metadata(),
            {
                let prepared_days = Rc::clone(&prepared_days);
                let committed_days = Rc::clone(&committed_days);
                move |_frame, day_index| {
                    assert_eq!(prepared_days.borrow().len(), day_index);
                    assert_eq!(committed_days.borrow().len(), day_index);
                    let day_start = (day_index as u128) * STAGE3_V11_DAY_NS;
                    let day_end = day_start
                        + (STAGE3_V11_PARENT_SUPPORT_COUNT as u128) * STAGE3_V11_PARENT_SUPPORT_NS;
                    assert_eq!(day_end, day_start + STAGE3_V11_DAY_NS);
                    prepared_days.borrow_mut().push(day_index);
                    Ok(())
                }
            },
            |_frame, _day_index, _lane_index| Ok(calendar_input()),
            |_row, _day_frame| Ok(()),
            |_hook| Ok(()),
            {
                let committed_days = Rc::clone(&committed_days);
                move |_frame, day_index, _publication_inputs| {
                    assert_eq!(committed_days.borrow().len(), day_index);
                    committed_days.borrow_mut().push(day_index);
                    Ok(())
                }
            },
            |_frame, _day_index| Ok(()),
        )
        .expect("complete-season scheduler");

    assert_eq!(prepared_days.borrow().len(), SEASON_DAY_COUNT);
    assert_eq!(committed_days.borrow().len(), SEASON_DAY_COUNT);
    assert_eq!(prepared_days.borrow().first().copied(), Some(0));
    assert_eq!(prepared_days.borrow().last().copied(), Some(364));
}
