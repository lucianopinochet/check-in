#![allow(non_snake_case)]

mod components;

use dioxus_router::prelude::*;
use dioxus::prelude::*;
use log::LevelFilter;
use components::*;

#[rustfmt::skip]
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route{
  #[layout(NavBar)]
    #[route("/")]
    Home{},
    #[route("/push")]
    Push{},
  #[end_layout]
	#[route("/..notfound")]
	NotFound{}
}
fn main() {
    // Init debug
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");    
    dioxus_desktop::launch(app);
}
fn app(cx: Scope) -> Element {
    render!{
        style {include_str!("./style.css")},
        Router::<Route> {}
    }
}


