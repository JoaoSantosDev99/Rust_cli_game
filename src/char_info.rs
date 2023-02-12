// Setters
pub mod getters {
    const TEST: u32 = 12;

    pub fn get_test() {
        println!("Hello test");

        enum Colors {
            blue,
            red,
            yellow,
        }

        enum Price {
            car(u32),
            bike(u32),
            boat(String),
        }

        let car_color = Colors::red;
        let car_price = Price::car(1000);
    }
}
