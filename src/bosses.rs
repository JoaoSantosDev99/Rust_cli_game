pub mod boss_registry {
    use std::fmt::{self};

    use rand::Rng;

    pub struct Boss<'a> {
        pub name: &'a str,
        pub health: u32,
        pub min_hit: u32,
        pub max_hit: u32,
        pub def_chance: u32,
        pub reward: u32,
    }

    pub const FIRST_BOSS: Boss = Boss {
        name: "Jesse James",
        health: 100,
        min_hit: 5,
        max_hit: 25,
        def_chance: 20,
        reward: 1000,
    };

    pub const SECOND_BOSS: Boss = Boss {
        name: "Billy the Kid",
        health: 400,
        min_hit: 20,
        max_hit: 150,
        def_chance: 5,
        reward: 2100,
    };

    pub const THIRD_BOSS: Boss = Boss {
        name: "Buffalo Bill",
        health: 600,
        min_hit: 60,
        max_hit: 260,
        def_chance: 40,
        reward: 100000,
    };

    pub const ALL_BOSSES: [Boss; 3] = [FIRST_BOSS, SECOND_BOSS, THIRD_BOSS];

    // Related Methods
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

    // Display
    impl fmt::Display for Boss<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, " Name: {} \n Health: {} \n Hit Range: {}-{} \n Chance of Defending: {}% \n Reward: {} coins",
            self.name, self.health, self.min_hit, self.max_hit, self.def_chance, self.reward)
        }
    }
}

pub mod boss_display {

    use super::boss_registry;
    use dialoguer::{console::Term, theme::ColorfulTheme, Select};
    use std::{thread, time::Duration};

    pub fn display_list() {
        fn clear_console() {
            // clears and put it to the first line
            print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
            print!("{}[2J", 27 as char);
        }

        clear_console();
        println!("Press {} to return", "ESC");

        let bosses = boss_registry::ALL_BOSSES;
        let mut boss_names: Vec<&str> = Vec::new();

        for boss in &bosses {
            boss_names.push(boss.name)
        }

        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&boss_names)
            .default(0)
            .interact_on_opt(&Term::stderr());

        match selection {
            Ok(opt) => match opt {
                Some(index) => {
                    println!("Returning in 10s \n");
                    println!("{}", bosses[index]);
                    thread::sleep(Duration::from_secs(10));
                    clear_console();
                    display_list()
                }
                None => display_list(),
            },
            Err(error) => eprint!("This is an error {}", error),
        }
    }
}
