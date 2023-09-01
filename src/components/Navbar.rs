use dioxus::prelude::*;
use crate::Route;
use dioxus_router::prelude::*;

#[inline_props]
pub fn NavBar(cx: Scope) -> Element{
  render!{
    div{
      class:"navbar",
      Link{
        to:Route::Home {},
        "Push"
      }
      Link{
        to:Route::Push {},
        "Pull"
      }  
    }
    Outlet::<Route> {}
  }
}