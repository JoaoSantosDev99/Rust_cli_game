mod bosses;
mod merchant;
mod player;

use crate::player::player::PlayerInfo;

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
    let mut player = player::player::PlayerInfo {
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
                        merchant_selection(player)
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

    // ---------------------  Merchant  ------------------------------------------------------
    // Store Items Type
    #[derive(Debug)]
    struct ItemDetails<'a> {
        name: &'a str,
        description: &'a str,
        price: u32,
        increase: u32,
    }

    impl fmt::Display for ItemDetails<'_> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{}          Price:${}\n({})\n\n\n",
                self.name, self.price, self.description
            )
        }
    }

    fn merchant_selection(player: &mut PlayerInfo) {
        clear_console();

        println!("Welcome stranger! \n");
        println!("Your balance is {}", player.money);

        // let health_items = vec![""]
        let store_items = vec!["Health", "Firepower", "Crit Chance", "Exit"];

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
                        crit_chance_items(player);
                    } else if index == 3 {
                        clear_console();
                        start_selection(player);
                    }
                }
                None => {
                    clear_console();
                    start_selection(player);
                }
            },

            Err(err) => {
                eprintln!("Error in merchant_selection items. {}", err)
            }
        }
    }

    fn health_items(player: &mut PlayerInfo) {
        clear_console();
        println!("Health Items \n");
        println!("Press {} to ruturn \n", String::from("esc"));

        let health_items_vec = vec![
            ItemDetails {
                name: "Health Upgrade 1",
                description: "Increase your health by 100 points",
                price: 100,
                increase: 100,
            },
            ItemDetails {
                name: "Health Upgrade 2",
                description: "Increase your health by 200 points",
                price: 150,
                increase: 200,
            },
            ItemDetails {
                name: "Health Upgrade 3",
                description: "Increase your health by 500 points",
                price: 300,
                increase: 500,
            },
        ];

        let health_selection = Select::with_theme(&ColorfulTheme::default())
            .items(&health_items_vec)
            .default(0)
            .interact_on_opt(&Term::stderr());

        match health_selection {
            Ok(opt) => match opt {
                Some(index) => {
                    if player.money < health_items_vec[index].price {
                        println!("Not enought cash, stranger!");
                        thread::sleep(Duration::from_millis(3000));
                        health_items(player);
                    };

                    let old_health = player.health;
                    player.health += health_items_vec[index].increase;
                    player.money -= health_items_vec[index].price;

                    println!(
                        "You upgraded your health from {} to {} for ${}",
                        old_health, player.health, health_items_vec[index].price
                    );

                    thread::sleep(Duration::from_millis(3000));
                    health_items(player);
                }
                None => {
                    merchant_selection(player);
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

        let damage_items_vec = vec![
            ItemDetails {
                name: "Damage Upgrade 1",
                description: "Increases your min and max Damage by 10 points",
                price: 200,
                increase: 10,
            },
            ItemDetails {
                name: "Damage Upgrade 2",
                description: "Increases your min and max Damage by 15 points",
                price: 300,
                increase: 15,
            },
            ItemDetails {
                name: "Damage Upgrade 3",
                description: "Increases your min and max Damage by 25 points",
                price: 500,
                increase: 25,
            },
        ];

        let damage_selection = Select::with_theme(&ColorfulTheme::default())
            .items(&damage_items_vec)
            .default(0)
            .interact_on_opt(&Term::stderr());

        match damage_selection {
            Ok(opt) => match opt {
                Some(index) => {
                    if player.money < damage_items_vec[index].price {
                        println!("Not enought cash, stranger!");
                        thread::sleep(Duration::from_millis(3000));

                        health_items(player);
                    };

                    let old_min = player.min_hit;
                    let old_max = player.max_hit;
                    player.min_hit += damage_items_vec[index].increase;
                    player.max_hit += damage_items_vec[index].increase;

                    player.money -= damage_items_vec[index].price;

                    println!(
                        "You upgraded your damage from {}-{} to {}-{} for ${}",
                        old_min,
                        old_max,
                        player.min_hit,
                        player.max_hit,
                        damage_items_vec[index].price
                    );

                    thread::sleep(Duration::from_millis(3000));

                    fire_power_items(player);
                }
                None => {
                    merchant_selection(player);
                }
            },
            Err(err) => {
                eprintln!("{}", err)
            }
        }
    }

    fn crit_chance_items(player: &mut PlayerInfo) {
        clear_console();
        println!("Crit chance Boosters");
        println!("Can go up to a maximum of 40% crit chance \n");
        println!("Press {} to ruturn \n", String::from("esc"));

        let crit_chance_items_vec = vec![
            ItemDetails {
                name: "Crit Chance Upgrade 1",
                description: "Increases your crit chance by 2%",
                price: 500,
                increase: 2,
            },
            ItemDetails {
                name: "Crit Chance Upgrade 2",
                description: "Increases your crit chance by 4%",
                price: 800,
                increase: 4,
            },
            ItemDetails {
                name: "Crit Chance Upgrade 3",
                description: "Increases your crit chance by 7%",
                price: 1500,
                increase: 7,
            },
        ];

        let crit_chance_selection = Select::with_theme(&ColorfulTheme::default())
            .items(&crit_chance_items_vec)
            .default(0)
            .interact_on_opt(&Term::stderr());

        match crit_chance_selection {
            Ok(opt) => match opt {
                Some(index) => {
                    if player.money < crit_chance_items_vec[index].price {
                        println!("Not enought cash, stranger!");
                        thread::sleep(Duration::from_millis(3000));

                        health_items(player);
                    };

                    let old_crit_chance = player.crit_chance;
                    player.crit_chance += crit_chance_items_vec[index].increase;

                    // Crit cahnce cap
                    if player.crit_chance > 40 {
                        player.crit_chance = 40
                    }

                    player.money -= crit_chance_items_vec[index].price;

                    println!(
                        "You upgraded your crit chance from {} to {} for ${}",
                        old_crit_chance, player.crit_chance, crit_chance_items_vec[index].price
                    );

                    thread::sleep(Duration::from_millis(3000));

                    crit_chance_items(player);
                }
                None => {
                    merchant_selection(player);
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
            let boss_hit_damage = boss_array[player.current_boss as usize].hit();

            let boss_reward: u32 = boss_array[player.current_boss as usize].reward;

            // Update highest boss achieved
            if player.highest_boss < player.current_boss {
                player.highest_boss = player.current_boss;
            };

            println!("Player hit: {player_hit_damage}");
            thread::sleep(set_delay(500));

            if boss_array[player.current_boss as usize].has_defended() {
                player_hit_counter += 1;
                println!("Boss defended!\n");
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
                        boss_hit_counter += 1;
                        player.current_boss = 0; // If the play dies, it goes back to the first boss;
                        player.deaths += 1;
                        println!(
                            "\n \nPlayer is now dead! Killed with {} hits",
                            boss_hit_counter
                        );
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
                                        merchant_selection(player)
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
                            boss_hit_counter += 1;
                            thread::sleep(set_delay(500));
                            player.current_boss = 0; // If the play dies, it goes back to the first boss;
                            player.deaths += 1;

                            println!(
                                "\n \nPlayer is now dead! Killed with {} hits",
                                boss_hit_counter
                            );
                            println!("Back to the start! \n",);

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
                                            merchant_selection(player)
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
                    player_hit_counter += 1;
                    player.money += boss_reward;
                    player.money_earned += boss_reward;
                    player.current_boss += 1;

                    println!(
                        "\n \nBoss is now dead! Killed with {} hits. Player balance is now {:.2}",
                        player_hit_counter, player.money
                    );

                    thread::sleep(set_delay(300));

                    println!(
                        "Next boss is now {}. \n",
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
                                    merchant_selection(player)
                                } else {
                                    clear_console();
                                    start_selection(player)
                                }
                            }
                            None => start_selection(player),
                        },
                        Err(err) => eprintln!("Erro {err:?}"),
                    }
                }
            }
        }
    }

    Ok(())
}
