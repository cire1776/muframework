use super::*;

#[test]
fn tick_if_nothing_scheduled_sends_no_commands() {
    let (command_sender, mut command_receiver) = channel();
    let mut subject = Timer::new(Some(command_sender));

    subject.tick(1776);

    assert_commands_are_empty(&mut command_receiver);
}

#[test]
fn tick_if_a_single_command_is_scheduled_sends_that_command() {
    let (command_sender, mut command_receiver) = channel();
    let mut subject = Timer::new(Some(command_sender));

    let _guard = subject.repeating_by_tick(1776, Command::NextTick, "something");

    subject.tick(1776);

    assert_is_next_tick(&mut command_receiver);
    assert_commands_are_empty(&mut command_receiver);
}

#[test]
fn tick_if_a_multiple_commands_are_scheduled_sends_those_commands() {
    let (command_sender, mut command_receiver) = channel();
    let mut subject = Timer::new(Some(command_sender));

    let _guard1 = subject.repeating_by_tick(1776, Command::NextTick, "something");
    let _guard2 = subject.repeating_by_tick(1776, Command::NextTick, "something");

    subject.tick(1776);

    assert_is_next_tick(&mut command_receiver);
    assert_is_next_tick(&mut command_receiver);
    assert_commands_are_empty(&mut command_receiver);
}

#[test]
fn tick_reschedules_the_alarm_if_it_is_repeating() {
    let (command_sender, mut command_receiver) = channel();
    let mut subject = Timer::new(Some(command_sender));

    let _guard = subject.repeating_by_tick(1776, Command::NextTick, "something");
    subject.tick(1776);

    assert_is_next_tick(&mut command_receiver);
    assert_commands_are_empty(&mut command_receiver);

    subject.tick(1776 + 1776);

    assert_is_next_tick(&mut command_receiver);
    assert_commands_are_empty(&mut command_receiver);
}

#[test]
fn can_remove_an_alarm_with_its_tag() {
    let (command_sender, mut command_receiver) = channel();
    let mut subject = Timer::new(Some(command_sender));

    let _guard = subject.repeating_by_tick(1776, Command::NextTick, "a tag");
    subject.tick(1776);

    assert_is_next_tick(&mut command_receiver);
    assert_commands_are_empty(&mut command_receiver);

    subject.cancel("a tag");

    subject.tick(1776 + 1776);

    assert_commands_are_empty(&mut command_receiver);
}

#[test]
fn can_randomly_stagger_starting_tick_to_minimize_bunching() {
    let (command_sender, mut command_receiver) = channel();
    let mut subject = Timer::new(Some(command_sender));
    let mut rng = Rng::new();
    rng.set_test_mode();

    rng.set("timer_stagger", 1776);

    subject.stagger_repeating_by_tick(10000, Command::NextTick, "a tag", &mut rng);

    subject.tick(1776);

    assert_is_next_tick(&mut command_receiver);
    assert_commands_are_empty(&mut command_receiver);
}

#[test]
fn repeating_tags_the_alarm_for_test() {
    let (command_sender, _command_receiver) = channel();
    let mut subject = Timer::new(Some(command_sender));

    subject.repeating_by_tick(1776, Command::NextTick, "a tag");

    assert_eq!(
        subject.tags[&"a tag".to_string()],
        common::timer::TagType::Ticks(1776)
    );
}

#[test]
fn stagger_repeating_tags_the_alarm_for_test() {
    let (command_sender, _command_receiver) = channel();
    let mut subject = Timer::new(Some(command_sender));
    let mut rng = Rng::new();

    subject.stagger_repeating_by_tick(1776, Command::NextTick, "a tag", &mut rng);

    assert_eq!(
        subject.tags[&"a tag".to_string()],
        common::timer::TagType::Ticks(1776)
    );
}

#[ignore]
#[test]
fn alarm_can_be_triggered_by_tag() {
    let (command_sender, mut command_receiver) = channel();
    let mut subject = Timer::new(Some(command_sender));

    subject.repeating_by_tick(100_000, Command::NextTick, "a tag");

    subject.trigger("a tag");

    assert_is_next_tick(&mut command_receiver);
    assert_commands_are_empty(&mut command_receiver);
}

#[test]
fn dropping_guard_disable_alarm() {
    let (command_sender, mut command_receiver) = channel();
    let mut subject = Timer::new(Some(command_sender));

    // dropping guard
    subject.repeating_by_tick(100_000, Command::NextTick, "a tag");

    subject.tick(100_000);

    assert_commands_are_empty(&mut command_receiver);
}
