pub mod kvstore {
    tonic::include_proto!("kvstore");
}

use kvstore::{kvstore_client::KvstoreClient, GetParams};
use kvstore::{Item};
use tonic::Request;

use crate::kvstore::AllItemsParams;


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = KvstoreClient::connect("http://[::1]:10000").await?;
    
    // let response = client.get(Request::new(GetParams {
    //     key: "hello".to_string()
    // })).await?;

    let response = client.put(Request::new(Item {
        key: "hello".to_string(),
        value: "world".to_string(),
    })).await?;

    println!("RESPONSE = {:?}", response);

    let responseb = client.put(Request::new(Item {
        key: "lovely".to_string(),
        value: "home".to_string(),
    })).await?;

    println!("RESPONSE = {:?}", responseb);

    let mut all_itms_stream = client.get_all_items(Request::new(AllItemsParams {})).await?.into_inner();

    while let Some(itm) = all_itms_stream.message().await? {
        print!("Recieved: {:?}\n", itm);
    }

     Ok(())
}


