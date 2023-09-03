use csv::Reader;
use dioxus::prelude::*;
use std::fs::File;

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
    println!("{record:?}");
  }
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
        if !first{
          id+=1;
        }
        let formated = format!("{id}").to_string();
        wtr.write_record(&[ formated, e.values.get("name").unwrap()[0].clone(), e.values.get("last").unwrap()[0].clone(), e.values.get("age").unwrap()[0].clone()]).unwrap();
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