pub mod boos_registry {

    static mut A: u8 = 12;

    pub struct Boss<'a> {
        pub name: &'a str,
        pub health: u32,
        pub min_hit: u32,
        pub max_hit: u32,
        pub def_chance: u32,
        pub reward: u32,
    }

    pub const FIRST_BOSS: Boss = Boss {
        name: "Boss 1",
        health: 200,
        min_hit: 10,
        max_hit: 100,
        def_chance: 20,
        reward: 10000,
    };

    pub const SECOND_BOSS: Boss = Boss {
        name: "Boss 2",
        health: 400,
        min_hit: 20,
        max_hit: 150,
        def_chance: 5,
        reward: 48000,
    };

    pub const THIRD_BOSS: Boss = Boss {
        name: "Boss 3",
        health: 600,
        min_hit: 60,
        max_hit: 260,
        def_chance: 30,
        reward: 97000,
    };

    pub const ALL_BOSSES: [Boss; 3] = [FIRST_BOSS, SECOND_BOSS, THIRD_BOSS];

    impl Boss<'_> {
        fn get_reward(&self) -> u32 {
            self.reward
        }
    }
}
