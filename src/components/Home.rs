use dioxus::prelude::*;
use std::fs::File;
use csv::ReaderBuilder;
use dioxus_free_icons::icons::{
  fa_solid_icons::FaMagnifyingGlass,
  bs_icons::BsInfoCircleFill,
  // md_navigation_icons::MdDeleteForever,
  md_action_icons::MdDeleteForever
};
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
      wtr.write_record(&["Id","Name", "last", "Age"]).unwrap();
      File::open("data.csv").unwrap()
    }
  };
  let mut rdr = ReaderBuilder::new()
    .flexible(true)
    .comment(Some(b'#'))
    .from_reader(file);
  let node_list = rdr.deserialize();
  // let node_list = node_list.map(|res|{
  //   let (id, name, last, age):Records = res.unwrap();
  //   if !query_name.get().is_empty() && query_age.get().is_empty(){
  //     if name.to_uppercase().contains(&query_name.get().to_uppercase()) || last.to_uppercase().contains(&query_name.get().to_uppercase()){
  //       (id,name,last,age)
  //     }
  //   }else if query_name.get().is_empty() && !query_age.get().is_empty(){
  //     if age.to_string().to_uppercase().contains(&query_age.get().to_uppercase()){
  //       (id,name,last,age)
  //     }
  //   }else if !query_name.get().is_empty() && !query_age.get().is_empty(){
  //     if (name.to_uppercase().contains(&query_name.get().to_uppercase()) || last.to_uppercase().contains(&query_name.get().to_uppercase())) && age.to_string().to_uppercase().contains(&query_age.get().to_uppercase()){
  //       (id,name,last,age)
  //     }
  //   }else{
  //     (id,name,last,age)
  //   }
  // });
  let node_list:Vec<_> = node_list.map(|res|{
    let (id, name, last, age):Records = res.unwrap();
    (id,name,last,age)
  }).collect();
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
    let (id, name, last, age ):Records = **result;
    render!{
      tr{td{"{id}"}td{"{name}"}td{"{last}"}td{"{age}"}
        td{
          button{
            onclick:|e|{
              println!("{e:?}");
            },
            class:"icon-option",
            Icon {
            width:15,
            height:15,
            icon: BsInfoCircleFill,
            }
            }
          }
          td{
            button{
              onclick:|e|{
                println!("{e:?}");
              },
              class:"icon-option",
              Icon {
              width:15,
              height:15,
              icon: MdDeleteForever,
              }
            }
          }
        }
    }
    // if !query_name.get().is_empty() && query_age.get().is_empty(){ 
    //   if name.to_uppercase().contains(&query_name.get().to_uppercase()) || last.to_uppercase().contains(&query_name.get().to_uppercase()){
    //     render!{
    //       tr{td{"{id}"}td{"{name}"}td{"{last}"}td{"{age}"}
    //       td{
    //         button{
    //           onclick:|e|{
    //             println!("{e:?}");
    //           },
    //           class:"icon-option",
    //           Icon {
    //           width:15,
    //           height:15,
    //           icon: BsInfoCircleFill,
    //           }
    //           }
    //         }
    //         td{
    //           button{
    //             onclick:|e|{
    //               println!("{e:?}");
    //             },
    //             class:"icon-option",
    //             Icon {
    //             width:15,
    //             height:15,
    //             icon: MdDeleteForever,
    //             }
    //           }
    //         }
    //       }
    //     }
    //   }else{
    //     render!{""}
    //   }
    // }else if query_name.get().is_empty() && !query_age.get().is_empty(){
    //   if age.to_string().to_uppercase().contains(&query_age.get().to_uppercase()){
    //     render!{
    //       tr{td{"{id}"}td{"{name}"}td{"{last}"}td{"{age}"}
    //       td{
    //         button{
    //             class:"icon-option",
    //             Icon {
    //             width:15,
    //             height:15,
    //             icon: BsInfoCircleFill,
    //             }
    //           }
    //         }
    //         td{
    //           button{
    //             class:"icon-option",
    //             Icon {
    //             width:15,
    //             height:15,
    //             icon: MdDeleteForever,
    //             }
    //           }
    //         }
    //       }
    //     }
    //   }else{
    //     render!{""}
    //   }
    // }else if !query_name.get().is_empty() && !query_age.get().is_empty(){
    //   if (name.to_uppercase().contains(&query_name.get().to_uppercase()) || last.to_uppercase().contains(&query_name.get().to_uppercase())) && age.to_string().to_uppercase().contains(&query_age.get().to_uppercase()){
    //     render!{
    //       tr{td{"{id}"}td{"{name}"}td{"{last}"}td{"{age}"}
    //       td{
    //         button{
    //             class:"icon-option",
    //             Icon {
    //             width:15,
    //             height:15,
    //             icon: BsInfoCircleFill,
    //             }
    //           }
    //         }
    //         td{
    //           button{
    //             class:"icon-option",
    //             Icon {
    //             width:15,
    //             height:15,
    //             icon: MdDeleteForever,
    //             }
    //           }
    //         }
    //       }
    //     }
    //   }else{
    //     render!{""}
    //   }
    // }else{
    //   render!{
    //     tr{td{"{id}"}td{"{name}"}td{"{last}"}td{"{age}"}
    //       td{
    //         button{
    //           class:"icon-option",
    //           Icon {
    //           width:15,
    //           height:15,
    //           icon: BsInfoCircleFill,
    //           }
    //         }
    //       }
    //       td{
    //         button{
    //           class:"icon-option",
    //           Icon {
    //           width:15,
    //           height:15,
    //           icon: MdDeleteForever,
    //           }
    //         }
    //       }
    //     }
    //   }
    // }
  });  
  let rendered_head = match toogle_query.get().clone(){
    [false, false] => render!{
      tr{
        th{
          style:"width:10%",
          h4{"Id"}
        }
        th{
          colspan:"2",
          style:"width:55%",
          div{
            class:"div-head",
            onclick:|_| {
              let mut toogle = toogle_query.get().clone();
              toogle[0] = true;
              toogle_query.set(toogle);
            },
            h4{"Name"},
            Icon {
              width:15,
              height:15,
              icon: FaMagnifyingGlass,
              class:"icon"
            },
          }
        }
        th{
          style:"width:25%",
          div{
            class:"div-head",
            onclick:|_| {
              let mut toogle = toogle_query.get().clone();
              toogle[1] = true;
              toogle_query.set(toogle);
            },
            h4{"Age"},
            Icon {
              width:15,
              height:15,
              icon: FaMagnifyingGlass,
              class:"icon"
            },
          }
        }
        th{
          colspan:"2",
          style:"width:10%",
          h4{"Options"}
        }
      }
    },
    [true, false] => render!{
      tr{
        th{
          style:"width:10%",
          h4{"Id"}
        }
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
        }
        th{
          style:"width:25%",
          div{
            class:"div-head",
            onclick:|_| {
              let mut toogle = toogle_query.get().clone();
              toogle[1] = true;
              toogle_query.set(toogle);
            },
            h4{"Age"},
            Icon {
              width:15,
              height:15,
              icon: FaMagnifyingGlass,
              class:"icon"
            },
          }
        }
        th{
          colspan:"2",
          style:"width:10%",
          h4{"Options"}
        }
      }
    },
    [false,true] =>render!{
      tr{
        th{
          style:"width:10%",
          h4{"Id"}
        }
        th{
          colspan:"2",
          style:"width:55%",
          div{
            class:"div-head",
            onclick:|_| {
              let mut toogle = toogle_query.get().clone();
              toogle[0] = true;
              toogle_query.set(toogle);
            },
            h4{"Name"},
            Icon {
              width:15,
              height:15,
              icon: FaMagnifyingGlass,
              class:"icon"
            },
          }
        }
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
          }
        }
        th{
          colspan:"2",
          style:"width:10%",
          h4{"Options"}
        }
      }
    },
    [true, true] => render!{
      tr{
        th{
          style:"width:10%",
          h4{"Id"}
        }
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
        }
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
          }
        }
        th{
          colspan:"2",
          style:"width:10%",
          h4{"Options"}
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