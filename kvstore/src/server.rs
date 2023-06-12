pub mod kvstore {
    tonic::include_proto!("kvstore");
}

use kvstore::kvstore_server::{Kvstore, KvstoreServer};
use kvstore::{AllItemsParams, GetParams, Item, Success};
use std::collections::HashMap;

use std::sync::Arc;
use tokio::sync::mpsc;
use tokio::sync::Mutex;
use tokio_stream::wrappers::ReceiverStream;
use tonic::transport::Server;
use tonic::{Request, Response, Status};

#[derive(Debug)]
struct KvstoreService {
    data: Arc<Mutex<HashMap<String, String>>>,
}

#[tonic::async_trait]
impl Kvstore for KvstoreService {
    async fn get(&self, request: Request<GetParams>) -> Result<Response<Item>, Status> {
        let hm = &self.data.lock().await;
        match hm.get(&request.get_ref().key) {
            Some(s) => {
                let prod = Item {
                    key: request.get_ref().key.to_string(),
                    value: s.to_string(),
                };
                return Ok(Response::new(prod));
            }
            None => {
                let prod = Item {
                    key: "".to_string(),
                    value: "".to_string(),
                };
                return Ok(Response::new(prod));
            }
        }
        // } else {
        //     let prod = Item {
        //                 key: "".to_string(),
        //                 value: "".to_string()
        //             };
        //             return Ok(Response::new(prod))
        // }
    }

    async fn put(&self, request: Request<Item>) -> Result<Response<Item>, Status> {
        let itms = request.get_ref();
        let key = itms.key.to_string();
        let val = itms.value.to_string();
        let mut hm = self.data.lock().await;
        hm.insert(key, val);
        Ok(Response::new(Item {
            key: "".to_string(),
            value: "".to_string(),
        }))
    }

    async fn delete(&self, request: Request<GetParams>) -> Result<Response<Success>, Status> {
        let itms = request.get_ref();
        let key = itms.key.to_string();
        let mut hm = self.data.lock().await;
        let out = hm.remove(&key);
        Ok(Response::new(match out {
            Some(_v) => Success { success: true },
            None => Success { success: false },
        }))
    }

    type GetAllItemsStream = ReceiverStream<Result<Item, Status>>;

    async fn get_all_items(
        &self,
        _request: Request<AllItemsParams>,
    ) -> Result<Response<Self::GetAllItemsStream>, Status> {
        let (tx, rx) = mpsc::channel(4);
        let data = self.data.clone();

        tokio::spawn(async move {
            let hm = data.lock().await.clone();
            for itm in hm.into_iter() {
                tx.send(Ok(Item {
                    key: itm.0,
                    value: itm.1,
                }))
                .await
                .unwrap()
            }
        });

        Ok(Response::new(ReceiverStream::new(rx)))
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let addr = "[::1]:10000".parse().unwrap();

    let route_guide = KvstoreService {
        data: Arc::new(Mutex::new(HashMap::new())),
    };

    let svc = KvstoreServer::new(route_guide);

    Server::builder().add_service(svc).serve(addr).await?;

    Ok(())
}
