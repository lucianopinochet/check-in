use dioxus::prelude::*;
use std::fs::File;

#[inline_props]
pub fn Push(cx: Scope) -> Element{
  let file =  match File::options().append(true).open("data.csv"){
    Ok(file) => {
      file
    },
    Err(_) => {
      File::options().append(true).open("data.csv").unwrap()
    }
  };
  let mut wtr  = csv::Writer::from_writer(file);
  render!{
    form{
      class:"check-io-form",
      prevent_default:"onsubmit",
      onsubmit: move |e|{
        wtr.write_record(&[e.values.get("name").unwrap()[0].clone(), e.values.get("last").unwrap()[0].clone(), e.values.get("age").unwrap()[0].clone()]).unwrap();
        wtr.flush().unwrap();
      },
      div{
        label{
          r#for:"name",
          "name",
        },
        input{
          r#type:"text",
          name:"name",
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
        }
      },
      input{
        r#type:"submit"
      }
    }
  }
}