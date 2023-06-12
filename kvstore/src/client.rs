pub mod kvstore {
    tonic::include_proto!("kvstore");
}

use inquire::{required, Select, Text};
use kvstore::Item;
use kvstore::{kvstore_client::KvstoreClient, GetParams};
use tonic::Request;

use crate::kvstore::AllItemsParams;

#[derive(Debug, Clone, Copy)]
enum SelectOption {
    GetItem,
    PutItem,
    ListAll,
    Delete,
    Exit,
}

impl std::fmt::Display for SelectOption {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SelectOption::GetItem => write!(f, "Get an Item"),
            SelectOption::PutItem => write!(f, "Add an Item"),
            SelectOption::ListAll => write!(f, "List all Item"),
            SelectOption::Delete => write!(f, "Delete an Item"),
            SelectOption::Exit => write!(f, "Exit"),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = KvstoreClient::connect("http://[::1]:10000").await?;

    // let response = client.get(Request::new(GetParams {
    //     key: "hello".to_string()
    // })).await?;

    loop {
        let option_vec = vec![
            SelectOption::GetItem,
            SelectOption::PutItem,
            SelectOption::ListAll,
            SelectOption::Delete,
            SelectOption::Exit,
        ];
        let ans = Select::new("Chose Option from Menu?", option_vec).prompt();
        match ans {
            Ok(opt) => match opt {
                SelectOption::GetItem => {
                    let key = Text::new("Key of Item:")
                        .with_validator(required!("This field is required"))
                        .prompt()?;
                    let response = client.get(Request::new(GetParams { key })).await.unwrap();
                    println!("{:?}", response);
                }
                SelectOption::PutItem => {
                    let key = Text::new("Key of Item:")
                        .with_validator(required!("This field is required"))
                        .prompt()?;
                    let value = Text::new("Value of Item:")
                        .with_validator(required!("This field is required"))
                        .prompt()?;
                    let response = client.put(Request::new(Item { key, value })).await.unwrap();
                    println!("{:?}", response);
                }
                SelectOption::ListAll => {
                    let mut all_itms_stream = client
                        .get_all_items(Request::new(AllItemsParams {}))
                        .await?
                        .into_inner();
                    while let Some(itm) = all_itms_stream.message().await? {
                        println!("Recieved: {:?}\n", itm);
                    }
                }
                SelectOption::Delete => {
                    let key = Text::new("Key of Item:")
                        .with_validator(required!("This field is required"))
                        .prompt()?;
                    let response = client
                        .delete(Request::new(GetParams { key }))
                        .await
                        .unwrap();
                    println!("{:?}", response);
                }
                SelectOption::Exit => {
                    println!("EXIT");
                    break;
                }
            },
            Err(e) => {
                break;
            }
        };
    }

    // let response = client.put(Request::new(Item {
    //     key: "hello".to_string(),
    //     value: "world".to_string(),
    // })).await?;

    // println!("RESPONSE = {:?}", response);

    // let responseb = client.put(Request::new(Item {
    //     key: "lovely".to_string(),
    //     value: "home".to_string(),
    // })).await?;

    // println!("RESPONSE = {:?}", responseb);

    // let mut all_itms_stream = client.get_all_items(Request::new(AllItemsParams {})).await?.into_inner();

    // while let Some(itm) = all_itms_stream.message().await? {
    //     print!("Recieved: {:?}\n", itm);
    // }

    Ok(())
}
