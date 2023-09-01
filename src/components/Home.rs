use dioxus::prelude::*;
use std::fs::File;
use csv::ReaderBuilder;

type Records = (String, String, u8);

#[inline_props]
pub fn Home(cx: Scope) -> Element{
  let toogle_query = use_state(cx, ||[false, false]);
  let query = use_state(cx, ||"".to_string());
  let file =  match File::open("data.csv"){
    Ok(file) => file,
    Err(_) => {
      File::create("data.csv").unwrap();
      File::open("data.csv").unwrap()
    }
  };
  let mut rdr = ReaderBuilder::new()
    .flexible(true)
    .comment(Some(b'#'))
    .from_reader(file);
  let node_list = rdr.deserialize();
  let rendered_node = node_list.map(|result|{
    let (name, last, age):Records = result.unwrap();
    if !query.get().is_empty(){
      if name.to_uppercase().starts_with(&query.get().to_uppercase()) || last.to_uppercase().starts_with(&query.get().to_uppercase()){
        render!{
          tr{td{"{name}"}td{"{last}"}td{"{age}"}}
        }
      }else{
        render!{""}
      }
    }else{
      render!{
        tr{td{"{name}"}td{"{last}"}td{"{age}"}}
      }
    }
  });
  let rendered_head = match toogle_query.get().clone(){
    [false, false] => render!{
      tr{
        th{
          onclick:|_| {
            let mut toogle = toogle_query.get().clone();
            toogle[0] = true;
            toogle_query.set(toogle);
          },
          colspan:"2",
          style:"width:66%",
          "name"
        }
        th{
          onclick:|_| {
            let mut toogle = toogle_query.get().clone();
            toogle[1] = true;
            toogle_query.set(toogle);
          },
          style:"width:33%",
          "age"
        }
      }
    },
    [true, false] => render!{
      tr{
        th{
          colspan:"2",
          style:"width:66%",
          form{
            prevent_default:"onsubmit",
            input{
              r#type:"text",
              value:"{query}",
              oninput:move |e| query.set(e.value.clone())
            }
          },
        }
        th{
          style:"width:33%",
          "age"
        }
      }
    },
    [false,true] =>render!{
      tr{
        th{
          colspan:"2",
          style:"width:66%",
          form{
            prevent_default:"onsubmit",
            input{
              r#type:"text",
              value:"{query}",
              oninput:move |e| query.set(e.value.clone())
            }
          },
        }
        th{
          style:"width:33%",
          "age"
        }
      }
    },
    [true, true] => render!{
      tr{
        th{
          colspan:"2",
          style:"width:66%",
          form{
            prevent_default:"onsubmit",
            input{
              r#type:"text",
              value:"{query}",
              oninput:move |e| query.set(e.value.clone())
            }
          },
        }
        th{
          style:"width:33%",
          "age"
        }
      }
    }
  };
  render!{
    rendered_head
    rendered_node
  }
}