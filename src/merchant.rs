pub mod Merchant {

    pub fn clear_console() {
        // clears and put it to the first line
        print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
        // print!("{}[2J", 27 as char); => clear the terminal too
    }

    pub fn main_selection(player: &mut PlayerInfo) {
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

    pub fn health_items(player: &mut PlayerInfo) {
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

    pub fn fire_power_items(player: &mut PlayerInfo) {
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

    pub fn defence_items(player: &mut PlayerInfo) {
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
}
