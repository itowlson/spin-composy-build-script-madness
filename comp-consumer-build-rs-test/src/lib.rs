use spin_sdk::http::{IntoResponse, Request, Response};
use spin_sdk::http_component;

spin_sdk::wit_bindgen::generate!({
    path: "./spin-dependencies.wit",
    world: "root",
    runtime_path: "::spin_sdk::wit_bindgen::rt",
    generate_all,
});

#[http_component]
fn handle_comp_consumer_build_rs_test(_req: Request) -> anyhow::Result<impl IntoResponse> {
    useless::feature::the_useless_thing::do_the_useless_thing();
    useless::feature::argh::do_argh();

    useful::thingy_dingy::exciteville::thrill_me();
    // println!("1 + 1 = {}", useful::thingy::this_is_also_terribly_exciting::surprise_me(1));
    
    Ok(Response::builder()
        .status(200)
        .header("content-type", "text/plain")
        .body("Hello World!!\n")
        .build())
}
