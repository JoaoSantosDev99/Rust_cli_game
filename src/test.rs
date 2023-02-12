use std::{thread, time};

fn main() {
    let delay = time::Duration::from_secs(1);

    'battle: loop {
        println!("sleeping for 3  sec ");

        break;
    }

    fn excute_loop() {
        'battle();
    }

    excute_loop();
    thread::sleep(time::Duration::from_secs(1));

    excute_loop();
    thread::sleep(time::Duration::from_secs(1));

    excute_loop();
    thread::sleep(time::Duration::from_secs(1));
}
