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
  let file =  match File::open("data.csv"){
    Ok(file) => file,
    Err(_) => {
      File::create("data.csv").unwrap();
      println!("created");
      File::open("data.csv").unwrap()
    }
  };
  let mut rdr = ReaderBuilder::new()
    .flexible(true)
    .comment(Some(b'#'))
    .from_reader(file);
  for result in rdr.deserialize(){
    let record:Records = result.unwrap();
    println!("{:?}",record)
  }
  render!{
    "Home"
  }
}
#[inline_props]
pub fn Push(cx: Scope) -> Element{
  let file =  match File::open("data.csv"){
    Ok(file) => file,
    Err(_) => {
      File::create("data.csv").unwrap();
      println!("created");
      File::open("data.csv").unwrap()
    }
  };
  let mut rdr = ReaderBuilder::new()
    .flexible(true)
    .comment(Some(b'#'))
    .from_reader(file);
  for result in rdr.deserialize(){
    let record:Records = result.unwrap();
    println!("{:?}",record)
  }
  let par = use_state(cx, ||("".to_string(), "".to_string(), "".to_string()));  
  let (name, last, age) = par.get();
  render!{
    "Push",
    form{
      class:"check-io-form",
      prevent_default:"onsubmit",
      onsubmit: move |_|{
        println!("{name} {last} {age}")
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
          oninput: move|e| par.set((e.value.clone(),last.clone(), age.clone()))
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
          oninput: move|e| par.set((name.clone(),e.value.clone(), age.clone()))
        },
      },
      div{
        label{
          r#for:"age",
          "age"
        },
        input{
          r#type:"text",
          name:"age",
          value:"{age}",
          oninput: move|e| par.set((name.clone(),last.clone(), e.value.clone()))
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
