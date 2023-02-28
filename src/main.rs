mod bosses;
mod player;

use crate::player::Player::{self, PlayerInfo};

use bosses::{boss_display, boss_registry};
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use std::time::Duration;
use std::{fmt, io};
use std::{thread, time};

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
        min_hit: 30,
        max_hit: 80,
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

    // Store Items Type
    #[derive(Debug)]
    struct ItemDetails<'a> {
        name: &'a str,
        description: &'a str,
        price: u32,
        is_available: bool,
        is_bought: bool,
        unlock_requirement: &'a str,
    }

    impl fmt::Display for ItemDetails<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}          Price:${}\n({})\n\n available:{}    acquired:{}\n\n\n",
                self.name, self.price, self.description, self.is_available, self.is_bought
            )
        }
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
                        clear_console();
                        println!("Returning to Menu in 10s \n");
                        println!("Player stats {:#?}", player);
                        thread::sleep(Duration::from_secs(10));
                        clear_console();
                        start_selection(player);
                    } else if index == 4 {
                        // Bosses Info
                        clear_console();
                        boss_display::display_list();
                    } else {
                        println!("Till the next time!")
                    }
                }
                None => println!("User did not select anything"),
            },
            Err(err) => eprint!("This is an error {}", err),
        }
    }

    // Merchant
    fn merchant(player: &mut PlayerInfo) {
        clear_console();

        println!("Welcome stranger! \n");
        println!("Your balance is {}", player.money);

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
                        health_items(player)
                    } else if index == 1 {
                        fire_power_items(player);
                    } else if index == 2 {
                        defence_items(player);
                    } else if index == 3 {
                        clear_console();
                        start_selection(player);
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

    fn health_items(player: &mut PlayerInfo) {
        clear_console();
        println!("Health Items \n");
        println!("Press {} to ruturn \n", String::from("esc"));

        let health_items_vec = vec![
            ItemDetails {
                name: "HP Upgrade 1",
                description: "Increase your health by 100 points",
                is_available: true,
                unlock_requirement: "",
                is_bought: false,
                price: 100,
            },
            ItemDetails {
                name: "HP Upgrade 2",
                description: "Increase your health by 200 points",
                is_available: false,
                unlock_requirement: "You need to get to the third boss",
                is_bought: false,
                price: 100,
            },
            ItemDetails {
                name: "HP Upgrade 3",
                description: "Increase your health by 200 points",
                is_available: false,
                unlock_requirement: "You need to get to the fourth boss",
                is_bought: false,
                price: 300,
            },
        ];

        let health_selection = Select::with_theme(&ColorfulTheme::default())
            .items(&health_items_vec)
            .default(0)
            .interact_on_opt(&Term::stderr());

        match health_selection {
            Ok(opt) => match opt {
                Some(index) => {
                    if index == 0 {
                        if health_items_vec[1].is_available {
                            let old_health = player.health;
                            player.health += 100;
                            player.money -= health_items_vec[1].price;

                            println!(
                                "You upgraded your health from {} to {} for ${}",
                                old_health, player.health, health_items_vec[1].price
                            );
                        } else {
                            println!(
                                "Item is not available, {}",
                                health_items_vec[1].unlock_requirement
                            );
                        }

                        thread::sleep(Duration::from_millis(500));
                        health_items(player);
                    } else if index == 1 {
                        if health_items_vec[1].is_available {
                        } else {
                            println!("Returning...\n");
                            println!(
                                "Item is not available, {}",
                                health_items_vec[1].unlock_requirement
                            );
                        }

                        thread::sleep(Duration::from_millis(4500));
                        health_items(player);
                        println!("Firepower");
                    } else {
                        merchant(player);
                    }
                }
                None => {
                    merchant(player);
                }
            },
            Err(err) => {
                eprintln!("{}", err)
            }
        }
    }

    fn fire_power_items(player: &mut PlayerInfo) {
        clear_console();
        println!("Health Items \n");
        println!("Press {} to ruturn \n", String::from("esc"));

        let health_items_vec = vec![
            ItemDetails {
                name: "HP Upgrade 1",
                description: "Increase your health by 100 points",
                is_available: true,
                unlock_requirement: "",
                is_bought: false,
                price: 100,
            },
            ItemDetails {
                name: "HP Upgrade 2",
                description: "Increase your health by 200 points",
                is_available: false,
                unlock_requirement: "You need to get to the third boss",
                is_bought: false,
                price: 100,
            },
            ItemDetails {
                name: "HP Upgrade 3",
                description: "Increase your health by 200 points",
                is_available: false,
                unlock_requirement: "You need to get to the fourth boss",
                is_bought: false,
                price: 300,
            },
        ];

        let health_selection = Select::with_theme(&ColorfulTheme::default())
            .items(&health_items_vec)
            .default(0)
            .interact_on_opt(&Term::stderr());

        match health_selection {
            Ok(opt) => match opt {
                Some(index) => {
                    if index == 0 {
                        if health_items_vec[1].is_available {
                            let old_health = player.health;
                            player.health += 100;
                            player.money -= health_items_vec[1].price;

                            println!(
                                "You upgraded your health from {} to {} for ${}",
                                old_health, player.health, health_items_vec[1].price
                            );
                        } else {
                            println!(
                                "Item is not available, {}",
                                health_items_vec[1].unlock_requirement
                            );
                        }

                        thread::sleep(Duration::from_millis(500));
                        health_items(player);
                    } else if index == 1 {
                        if health_items_vec[1].is_available {
                        } else {
                            println!("Returning...\n");
                            println!(
                                "Item is not available, {}",
                                health_items_vec[1].unlock_requirement
                            );
                        }

                        thread::sleep(Duration::from_millis(4500));
                        health_items(player);
                        println!("Firepower");
                    } else {
                        merchant(player);
                    }
                }
                None => {
                    merchant(player);
                }
            },
            Err(err) => {
                eprintln!("{}", err)
            }
        }
    }

    fn defence_items(player: &mut PlayerInfo) {
        clear_console();
        println!("Health Items \n");
        println!("Press {} to ruturn \n", String::from("esc"));

        let health_items_vec = vec![
            ItemDetails {
                name: "HP Upgrade 1",
                description: "Increase your health by 100 points",
                is_available: true,
                unlock_requirement: "",
                is_bought: false,
                price: 100,
            },
            ItemDetails {
                name: "HP Upgrade 2",
                description: "Increase your health by 200 points",
                is_available: false,
                unlock_requirement: "You need to get to the third boss",
                is_bought: false,
                price: 100,
            },
            ItemDetails {
                name: "HP Upgrade 3",
                description: "Increase your health by 200 points",
                is_available: false,
                unlock_requirement: "You need to get to the fourth boss",
                is_bought: false,
                price: 300,
            },
        ];

        let health_selection = Select::with_theme(&ColorfulTheme::default())
            .items(&health_items_vec)
            .default(0)
            .interact_on_opt(&Term::stderr());

        match health_selection {
            Ok(opt) => match opt {
                Some(index) => {
                    if index == 0 {
                        if health_items_vec[1].is_available {
                            let old_health = player.health;
                            player.health += 100;
                            player.money -= health_items_vec[1].price;

                            println!(
                                "You upgraded your health from {} to {} for ${}",
                                old_health, player.health, health_items_vec[1].price
                            );
                        } else {
                            println!(
                                "Item is not available, {}",
                                health_items_vec[1].unlock_requirement
                            );
                        }

                        thread::sleep(Duration::from_millis(500));
                        health_items(player);
                    } else if index == 1 {
                        if health_items_vec[1].is_available {
                        } else {
                            println!("Returning...\n");
                            println!(
                                "Item is not available, {}",
                                health_items_vec[1].unlock_requirement
                            );
                        }

                        thread::sleep(Duration::from_millis(4500));
                        health_items(player);
                        println!("Firepower");
                    } else {
                        merchant(player);
                    }
                }
                None => {
                    merchant(player);
                }
            },
            Err(err) => {
                eprintln!("{}", err)
            }
        }
    }

    // Battle
    fn start_new_battle(player: &mut PlayerInfo) {
        let boss_array = boss_registry::ALL_BOSSES;

        // Delay Amounts
        fn set_delay(mil_sec: u64) -> Duration {
            time::Duration::from_millis(mil_sec)
        }
        let delay = time::Duration::from_secs(2);

        // Helpers
        let mut current_boss = player.current_boss;
        let mut players_health = player.health;
        let mut boss_health = boss_array[player.current_boss as usize].health; // not safe?

        // Misc.
        let mut player_hit_counter = 0;
        let mut boss_hit_counter = 0;

        println!(
            "Current boss: {} \n",
            boss_array[player.current_boss as usize].name
        );
        thread::sleep(Duration::from_millis(600));

        loop {
            thread::sleep(set_delay(1000));

            let player_hit_damage = player.hit();
            let boss_hit_damage = boss_array[current_boss as usize].hit();

            let boss_reward: u32 = boss_array[player.current_boss as usize].reward;

            // Update highest boss achieved
            if player.highest_boss < player.current_boss {
                player.highest_boss = player.current_boss;
            };

            println!("Player hit: {player_hit_damage}");
            thread::sleep(set_delay(500));

            if boss_array[current_boss as usize].has_defended() {
                println!("Boss defended a {} hit \n", player_hit_damage);
                thread::sleep(delay);

                if players_health > boss_hit_damage {
                    println!("Boss hit: {boss_hit_damage}");
                    thread::sleep(set_delay(500));
                    players_health -= boss_hit_damage;
                    boss_hit_counter += 1;

                    println!("Player has now {players_health}hp \n");
                    thread::sleep(delay);
                } else {
                    break {
                        current_boss = 0; // If the play dies, it goes back to the first boss;
                        println!("Player is now dead! Killed with {} hits", boss_hit_counter);
                        println!("Back to the start! \n",);

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
                                        clear_console();
                                        start_new_battle(player)
                                    } else if val == 1 {
                                        clear_console();
                                        merchant(player)
                                    } else {
                                        clear_console();
                                        start_selection(player)
                                    }
                                }
                                None => {}
                            },
                            Err(err) => eprintln!("Erro {err:?}"),
                        }
                    };
                }
            } else {
                if boss_health > player_hit_damage {
                    boss_health -= player_hit_damage;
                    player_hit_counter += 1;
                    println!("Boss has now {boss_health}hp \n");
                    thread::sleep(delay);

                    if players_health > boss_hit_damage {
                        println!("Boss hit {boss_hit_damage}hp");
                        players_health -= boss_hit_damage;
                        boss_hit_counter += 1;
                        thread::sleep(set_delay(500));

                        println!("Player has now {players_health}hp \n");
                        thread::sleep(delay);
                    } else {
                        break {
                            println!("Boss hit {boss_hit_damage}hp");
                            thread::sleep(set_delay(500));
                            current_boss = 0; // If the play dies, it goes back to the first boss;
                            println!(
                                "Player is now dead! Killed with {} hits \n",
                                boss_hit_counter
                            );
                            println!("Back to the start!",);

                            thread::sleep(set_delay(3000));
                            let options = vec!["Next Battle", "Vendor", "Exit"];

                            let next_options = Select::with_theme(&ColorfulTheme::default())
                                .items(&options)
                                .default(0)
                                .interact_on_opt(&Term::stderr());

                            match next_options {
                                Ok(opt) => match opt {
                                    Some(val) => {
                                        if val == 0 {
                                            clear_console();
                                            start_new_battle(player)
                                        } else if val == 1 {
                                            clear_console();
                                            merchant(player)
                                        } else {
                                            clear_console();
                                            start_selection(player)
                                        }
                                    }
                                    None => {}
                                },
                                Err(err) => eprintln!("Erro {err:?}"),
                            }
                        };
                    }
                } else {
                    // Boss death
                    player.money += boss_reward;
                    player.money_earned += boss_reward;
                    player.current_boss += 1;

                    println!(
                        "Boss is now dead! Killed with {} hits. Player balance is now {:.2} \n",
                        player_hit_counter, player.money
                    );

                    thread::sleep(set_delay(300));

                    println!(
                        "Next boss is now {}.",
                        boss_array[player.current_boss as usize].name
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
                                    clear_console();
                                    start_new_battle(player)
                                } else if val == 1 {
                                    clear_console();
                                    merchant(player)
                                } else {
                                    clear_console();
                                }
                            }
                            None => {}
                        },
                        Err(err) => eprintln!("Erro {err:?}"),
                    }
                }
            }
        }
    }

    Ok(())
}
