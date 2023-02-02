mod char_info;
use dialoguer::{console::Term, theme::ColorfulTheme, Select};
use rand::Rng;
use std::io;

fn main() -> std::io::Result<()> {
    // let mut name = String::new();
    // println!("Please type a name for your character");
    // io::stdin().read_line(&mut name).expect("Invalid name");
    // println!("Your name is {name}");
    // let items = vec!["Item 1", "item 2", "item 3"];
    // let selection = Select::with_theme(&ColorfulTheme::default())
    //     .items(&items)
    //     .default(0)
    //     .interact_on_opt(&Term::stderr())?;

    // match selection {
    //     Some(index) => {
    //         println!("User selected item : {}", items[index],)
    //     }
    //     None => println!("User did not select anything"),
    // }

    //fight simulation
    let mut players_health = 170;
    let mut boss_helth = 250;

    let mut player_hit_counter = 0;
    let mut boss_hit_counter = 0;

    loop {
        let player_hit_damage = player_hit();
        let boss_hit_damage = boss_hit();

        if boss_helth > player_hit_damage {
            player_hit_counter += 1;
            boss_helth -= player_hit_damage;
            println!("Boss has now {}hp \n", boss_helth);

            if players_health > boss_hit_damage {
                players_health -= boss_hit_damage;
                boss_hit_counter += 1;

                println!("Player has now {}hp \n", players_health)
            } else {
                break println!(
                    "Player is now dead! Killed with {} hits \n",
                    boss_hit_counter
                );
            }
        } else {
            break println!(
                "Boss is now dead! Killed with {} hits \n",
                player_hit_counter
            );
        }
    }

    fn player_hit() -> u32 {
        let mut rng = rand::thread_rng();
        let x: u32 = rng.gen();
        x % 60 + 10 // 10 to 40 hit points
    }

    fn boss_hit() -> u32 {
        let mut rng = rand::thread_rng();
        let x: u32 = rng.gen();
        x % 50 // 0 to 50 hit points
    }

    Ok(())
}
