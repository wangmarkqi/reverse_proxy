use futures::stream::StreamExt;

use mongodb::{bson::{doc, Bson}, options::FindOptions, Collection};
use mongodb::{Client, options::ClientOptions};
use async_std::task;
lazy_static! {
    static ref COL: Collection = {
      task::block_on(async {
      let col=get_collection().await.unwrap();
      col
    })
    };
}



pub async fn get_collection() -> anyhow::Result<Collection> {
    let mut client_options = ClientOptions::parse("mongodb://39.96.40.177:27017/").await?;
    client_options.app_name = Some("reverse".to_string());
    let client = Client::with_options(client_options)?;
    let db = client.database("config");
    let collection = db.collection("reverse");
    Ok(collection)
}

pub async fn get_port(name: &str) -> String {
    let collection = &*COL;
    let filter = doc! { "name": name };
    let mut cursor = collection.find(filter, None).await.unwrap();
    while let Some(result) = cursor.next().await {
        match result {
            Ok(document) => {
                if let Some(port) = document.get("port").and_then(Bson::as_str) {
                    return port.to_string();
                }
            }
            Err(e) => {
                dbg!(e);
                return "".to_string();
            }
        }
    }
    "".to_string()
}

pub async fn insert_data() -> anyhow::Result<()> {
    // Get a handle to a collection in the database.
    let collection = &*COL;

    let docs = vec![
        vec!["static", "7891"],
        vec!["rpc", "7891"],
        vec!["flutter", "8084"],
        vec!["app", "8085"],
        vec!["pass", "8884"],
        vec!["hardware", "8885"],
        vec!["wsmApi", "8886"],
        vec!["rm", "8887"],
        vec!["xg", "9001"],
    ];
    for item in docs.iter() {
        let filter = doc! {"name":item[0]};
        // let update = doc! {"$set": {"port":item[1]}};
        let fd = collection.find_one(filter, None).await?;
        if let Some(x) = fd {
            dbg!(x);
        } else {
            let txt = doc! {"name":item[0],"port":item[1]};
            collection.insert_one(txt, None).await?;
        }

        // collection.update_one(filter,update, None).await?;
    }
    Ok(())
}