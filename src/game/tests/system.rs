use super::*;
use std::{thread, time::Duration};

#[test]
fn start_heartbeat_begins_timer_with_specified_rate() {
    let (command_sender, mut command_receiver) = channel();
    let mut timer = Timer::new(Some(command_sender));

    let game_state = GameState::new();

    let _guard = game_state.test_start_heartbeat(&mut timer);

    thread::sleep(Duration::new(1, 0));

    for _ in 0..59 {
        assert_is_next_tick(&mut command_receiver);
    }

    optional_is_next_tick(&mut command_receiver);

    assert_commands_are_empty(&mut command_receiver);
}
