wit_bindgen::generate!();

struct Thingy;

impl exports::useful::thingy::this_is_terribly_exciting::Guest for Thingy {
    #[allow(async_fn_in_trait)]
    fn thrill_me() -> () {
        println!("are you thrilled yet")
    }
}

impl exports::useful::thingy::this_is_also_terribly_exciting::Guest for Thingy {
    #[allow(async_fn_in_trait)]
    fn surprise_me(bubbles:i32,) -> i32 {
        bubbles + 1
    }
}

export!(Thingy);
