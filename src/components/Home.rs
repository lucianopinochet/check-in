  use dioxus_router::prelude::*;
use dioxus::prelude::*;
use std::fs::File;
use csv::ReaderBuilder;
use dioxus_free_icons::icons::{
  fa_solid_icons::FaMagnifyingGlass,
  bs_icons::{BsInfoCircleFill, BsArrowDownCircleFill},
};
use crate::Route;
use dioxus_free_icons::Icon;
pub type Records = (u16,String, String, u8);
enum Sort{
  UP,
  DOWN,
  NONE
}
#[inline_props]
pub fn Home(cx: Scope) -> Element{
  let toogle_query = use_state(cx, ||[false, false]);
  let query_name = use_state(cx, ||"".to_string());
  let query_age = use_state(cx, ||"".to_string());
  let name_sort = use_state(cx ,|| Sort::NONE);
  let age_sort = use_state(cx ,|| Sort::NONE);
  let file =  match File::open("data.csv"){
    Ok(file) => file,
    Err(_) => {
      let f = File::create("data.csv").unwrap();
      let mut wtr = csv::Writer::from_writer(f);
      wtr.write_record(&["Id","Name", "Last", "Age"]).unwrap();
      File::open("data.csv").unwrap()
    }
  };
  let mut rdr = ReaderBuilder::new()
    .flexible(true)
    .comment(Some(b'#'))
    .from_reader(file);
  let node_list = rdr.deserialize();  
  let mut node_list:Vec<_> = node_list.map(|res|{
    let (id, name, last, age):Records = res.unwrap();
    (id,name,last,age)
  }).collect();
  match name_sort.get() {
    Sort::UP => node_list.sort_by(|a, b| {
      let (_,a_name,_,_) = a;
      let (_,b_name,_,_) = b;
      a_name.to_uppercase().partial_cmp(&b_name.to_uppercase()).unwrap()
    }),
    Sort::DOWN => {
      node_list.sort_by(|a, b| {
        let (_,a_name,_,_) = a;
        let (_,b_name,_,_) = b;
        a_name.to_uppercase().partial_cmp(&b_name.to_uppercase()).unwrap()
      });
      node_list.reverse();
    },
    Sort::NONE => ()
  }
  match age_sort.get() {
    Sort::UP => node_list.sort_by(|a, b| {
      let (_,_,_,a_age) = a;
      let (_,_,_,b_age) = b;
      a_age.partial_cmp(b_age).unwrap()
    }),
    Sort::DOWN => {
      node_list.sort_by(|a, b| {
        let (_,_,_,a_age) = a;
        let (_,_,_,b_age) = b;
        a_age.partial_cmp(b_age).unwrap()
      });
      node_list.reverse();
    },
    Sort::NONE => ()
  }
  let node_list:Vec<_> = node_list.iter().filter(|&row|{
    let (_id, name, last, age) = row.clone();
    if !query_name.get().is_empty() && query_age.get().is_empty(){
          if name.to_uppercase().contains(&query_name.get().to_uppercase()) || last.to_uppercase().contains(&query_name.get().to_uppercase()){
            true
          }else{
            false
          }
        }else if query_name.get().is_empty() && !query_age.get().is_empty(){
          if age.to_string().to_uppercase().contains(&query_age.get().to_uppercase()){
            true
          }else{
            false
          }
        }else if !query_name.get().is_empty() && !query_age.get().is_empty(){
          if (name.to_uppercase().contains(&query_name.get().to_uppercase()) || last.to_uppercase().contains(&query_name.get().to_uppercase())) && age.to_string().to_uppercase().contains(&query_age.get().to_uppercase()){
            true
          }else{
            false
          }
        }else{
          true
        }

  }).collect();
  let rendered_body = node_list.iter().map(|result|{
    let result = *result;
    let (id, name, last, age ):Records = result.clone();
    render!{
      tr{td{"{id}"}td{"{name}"}td{"{last}"}td{"{age}"}
        td{
          div{
            class:"icon-option",
            Link{
              to:Route::Record{
                id:id
              },
              Icon {
                width:15,
                height:15,
                icon: BsInfoCircleFill,
                class:"icon"
              }
            }
          }
        }
      }
    }
  });
  let render_name = if toogle_query.get()[0] == false{
    render!{
      th{
        colspan:"2",
        style:"width:55%",
        div{
          class:"div-head",
          div{
            onclick:|_| {
              let mut toogle = toogle_query.get().clone();
              toogle[0] = true;
              toogle_query.set(toogle);
            },
            Icon {
              width:15,
              height:15,
              icon: FaMagnifyingGlass,
              class:"icon"
            },
          }
          h4{"Name"},
          div{
            onclick:|_|{
              match name_sort.get(){
                Sort::NONE => {
                  name_sort.set(Sort::UP);
                  age_sort.set(Sort::NONE);
                },
                Sort::UP => {
                  name_sort.set(Sort::DOWN);
                  age_sort.set(Sort::NONE);
                },
                Sort::DOWN => {
                  name_sort.set(Sort::NONE);
                  age_sort.set(Sort::NONE);
                }
              }
            },
            Icon{
              width:15,
              height:15,
              class:"icon",
              icon:BsArrowDownCircleFill
            }
          }
        }
      }
    }
  }else{
    render!{
      th{
        colspan:"2",
        style:"width:55%",
        h4{"Name"},
        form{
          prevent_default:"onsubmit",
          input{
            r#type:"text",
            value:"{query_name}",
            oninput:move |e| query_name.set(e.value.clone())
          }
        },
        div{
          onclick:|_|{
            match name_sort.get(){
              Sort::NONE => {
                name_sort.set(Sort::UP);
                age_sort.set(Sort::NONE);
              },
              Sort::UP => {
                name_sort.set(Sort::DOWN);
                age_sort.set(Sort::NONE);
              },
              Sort::DOWN => {
                name_sort.set(Sort::NONE);
                age_sort.set(Sort::NONE);
              }
            }
          },
          Icon{
            width:15,
            height:15,
            class:"icon",
            icon:BsArrowDownCircleFill
          }
        }
      }
    }
  };
  let render_age = if toogle_query.get()[1] == false{
    render!{
      th{
        style:"width:25%",
        div{
          class:"div-head",
          div{
            onclick:|_| {
              let mut toogle = toogle_query.get().clone();
              toogle[1] = true;
              toogle_query.set(toogle);
            },
            Icon {
              width:15,
              height:15,
              icon: FaMagnifyingGlass,
              class:"icon"
            },
          }
          h4{"Age"},
          div{
            onclick:|_|{
              match age_sort.get(){
                Sort::NONE => {
                  age_sort.set(Sort::UP);
                  name_sort.set(Sort::NONE);
                },
                Sort::UP => {
                  age_sort.set(Sort::DOWN);
                  name_sort.set(Sort::NONE);
                },
                Sort::DOWN => {
                  age_sort.set(Sort::NONE);
                  name_sort.set(Sort::NONE);
                }
              }
            },
            Icon{
              width:15,
              height:15,
              class:"icon",
              icon:BsArrowDownCircleFill
            }
          }
        }
      }
    }
  }else{
    render!{
      th{
        style:"width:25%",
        h4{"Age"},
        form{
          prevent_default:"onsubmit",
          input{
            r#type:"text",
            value:"{query_age}",
            oninput:move |e| query_age.set(e.value.clone())
          }
        },
        div{
          onclick:|_|{
            match age_sort.get(){
              Sort::NONE => {
                age_sort.set(Sort::UP);
                name_sort.set(Sort::NONE);
              },
              Sort::UP => {
                age_sort.set(Sort::DOWN);
                name_sort.set(Sort::NONE);
              },
              Sort::DOWN => {
                age_sort.set(Sort::NONE);
                name_sort.set(Sort::NONE);
              }
            }
          },
          Icon{
            width:15,
            height:15,
            class:"icon",
            icon:BsArrowDownCircleFill
          }
        }
      }
    }
  };
  render!{
    table{
      tr{
        th{
          style:"width:10%",
          h4{"Id"}
        },
        render_name,
        render_age,
        th{
          style:"width:10%",
          h4{"Options"}
        }
      }
      // rendered_head
      rendered_body
    }
  }
}