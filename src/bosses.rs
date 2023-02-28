pub mod boos_registry {
    use rand::Rng;

    pub struct Boss<'a> {
        pub name: &'a str,
        pub health: u32,
        pub min_hit: u32,
        pub max_hit: u32,
        pub def_chance: u32,
        pub reward: u32,
    }

    impl Boss<'_> {}

    pub const FIRST_BOSS: Boss = Boss {
        name: "Jesse James",
        health: 100,
        min_hit: 5,
        max_hit: 25,
        def_chance: 20,
        reward: 10000,
    };

    pub const SECOND_BOSS: Boss = Boss {
        name: "Billy the Kid",
        health: 400,
        min_hit: 20,
        max_hit: 150,
        def_chance: 5,
        reward: 48000,
    };

    pub const THIRD_BOSS: Boss = Boss {
        name: "Buffalo Bill",
        health: 600,
        min_hit: 60,
        max_hit: 260,
        def_chance: 30,
        reward: 97000,
    };

    pub const ALL_BOSSES: [Boss; 3] = [FIRST_BOSS, SECOND_BOSS, THIRD_BOSS];

    impl Boss<'_> {
        pub fn hit(&self) -> u32 {
            let mut rng = rand::thread_rng();

            //return
            rng.gen_range(self.min_hit..=self.max_hit)
        }

        pub fn has_defended(&self) -> bool {
            let mut rng = rand::thread_rng();
            let x: u32 = rng.gen_range(1..=100);

            //return
            let defended = if x <= self.def_chance { true } else { false };
            defended
        }
    }
}
