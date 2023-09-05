use std::fs::File;
use dioxus::prelude::*;
use csv::{Reader, Writer};
use crate::components::Home::Records;
use serde::Deserialize;
#[derive(Debug, Deserialize)]
struct Record{
  id:u16,
  name:String,
  last:String,
  age:u8
}
#[inline_props]
pub fn Record(cx:Scope, id:u16) -> Element{
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
  let mut rdr = Reader::from_reader(File::open("data.csv").unwrap());
  render!{
    form{
      class:"check-in-form",
      prevent_default:"onsubmit",
      onsubmit:move |e|{
        let mut wrt = Writer::from_writer(File::open("dataa.csv").unwrap());
        for rec in rdr.deserialize(){
          let rec:Record = rec.unwrap();
          if rec.id == record.0{
            wrt.write_record(&[rec.id.to_string(), e.values.get("name").unwrap()[0].clone(), e.values.get("last").unwrap()[0].clone(), e.values.get("age").unwrap()[0].clone()]).unwrap();
          }else{
            wrt.write_record(&[rec.id.to_string(), rec.name, rec.last, rec.age.to_string()]).unwrap();
          };
        }
        wrt.flush().unwrap();
      },
      div{
        label{
          r#for:"name",
          "name",
        },
        input{
          r#type:"text",
          name:"name",
          value:"{record.1}"
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
          value:"{record.2}"
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
          value:"{record.3}"
        }
      },
      input{
        r#type:"submit",
      }
    }
  }
}