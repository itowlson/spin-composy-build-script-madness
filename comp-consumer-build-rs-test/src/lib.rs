use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

include!(concat!(env!("OUT_DIR"), "/biscuits.rs"));

#[http_component]
fn handle_comp_consumer_build_rs_test(_req: Request) -> anyhow::Result<impl IntoResponse> {
    useless_dep::useless::feature::the_useless_thing::do_the_useless_thing();
    useless_dep::useless::feature::argh::do_argh();

    useful_dep::useful::thingy::this_is_terribly_exciting::thrill_me();
    println!("1 + 1 = {}", useful_dep::useful::thingy::this_is_also_terribly_exciting::surprise_me(1));
    
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello World!!\n")
        .build())
}
