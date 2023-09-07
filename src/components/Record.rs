use std::fs::File;
use dioxus::prelude::*;
use csv::{Reader, Writer, StringRecord};
use dioxus_router::prelude::Link;
use crate::components::Home::Records;
use crate::Route;
#[inline_props]
pub fn Record(cx:Scope, id:u16) -> Element{
  let delete  = use_state(cx, ||false);
  let mut rdr = Reader::from_reader(File::open("data.csv").unwrap());
  let records = rdr.deserialize();
  let mut record:Records = (0, "".to_string(), "".to_string(), 0);
  for res in records{
    let (record_id, name, last, age):Records = res.unwrap();
    if record_id == id.clone(){
      record = (record_id, name, last, age);
      break
    }
  };
  let name = use_state(cx, || record.1);
  let last = use_state(cx, ||record.2);
  let age = use_state(cx, ||record.3.to_string());
  let delete_button = if delete.get().clone() == false{
      render!{
        div{
          input{
            r#type:"button",
            value:"Eliminar",
            prevent_default:"onsubmit",
            onclick:move|_|delete.set(true),
          }
        }
      }
  }else{
    render!{
      div{
        Link{
          to:Route::Home{},
          onclick:move|_|{
            let mut rdr = Reader::from_path("data.csv").unwrap();
            let headers = rdr.headers().unwrap().clone();
            let mut records:Vec<StringRecord> = Vec::new();
            for result in rdr.records(){
              let record = result.unwrap();
              if record.get(0).unwrap() != id.to_string(){
                records.push(record);
              }
            }
            let mut wrt = Writer::from_path("data.csv").unwrap();
            wrt.write_record(&headers).unwrap();
            for record in records{
              wrt.write_record(&record).unwrap();
            }
            wrt.flush().unwrap();
          },
          input{
            r#type:"submit",
            value:"Seguro?"
          }
      
        }
      }
    }
  };    

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
          oninput:move|e|last.set(e.value.clone())
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
          oninput:move|e|age.set(e.value.clone())
        }
      },
      div{
        Link{
          to:Route::Home{},
          onclick:move |_|{
            let mut rdr = Reader::from_path("data.csv").unwrap();
            let headers = rdr.headers().unwrap().clone();
            let mut records:Vec<StringRecord> = Vec::new();
            for result in rdr.records(){
              let record = result.unwrap();
              if record.get(0).unwrap() == id.to_string(){
                records.push(StringRecord::from(vec![id.to_string(), name.get().clone(), last.get().clone(), age.get().clone()]));
              }else{
                records.push(record);
              }
            }
            let mut wrt = Writer::from_path("data.csv").unwrap();
            wrt.write_record(&headers).unwrap();
            for record in records{
              wrt.write_record(&record).unwrap();
            }
            wrt.flush().unwrap();
          },
          input{
            r#type:"submit",
            value:"Enviar"
          }
        }
      },
      delete_button,
    }
  }
}
// pub fn AppRecord(cx:Scope) -> Element{
//   render!{
//     "",
    
//   }
// }