mod bosses;
mod player;

use crate::player::Player::{self, PlayerInfo};

use bosses::boos_registry;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use rand::Rng;
use std::io;
use std::time::Duration;
use std::{thread, time};

use crate::bosses::boos_registry::ALL_BOSSES;

fn main() -> std::io::Result<()> {
    clear_console();
    // Utility functions
    fn clear_console() {
        // clears and put it to the first line
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        // print!("{}[2J", 27 as char); => clear the terminal too
    }

    // Creates an instance of Player struct
    let mut player = Player::PlayerInfo {
        name: String::new(),
        health: 100,
        min_hit: 10,
        max_hit: 40,
        crit_chance: 1,
        money: 0,
        deaths: 0,
        boss_kills: 0,
        current_boss: 0,
        money_earned: 0,
        highest_boss: 1,
    };

    // Prompts the user to chose a name
    println!("Please type a name for your character");
    io::stdin()
        .read_line(&mut player.name)
        .expect("Invalid name");

    let name_lenght = player.name.len();
    player.name.truncate(name_lenght - 1);

    clear_console();
    println!("\n Welcome to the game {}! \n", player.name);

    start_selection(&mut player);

    // Merchant
    fn merchant(player: &mut PlayerInfo) {
        clear_console();

        println!("Welcome stranger! \n");
        println!("Your balance is {}", player.money);
        struct item_details<'a> {
            name: &'a str,
            description: &'a str,
            price: u32,
            is_available: bool,
            is_bought: bool,
        }
        // let health_items = vec![""]
        let store_items = vec!["Health", "Firepower", "Defence", "Exit"];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&store_items)
            .default(0)
            .interact_on_opt(&Term::stderr());

        match selection {
            Ok(opt) => match opt {
                Some(index) => {
                    if index == 0 {
                        println!("Health");
                        merchant(player);
                    } else if index == 1 {
                        println!("Firepower");
                        merchant(player);
                    } else if index == 2 {
                        println!("Defence");
                        merchant(player);
                    } else if index == 3 {
                        clear_console();
                    }
                }
                None => {
                    clear_console();
                }
            },

            Err(err) => {
                eprintln!("Error in merchant items. {}", err)
            }
        }
    }

    fn health_items() {
        clear_console();
        todo!();
    }

    fn fire_power_items() {
        clear_console();
        todo!();
    }

    fn defence_items() {
        clear_console();
        todo!();
    }

    // Start Selection
    fn start_selection(player: &mut PlayerInfo) {
        clear_console();

        let items = vec![
            "Start Game",
            "Progress",
            "Merchant",
            "Player Info",
            "Bosses Info",
            "Exit",
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .items(&items)
            .default(0)
            .interact_on_opt(&Term::stderr());

        match selection {
            Ok(opt) => match opt {
                Some(index) => {
                    if index == 0 {
                        // Start Game
                        start_new_battle(player);
                    } else if index == 1 {
                        // Progress
                        println!("Story progress: {:.2}", player.get_story_progess());
                        thread::sleep(Duration::from_millis(500));

                        println!("Bosses Killed: {}", player.boss_kills);
                        thread::sleep(Duration::from_millis(500));

                        println!("Current Boss: {}", player.current_boss);
                        thread::sleep(Duration::from_millis(500));

                        println!("Total money earned: {}", player.money_earned);
                        thread::sleep(Duration::from_millis(500));

                        println!("Current balance {} \n \n", player.money);
                        thread::sleep(Duration::from_millis(100));

                        start_selection(player);
                    } else if index == 2 {
                        // Merchant
                        clear_console();
                        println!("Welcome to the Merchant \n");
                        merchant(player)
                    } else if index == 3 {
                        // Player info
                        println!("Player stats {:#?}", player);
                        thread::sleep(Duration::from_secs(1));
                    } else if index == 4 {
                        // Bosses Info
                        println!("Boss info")
                    } else {
                        println!("Till the next time!")
                    }
                }
                None => println!("User did not select anything"),
            },
            Err(err) => eprint!("This is an error {}", err),
        }
    }

    //fight simulation

    // Time between hits

    // Boss related info
    fn start_new_battle(player: &mut Player::PlayerInfo) {
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
                thread::sleep(set_delay(300));
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

    // start_new_battle(&mut player);

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

    Ok(())
}
