use std::fs::File;
use csv::ReaderBuilder;
use dioxus::prelude::*;
use dioxus_router::prelude::*;
use crate::Route;

type Records = (String, String, u8);

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
#[inline_props]
pub fn Home(cx: Scope) -> Element{
  let toogle_name = use_state(cx, ||false);
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
    render!{
      tr{td{"{name}"}td{"{last}"}td{"{age}"}}
    }
  });
  render!{
    table{
      tr{
        if !toogle_name{
          th{
            "name",
            onclick:|_| toogle_name.set(true),
          }
        }else{
          th{
            "hola"
          }
        },
        th{
          "last"
        }
        th{
          "age"
        }
      }
      rendered_node
    }
  }
}
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
  let par = use_state(cx, ||("".to_string(), "".to_string(), "".to_string()));  
  let (name, last, age) = par.get();
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
          value:"{name}",
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
        }
      },
      input{
        r#type:"submit"
      }
    }
  }
}
#[inline_props]
pub fn NotFound(cx: Scope) -> Element{
  render!{
    "Not Found"
  }
}
