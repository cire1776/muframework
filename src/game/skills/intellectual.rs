use super::*;

pub struct IntellectualSkill {}

impl IntellectualSkill {
    pub fn can_produce(book: &Item, player: &Player) -> bool {
        Self::book_is_mounted(book, player) && Self::player_can_read_book(book, player)
    }

    fn book_is_mounted(book: &Item, player: &Player) -> bool {
        player.mounting_points.at(&MountingPoint::OnHand) == Some(book.id)
    }

    fn player_can_read_book(book: &Item, player: &Player) -> bool {
        let level = player.get_level_for(Intellectual);
        let book_level = book
            .item_type
            .get_property_as_integer("minimum_intellectual") as u8;

        level >= book_level
    }

    pub fn expiration(_book: &Item, player: &Player) -> u32 {
        (60 + player.get_attribute(Attribute::SkillTime(Intellectual), 0)) as u32
    }

    pub fn produce_results_for(
        book: &Item,
        player: &mut Player,
        rng: &mut Rng,
        update_tx: Option<&GameUpdateSender>,
    ) {
        if player.get_level_for(Intellectual)
            <= book
                .item_type
                .get_property_as_integer("maximum_intellectual") as u8
        {
            player.increment_xp(Intellectual, 10, rng, update_tx);
        }

        if let Some(skill) = book.item_type.associated_skill() {
            if Self::book_is_in_skill_range(book, player) {
                player.increment_xp(skill, 10, rng, update_tx);
            }
        }
    }

    fn book_is_in_skill_range(book: &Item, player: &Player) -> bool {
        let skill = book.item_type.associated_skill();
        let book_min_level = book
            .item_type
            .get_property_as_integer("minimum_topic_skill") as u8;

        let book_max_level = book
            .item_type
            .get_property_as_integer("maximum_topic_skill") as u8;

        let player_level = player.get_level_for(skill.unwrap());

        player_level >= book_min_level && player_level <= book_max_level
    }
}
