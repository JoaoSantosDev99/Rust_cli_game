// pub mod merchantSelection {
//     use crate::player::Player::PlayerInfo;
//     use dialoguer::{console::Term, theme::ColorfulTheme, Select};
//     use std::{fmt, thread, time::Duration};

//     fn clear_console() {
//         print!("{esc}[2J{esc}[1;1H", esc = 27 as char);
//     }

//     // Store Items Type
//     #[derive(Debug)]
//     struct ItemDetails<'a> {
//         name: &'a str,
//         description: &'a str,
//         price: u32,
//         increase: u32,
//     }

//     impl fmt::Display for ItemDetails<'_> {
//         fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//             write!(
//                 f,
//                 "{}          Price:${}\n({})\n\n\n",
//                 self.name, self.price, self.description
//             )
//         }
//     }

//     fn merchantSelection(player: &mut PlayerInfo) {
//         clear_console();

//         println!("Welcome stranger! \n");
//         println!("Your balance is {}", player.money);

//         // let health_items = vec![""]
//         let store_items = vec!["Health", "Firepower", "Defence", "Exit"];

//         let selection = Select::with_theme(&ColorfulTheme::default())
//             .items(&store_items)
//             .default(0)
//             .interact_on_opt(&Term::stderr());

//         match selection {
//             Ok(opt) => match opt {
//                 Some(index) => {
//                     if index == 0 {
//                         health_items(player)
//                     } else if index == 1 {
//                         fire_power_items(player);
//                     } else if index == 2 {
//                         crit_chance_items(player);
//                     } else if index == 3 {
//                         clear_console();
//                         start_selection(player);
//                     }
//                 }
//                 None => {
//                     clear_console();
//                     start_selection(player);
//                 }
//             },

//             Err(err) => {
//                 eprintln!("Error in merchantSelection items. {}", err)
//             }
//         }
//     }

//     fn health_items(player: &mut PlayerInfo) {
//         clear_console();
//         println!("Health Items \n");
//         println!("Press {} to ruturn \n", String::from("esc"));

//         let health_items_vec = vec![
//             ItemDetails {
//                 name: "Health Upgrade 1",
//                 description: "Increase your health by 100 points",
//                 price: 100,
//                 increase: 100,
//             },
//             ItemDetails {
//                 name: "Health Upgrade 2",
//                 description: "Increase your health by 200 points",
//                 price: 150,
//                 increase: 200,
//             },
//             ItemDetails {
//                 name: "Health Upgrade 3",
//                 description: "Increase your health by 500 points",
//                 price: 300,
//                 increase: 500,
//             },
//         ];

//         let health_selection = Select::with_theme(&ColorfulTheme::default())
//             .items(&health_items_vec)
//             .default(0)
//             .interact_on_opt(&Term::stderr());

//         match health_selection {
//             Ok(opt) => match opt {
//                 Some(index) => {
//                     if player.money < health_items_vec[index].price {
//                         println!("Not enought cash, stranger!");
//                         thread::sleep(Duration::from_millis(500));
//                         health_items(player);
//                     };

//                     let old_health = player.health;
//                     player.health += health_items_vec[index].increase;
//                     player.money -= health_items_vec[index].price;

//                     println!(
//                         "You upgraded your health from {} to {} for ${}",
//                         old_health, player.health, health_items_vec[index].price
//                     );

//                     thread::sleep(Duration::from_millis(500));
//                     health_items(player);
//                 }
//                 None => {
//                     merchantSelection(player);
//                 }
//             },
//             Err(err) => {
//                 eprintln!("{}", err)
//             }
//         }
//     }

//     fn fire_power_items(player: &mut PlayerInfo) {
//         clear_console();
//         println!("Health Items \n");
//         println!("Press {} to ruturn \n", String::from("esc"));

//         let damage_items_vec = vec![
//             ItemDetails {
//                 name: "Damage Upgrade 1",
//                 description: "Increases your min and max Damage by 10 points",
//                 price: 200,
//                 increase: 10,
//             },
//             ItemDetails {
//                 name: "Damage Upgrade 2",
//                 description: "Increases your min and max Damage by 15 points",
//                 price: 300,
//                 increase: 15,
//             },
//             ItemDetails {
//                 name: "Damage Upgrade 3",
//                 description: "Increases your min and max Damage by 25 points",
//                 price: 500,
//                 increase: 25,
//             },
//         ];

//         let damage_selection = Select::with_theme(&ColorfulTheme::default())
//             .items(&damage_items_vec)
//             .default(0)
//             .interact_on_opt(&Term::stderr());

//         match damage_selection {
//             Ok(opt) => match opt {
//                 Some(index) => {
//                     if player.money < damage_items_vec[index].price {
//                         println!("Not enought cash, stranger!");
//                         thread::sleep(Duration::from_millis(500));
//                         health_items(player);
//                     };

//                     let old_health = player.health;
//                     player.health += damage_items_vec[index].increase;
//                     player.money -= damage_items_vec[index].price;

//                     println!(
//                         "You upgraded your health from {} to {} for ${}",
//                         old_health, player.health, damage_items_vec[index].price
//                     );

//                     thread::sleep(Duration::from_millis(500));
//                     health_items(player);
//                 }
//                 None => {
//                     merchantSelection(player);
//                 }
//             },
//             Err(err) => {
//                 eprintln!("{}", err)
//             }
//         }
//     }

//     fn crit_chance_items(player: &mut PlayerInfo) {
//         clear_console();
//         println!("Crit chance Boosters");
//         println!("Can go up to a maximum of 40% crit chance \n");
//         println!("Press {} to ruturn \n", String::from("esc"));

//         let crit_chance_items_vec = vec![
//             ItemDetails {
//                 name: "Crit Chance Upgrade 1",
//                 description: "Increases your crit chance by 2%",
//                 price: 500,
//                 increase: 2,
//             },
//             ItemDetails {
//                 name: "Crit Chance Upgrade 2",
//                 description: "Increases your crit chance by 4%",
//                 price: 800,
//                 increase: 4,
//             },
//             ItemDetails {
//                 name: "Crit Chance Upgrade 3",
//                 description: "Increases your crit chance by 7%",
//                 price: 1500,
//                 increase: 7,
//             },
//         ];

//         let crit_chance_selection = Select::with_theme(&ColorfulTheme::default())
//             .items(&crit_chance_items_vec)
//             .default(0)
//             .interact_on_opt(&Term::stderr());

//         match crit_chance_selection {
//             Ok(opt) => match opt {
//                 Some(index) => {
//                     if player.money < crit_chance_items_vec[index].price {
//                         println!("Not enought cash, stranger!");
//                         thread::sleep(Duration::from_millis(500));
//                         health_items(player);
//                     };

//                     let old_health = player.health;
//                     player.health += crit_chance_items_vec[index].increase;
//                     player.money -= crit_chance_items_vec[index].price;

//                     println!(
//                         "You upgraded your health from {} to {} for ${}",
//                         old_health, player.health, crit_chance_items_vec[index].price
//                     );

//                     thread::sleep(Duration::from_millis(500));
//                     health_items(player);
//                 }
//                 None => {
//                     merchantSelection(player);
//                 }
//             },
//             Err(err) => {
//                 eprintln!("{}", err)
//             }
//         }
//     }
// }
