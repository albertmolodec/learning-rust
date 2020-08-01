fn serve_order() {
    println!("Serve order");
}

mod back_of_house {
    fn fix_incorrect_order() {
        println!("Fix incorrect order");
        cook_order();
        super::serve_order();
    }

    fn cook_order() {
        println!("Cook order");
    }

    pub struct Breakfast {
        pub toast: String,
        seasonal_fruit: String,
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches"),
            }
        }
    }
}

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("Add to waitlist");
        }

        fn seat_at_table() {
            println!("Seat at table");
        }
    }

    mod serving {
        fn take_order() {
            println!("Take order");
        }

        fn serve_order() {
            println!("Serve order");
        }

        fn take_payment() {
            println!("Take payment");
        }
    }
}

pub fn eat_at_restaurant() {
    // Order a breakfast in the summer with Rye toast
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // Change our mind about what bread we'd like
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    front_of_house::hosting::add_to_waitlist();
}
