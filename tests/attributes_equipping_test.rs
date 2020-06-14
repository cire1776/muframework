extern crate muframework;

#[cfg(test)]
mod test_attributes_equipping {
    use muframework::game::character::Player;

    #[test]
    fn naked_standard_player_has_0_armor_class() {
        let _player = Player::new();
    }
}
