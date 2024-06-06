mod front_of_house {
    pub struct Breakfast{
        pub toast: String,
        seasonal_fruit: String
    }

    impl Breakfast {
        pub fn summer(toast: &str) -> Breakfast {
            Breakfast {
                toast: String::from(toast),
                seasonal_fruit: String::from("peaches")
            }
        }
    }

    pub enum Appetizer {}

    pub mod hosting {
        pub fn add_to_waitlist() {
            // super关键字访问上级
            super::serving::serve_order()
        }
        fn seat_at_table() {}
    }

    mod serving {
        fn take_order() {}
        pub fn serve_order() {}
        fn take_payment() {}
    }
}

mod back_of_house;  // 模块后跟;，则从同级目录加载

// use模块
pub use crate::front_of_house::hosting; // pub use 重新导出

pub fn eat_at_restaurant() {
    crate::front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::add_to_waitlist();
    //
    let mut meal = front_of_house::Breakfast::summer("Rye");
    meal.toast = String::from("Wheat");

    println!("I'd like {} toast please", meal.toast);

    hosting::add_to_waitlist();
}