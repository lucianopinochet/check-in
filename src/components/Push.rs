use csv::{Reader, Writer};
use dioxus::prelude::*;
use dioxus_router::prelude::Link;
use std::fs::File;
use crate::Route;
#[inline_props]
pub fn Push(cx: Scope) -> Element{
  let mut id:u16 = 0;
  let mut first = true;
  let mut rdr = Reader::from_reader(File::open("data.csv").unwrap());
  let mut iter = rdr.records();
  while let Some(res) = iter.next(){
    first = false;
    let record = res.unwrap();
    id = record.get(0).unwrap().parse::<u16>().unwrap();
  }
  let file =  match File::options().append(true).open("data.csv"){
    Ok(file) => {
      file
    },
    Err(_) => {
      File::options().append(true).open("data.csv").unwrap()
    }
  };
  let mut wtr  = Writer::from_writer(file);
  let name = use_state(cx, || "".to_string());
  let last = use_state(cx, ||"".to_string());
  let age = use_state(cx, ||0);
  render!{
    form{
      class:"check-in-form",
      prevent_default:"onsubmit",
      div{
        label{
          r#for:"name",
          "name",
        },
        input{
          r#type:"text",
          name:"name",
          value:"{name}",
          oninput:move|e|name.set(e.value.clone()),
        },
      },
      div{
        label{
          r#for:"last",
          "last"
        },
        input{
          r#type:"text",
          name:"last",
          value:"{last}",
          oninput:move|e|last.set(e.value.clone()),
        },
      },
      div{
        label{
          r#for:"age",
          "age"
        },
        input{
          r#type:"number",
          name:"age",
          value:"{age}",
          oninput:move|e|age.set(e.value.clone().parse::<u8>().unwrap()),
        }
      },
      Link{
        to:Route::Home{},
        onclick:move|_|{
          if !first{
            id+=1;
          }
          let formated = format!("{id}").to_string();
          wtr.write_record(&[ formated, name.get().clone(), last.get().clone(), age.get().clone().to_string()]).unwrap();
          wtr.flush().unwrap();
        },
        input{
          r#type:"submit"
        }
      }
    }
  }
}