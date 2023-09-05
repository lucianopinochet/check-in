#![allow(non_snake_case)]
// #![windows_subsystem = "windows"]

mod components;

use dioxus_router::prelude::*;
use dioxus::prelude::*;
use log::LevelFilter;
use dioxus_desktop::tao::window::WindowBuilder;
use components::*;
#[rustfmt::skip]
#[derive(Clone, Routable, Debug, PartialEq)]
pub enum Route{
  #[layout(NavBar)]
    #[route("/")]
    Home{},
    #[route("/push")]
    Push{},
    #[route("/:id")]
    Record{
      id:u16
    },
  #[end_layout]
	#[route("/..notfound")]
	NotFound{}
}


fn main() {
    dioxus_logger::init(LevelFilter::Info).expect("failed to init logger");    
    dioxus_desktop::launch_cfg(app, dioxus_desktop::Config::new().with_window(WindowBuilder::new().with_title("Check In").with_resizable(true)));
}
fn app(cx: Scope) -> Element {
    render!{
        style {include_str!("./style.css")},
        Router::<Route> {}
    }
}


