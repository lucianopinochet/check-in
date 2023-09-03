use dioxus::prelude::*;
use std::fs::File;
use csv::ReaderBuilder;
use dioxus_free_icons::icons::fa_solid_icons::FaMagnifyingGlass;
use dioxus_free_icons::Icon;
type Records = (u16,String, String, u8);

#[inline_props]
pub fn Home(cx: Scope) -> Element{
  let toogle_query = use_state(cx, ||[false, false]);
  let query_name = use_state(cx, ||"".to_string());
  let query_age = use_state(cx, ||"".to_string());
  let file =  match File::open("data.csv"){
    Ok(file) => file,
    Err(_) => {
      let f = File::create("data.csv").unwrap();
      let mut wtr = csv::Writer::from_writer(f);
      wtr.write_record(&["id","name", "last", "age"]).unwrap();
      File::open("data.csv").unwrap()
    }
  };
  let mut rdr = ReaderBuilder::new()
    .flexible(true)
    .comment(Some(b'#'))
    .from_reader(file);
  let node_list = rdr.deserialize();
  let rendered_body = node_list.map(|result|{
    let (id, name, last, age ):Records = result.unwrap();
    if !query_name.get().is_empty() && query_age.get().is_empty(){ 
      if name.to_uppercase().contains(&query_name.get().to_uppercase()) || last.to_uppercase().contains(&query_name.get().to_uppercase()){
        render!{
          tr{td{"{id}"}td{"{name}"}td{"{last}"}td{"{age}"}}
        }
      }else{
        render!{""}
      }
    }else if query_name.get().is_empty() && !query_age.get().is_empty(){
      if age.to_string().to_uppercase().contains(&query_age.get().to_uppercase()){
        render!{
          tr{td{"{id}"}td{"{name}"}td{"{last}"}td{"{age}"}}
        }
      }else{
        render!{""}
      }
    }else if !query_name.get().is_empty() && !query_age.get().is_empty(){
      if (name.to_uppercase().contains(&query_name.get().to_uppercase()) || last.to_uppercase().contains(&query_name.get().to_uppercase())) && age.to_string().to_uppercase().contains(&query_age.get().to_uppercase()){
        render!{
          tr{td{"{id}"}td{"{name}"}td{"{last}"}td{"{age}"}}
        }
      }else{
        render!{""}
      }
    }else{
      render!{
        tr{td{"{id}"}td{"{name}"}td{"{last}"}td{"{age}"}}
      }
    }
  });
  let rendered_head = match toogle_query.get().clone(){
    [false, false] => render!{
      tr{
        th{
          style:"width:10%",
          h4{"id"}
        }
        th{
          onclick:|_| {
            let mut toogle = toogle_query.get().clone();
            toogle[0] = true;
            toogle_query.set(toogle);
          },
          colspan:"2",
          style:"width:60%",
          h4{"name"},
          Icon {
            width:15,
            height:15,
            icon: FaMagnifyingGlass,
            class:"icon"
          },
        }
        th{
          onclick:|_| {
            let mut toogle = toogle_query.get().clone();
            toogle[1] = true;
            toogle_query.set(toogle);
          },
          style:"width:30%",
          h4{"age"},
          Icon {
            width:15,
            height:15,
            icon: FaMagnifyingGlass,
            class:"icon"
          },
        }
      }
    },
    [true, false] => render!{
      tr{
        th{
          style:"width:10%",
          h4{"id"}
        }
        th{
          colspan:"2",
          style:"width:66%",
          h4{"name"},
          form{
            prevent_default:"onsubmit",
            input{
              r#type:"text",
              value:"{query_name}",
              oninput:move |e| query_name.set(e.value.clone())
            }
          },
        }
        th{
          onclick:|_| {
            let mut toogle = toogle_query.get().clone();
            toogle[1] = true;
            toogle_query.set(toogle);
          },
          style:"width:33%",
          h4{"age"},
          Icon {
            width:15,
            height:15,
            icon: FaMagnifyingGlass,
            class:"icon"
          },
        }
      }
    },
    [false,true] =>render!{
      tr{
        th{
          style:"width:10%",
          h4{"id"}
        }
        th{
          onclick:|_| {
            let mut toogle = toogle_query.get().clone();
            toogle[0] = true;
            toogle_query.set(toogle);
          },
          colspan:"2",
          style:"width:66%",
          h4{"name"},
          Icon {
            width:15,
            height:15,
            icon: FaMagnifyingGlass,
            class:"icon"
          },
        }
        th{
          style:"width:33%",
          h4{"age"},
          form{
            prevent_default:"onsubmit",
            input{
              r#type:"text",
              value:"{query_age}",
              oninput:move |e| query_age.set(e.value.clone())
            }
          }
        }
      }
    },
    [true, true] => render!{
      tr{
        th{
          style:"width:10%",
          h4{"id"}
        }
        th{
          colspan:"2",
          style:"width:66%",
          h4{"name"},
          form{
            prevent_default:"onsubmit",
            input{
              r#type:"text",
              value:"{query_name}",
              oninput:move |e| query_name.set(e.value.clone())
            }
          },
        }
        th{
          style:"width:33%",
          h4{"age"},
          form{
            prevent_default:"onsubmit",
            input{
              r#type:"text",
              value:"{query_age}",
              oninput:move |e| query_age.set(e.value.clone())
            }
          }
        }
      }
    }
  };
  render!{
    table{
      rendered_head
      rendered_body
    }
  }
}