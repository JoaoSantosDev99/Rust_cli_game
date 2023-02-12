mod bosses;
mod char_info;
use bosses::boos_registry;
use char_info::getters;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use rand::Rng;
use std::io;
use std::time::Duration;
use std::{thread, time};

fn main() -> std::io::Result<()> {
    // Player info
    #[derive(Debug)]
    struct Player {
        name: String,
        health: u32,
        min_hit: u32,
        max_hit: u32,
        crit_chance: u32, // unsafe
        money: u32,
        deaths: u32,
    }

    let mut player = Player {
        name: String::new(),
        health: 100,
        min_hit: 10,
        max_hit: 40,
        crit_chance: 1,
        money: 0,
        deaths: 0,
    };

    impl Player {
        fn hit(&self) -> u32 {
            fn crit_calc(player: &Player) -> bool {
                let mut rng = rand::thread_rng();
                let x: u32 = rng.gen();
                let number = x % 100;

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

    // println!("Please type a name for your character");
    // io::stdin()
    //     .read_line(&mut player.name)
    //     .expect("Invalid name");
    // println!("Your name is {}", player.name);
    // println!("Player stats {:#?}", player);

    // let items = vec!["Item 1", "item 2", "item 3"];
    // let selection = Select::with_theme(&ColorfulTheme::default())
    //     .items(&items)
    //     .default(0)
    //     .interact_on_opt(&Term::stderr())?;

    // match selection {
    //     Some(index) => {
    //         if index == 0 {
    //             println!("primeira")
    //         } else if index == 1 {
    //             println!("sec")
    //         } else {
    //             println!("ter")
    //         }
    //     }
    //     None => println!("User did not select anything"),
    // }

    //fight simulation

    // Time between hits

    // Boss related info
    fn start_new_battle(player: &mut Player) {
        // Delay Amounts
        fn set_delay(mil_sec: u64) -> Duration {
            time::Duration::from_millis(mil_sec)
        }
        let delay = time::Duration::from_secs(2);

        let mut current_boss = 0;
        let mut players_health = 170;
        let mut boss_helth = 250;
        let mut player_hit_counter = 0;
        let mut boss_hit_counter = 0;
        let mut player_balance: f64;

        // Battle
        loop {
            let boss_array = boos_registry::ALL_BOSSES;

            let player_hit_damage = player_hit();
            // let boss_hit_damage = boss_hit()[0];
            let boss_hit_damage = boss_hit();
            let boss_reward: u32 = 10000;

            println!("Player hit: {player_hit_damage}");
            thread::sleep(set_delay(500));
            if boss_helth > player_hit_damage {
                player_hit_counter += 1;

                if !has_boss_defended(20) {
                    boss_helth -= player_hit_damage;
                    println!("Boss has now {boss_helth}hp \n");
                    thread::sleep(delay);
                } else {
                    println!("Boss defended a {} git \n", player_hit_damage);
                    thread::sleep(delay);
                }

                println!("Boss hit: {boss_hit_damage}");
                thread::sleep(set_delay(500));
                if players_health > boss_hit_damage {
                    players_health -= boss_hit_damage;
                    boss_hit_counter += 1;

                    println!("Player has now {players_health}hp \n");
                    thread::sleep(delay);
                } else {
                    break {
                        current_boss += 0; // If the play dies, it goes back to the first boss;
                        println!(
                            "Player is now dead! Killed with {} hits \n",
                            boss_hit_counter
                        );
                        thread::sleep(set_delay(3000));
                        let options = vec!["Next Boss", "Vendor", "Exit"];
                        let next_options = Select::with_theme(&ColorfulTheme::default())
                            .items(&options)
                            .default(0)
                            .interact_on_opt(&Term::stderr());

                        match next_options {
                            Ok(opt) => match opt {
                                Some(val) => {
                                    if val == 0 {
                                        println!("you continued")
                                    } else if val == 1 {
                                        println!("vendor")
                                    } else {
                                        println!("Game over")
                                    }
                                }
                                None => {}
                            },
                            Err(err) => eprintln!("Erro {err:?}"),
                        }
                    };
                }
            } else {
                player_balance = boss_reward as f64 / player_hit_counter as f64;

                println!(
                    "Boss is now dead! Killed with {} hits. Player balance is now {:.2} \n",
                    player_hit_counter, player_balance
                );
                thread::sleep(set_delay(3));
                current_boss += 1;
                println!(
                    "Next boss is now {}. Name: {}",
                    current_boss, boss_array[0].name
                );

                let options = vec!["Next Boss", "Vendor", "Exit"];
                let next_options = Select::with_theme(&ColorfulTheme::default())
                    .items(&options)
                    .default(0)
                    .interact_on_opt(&Term::stderr());

                match next_options {
                    Ok(opt) => match opt {
                        Some(val) => {
                            if val == 0 {
                                println!("you continued")
                            } else if val == 1 {
                                println!("vendor")
                            } else {
                                println!("Game over")
                            }
                        }
                        None => {}
                    },
                    Err(err) => eprintln!("Erro {err:?}"),
                }
            }
        }
    }

    start_new_battle(&mut player);

    fn player_hit() -> u32 {
        let mut rng = rand::thread_rng();
        let x: u32 = rng.gen();
        x % 60 + 10 // 10 to 60 hit points
    }

    fn boss_hit() -> u32 {
        let mut rng = rand::thread_rng();
        let x: u32 = rng.gen();
        // 0 to 50 hit points
        let hit = x % 50;
        let defended = false;
        x % 50
    }

    fn has_boss_defended(chance: u32) -> bool {
        let mut rng = rand::thread_rng();
        let total_range: u32 = rng.gen();
        let defence_number = total_range % 100;
        defence_number <= chance
    }

    // getters::get_test();

    Ok(())
}
