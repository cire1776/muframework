use super::*;

#[test]
fn if_roll_fails_doesnt_advance_to_level_2() {
    let mut rng = Rng::new();
    rng.set_test_mode();
    rng.set_fail("levelling check");

    let mut player = Player::new();

    player.set_level_for(Cooking, 1, None);

    player.increment_xp(Cooking, 5, &mut rng, None);

    assert_eq!(player.get_level_for(Cooking), 1);
}

#[test]
fn if_roll_succeed_advances_to_level_2() {
    let mut rng = Rng::new();
    rng.set_test_mode();
    rng.set_succeed("levelling check");

    let mut player = Player::new();

    player.set_level_for(Cooking, 1, None);

    player.increment_xp(Cooking, 5, &mut rng, None);

    assert_eq!(player.get_level_for(Cooking), 2);
}

#[test]
fn at_level_45_success_does_not_advance_level() {
    let mut rng = Rng::new();
    rng.set_test_mode();
    rng.set_succeed("levelling check");

    let mut player = Player::new();

    player.set_level_for(Cooking, 45, None);

    player.increment_xp(Cooking, 5, &mut rng, None);

    assert_eq!(player.get_level_for(Cooking), 45);
}

#[test]
fn at_level_0_success_does_not_advance_level() {
    let mut rng = Rng::new();
    rng.set_test_mode();
    rng.set_succeed("levelling check");

    let mut player = Player::new();

    player.set_level_for(Cooking, 0, None);

    player.increment_xp(Cooking, 5, &mut rng, None);

    assert_eq!(player.get_level_for(Cooking), 0);
}
