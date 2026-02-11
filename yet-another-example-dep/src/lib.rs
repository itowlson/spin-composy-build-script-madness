wit_bindgen::generate!({
    world: "uselessness",
});

struct Useless;

impl exports::useless::feature::the_useless_thing::Guest for Useless {
    #[allow(async_fn_in_trait)]
    fn do_the_useless_thing() -> () {
        println!("uh this is flippin useless");
    }

    fn do_another_but_equally_useless_thing() -> () {
        println!("I fear this is also useless");
    }
}

impl exports::useless::feature::argh::Guest for Useless {
    #[allow(async_fn_in_trait)]
    fn do_argh() -> () {
        println!("argh!")
    }
}

// impl exports::useless::feature::hibble_bibble::Guest for Useless {
//     #[allow(async_fn_in_trait)]
//     fn hibble_and_indeed_bibble() -> () {
//         println!("hibble and, indeed, bibble")
//     }
// }

export!(Useless);
