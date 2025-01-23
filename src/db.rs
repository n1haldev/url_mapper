use mongodb::Collection;
use crate::model::UrlMap;
use mongodb::options::ClientOptions;
use mongodb::Client;
use mongodb::bson::doc;
use mongodb::results::InsertOneResult;

pub struct AppState {
    collection: Collection<UrlMap>,
}

impl AppState {
    pub async fn new() -> AppState {
        let client_options = ClientOptions::parse("mongodb://localhost:27017").await.unwrap();
        let client = Client::with_options(client_options).unwrap();

        let collection = client.database("urls_db").collection("urls");
        AppState {
            collection
        }
    }

    pub async fn add(&self, short_url: String, long_url: String) -> InsertOneResult {
        let url_mapping = UrlMap {
            short_url,
            long_url,
        };

        let result = self.collection.insert_one(url_mapping).await.unwrap();
        result
    }

    pub async fn retreive(&self, short_url: String) -> Option<UrlMap> {
        let result = self.collection.find_one(doc! {"short_url": short_url}).await.unwrap();
        result
    }
}
