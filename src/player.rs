use crate::bosses::boss_registry;
use rand::Rng;

pub mod Player {
    use rand::Rng;

    use crate::bosses::boss_registry;

    #[derive(Debug)]
    pub struct PlayerInfo {
        // Core
        pub name: String,
        pub health: u32,
        pub min_hit: u32,
        pub max_hit: u32,
        pub crit_chance: u32, // not safe yet
        pub money: u32,
        pub current_boss: u32,
        pub highest_boss: u32,

        // Trivia
        pub boss_kills: u32,
        pub money_earned: u32,
        pub deaths: u32,
    }

    impl PlayerInfo {
        pub fn get_story_progess(&self) -> f32 {
            let progress = (self.highest_boss / boss_registry::ALL_BOSSES.len() as u32) as f32;
            progress
        }

        pub fn hit(&self) -> u32 {
            fn crit_calc(player: &PlayerInfo) -> bool {
                let mut rng = rand::thread_rng();
                let number: u32 = rng.gen_range(1..=100);

                if number <= player.crit_chance {
                    true
                } else {
                    false
                }
            }

            let mut rng = rand::thread_rng();
            let rand_num: u32 = rng.gen();

            let hit_calc = rand_num % (self.max_hit + 1);
            let hit_damage = if hit_calc < self.min_hit {
                self.min_hit
            } else {
                hit_calc
            };

            let crit_status = crit_calc(&self);

            if crit_status {
                hit_damage * 2
            } else {
                hit_damage
            }
        }
    }
}
