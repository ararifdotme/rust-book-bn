mod back_of_house {
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

pub fn eat_at_restaurant() {
    // গ্রীষ্মে Rye toast-সহ একটি নাস্তার অর্ডার দিন।
    let mut meal = back_of_house::Breakfast::summer("Rye");
    // কোন রুটি চাই, সে বিষয়ে মত বদলান।
    meal.toast = String::from("Wheat");
    println!("I'd like {} toast please", meal.toast);

    // পরের লাইনটির কমেন্ট তুলে দিলে এটি compile হবে না; খাবারের সঙ্গে আসা
    // মৌসুমি ফল দেখার বা বদলানোর অনুমতি আমাদের নেই।
    // meal.seasonal_fruit = String::from("blueberries");
}
