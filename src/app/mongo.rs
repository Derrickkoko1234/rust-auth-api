use crate::{
    app::redis::Redis,
    library::{constant, pool::Pool},
};
use actix_web::error;
use mongodb::{error::Error, options::FindOptions, IndexModel};
use serde::{de::DeserializeOwned, Serialize};
use std::env;
use structural::Structural;

use super::cloudflare_s3::CloudflareS3;
use super::fcm::FCM;

use bson::{doc, Document};
use mongodb::{
    results::{DeleteResult, InsertManyResult, InsertOneResult, UpdateResult},
    Client, Collection, Cursor, Database,
};
use simple_mutex::Mutex;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppInstance {
    pub db: Arc<Mutex<Database>>,
    pub redis: Redis,
    pub pool: Pool,
    pub mutex: Arc<Mutex<i32>>,
    pub cloudflare_s3: CloudflareS3,
    pub fcm: FCM,
}

fn push_index(index: &str, indexes: &mut Vec<IndexModel>, col: &str) {
    let without_text = IndexModel::builder().keys(doc! {index:-1}).build(); // in descending order i.e -1

    if constant::FULL_TEXT_SEARCH_FIELDS.contains(&index) {
        let with_text = IndexModel::builder().keys(doc! {index:"text"}).build(); // for full-text search. i.e the $text index
        indexes.push(with_text);
    }

    indexes.push(without_text);
}

impl AppInstance {
    pub async fn init() -> Self {
        let uri = env::var("DB_URL").expect("DB_URL not found");
        let db_name = env::var("DB_NAME").expect("DB_NAME not found");

        let client = Client::with_uri_str(uri)
            .await
            .expect("error connecting to database");
        let db = Arc::new(Mutex::new(client.database(&db_name)));

        // init redis
        let redis = Redis::init().unwrap();

        // init worker pool
        let pool = Pool::new();

        // init global mutex
        let mutex = Arc::new(Mutex::new(0));

        let cloudflare_s3 = CloudflareS3::init();
        let fcm = FCM::init().await.expect("FCM init failed");

        let app_instance = AppInstance {
            db,
            redis,
            pool,
            mutex,
            cloudflare_s3,
            fcm,
        };

        // create indexes
        let _ = Self::create_index(&app_instance).await;

        app_instance
    }

    pub fn col_helper<T>(data_source: &Self, collection_name: &str) -> Collection<T> {
        // locking here to prevent race conditions. especially on usage stats and wallet balance updates
        data_source.db.lock().collection(collection_name)
    }

    pub async fn create_index(&self) -> Result<bool, Error> {
        for collection_name in constant::DB_COLLECTIONS {
            let col = AppInstance::col_helper::<String>(&self, collection_name);

            let mut indexes = vec![];

            if collection_name.eq(constant::USERS_COLLECTION) {
                for index in constant::USERS_COLLECTION_INDEXES {
                    push_index(index, &mut indexes, &collection_name);
                }
            }

            if collection_name.eq(constant::BLOCKCHAINS_COLLECTION) {
                for index in constant::BLOCKCHAINS_COLLECTION_INDEXES {
                    push_index(index, &mut indexes, &collection_name);
                }
            }

            if collection_name.eq(constant::TOKENS_COLLECTION) {
                for index in constant::TOKENS_COLLECTION_INDEXES {
                    push_index(index, &mut indexes, &collection_name);
                }
            }

            if collection_name.eq(constant::BANNERS_COLLECTION) {
                for index in constant::BANNERS_COLLECTION_INDEXES {
                    push_index(index, &mut indexes, &collection_name);
                }
            }

            if collection_name.eq(constant::COUNTRIES_COLLECTION) {
                for index in constant::COUNTRIES_COLLECTION_INDEXES {
                    push_index(index, &mut indexes, &collection_name);
                }
            }

            if collection_name.eq(constant::FEEDBACKS_COLLECTION) {
                for index in constant::FEEDBACKS_COLLECTION_INDEXES {
                    push_index(index, &mut indexes, &collection_name);
                }
            }

            if collection_name.eq(constant::KYC_SUBMISSIONS) {
                for index in constant::KYC_SUBMISSIONS_INDEXES {
                    push_index(index, &mut indexes, &collection_name);
                }
            }

            if collection_name.eq(constant::BROADCASTS_COLLECTION) {
                for index in constant::BROADCASTS_COLLECTION_INDEXES {
                    push_index(index, &mut indexes, &collection_name);
                }
            }

            if collection_name.eq(constant::FIAT_WALLETS_COLLECTION) {
                for index in constant::FIAT_WALLETS_COLLECTION_INDEXES {
                    push_index(index, &mut indexes, &collection_name);
                }
            }

            if collection_name.eq(constant::P2P_ORDERS_COLLECTION) {
                for index in constant::P2P_ORDERS_COLLECTION_INDEXES {
                    push_index(index, &mut indexes, &collection_name);
                }
            }

            // ADD OTHERS HERE...

            // create indexes
            for index in indexes {
                let _ = col.create_index(index, None).await;
            }
        }

        Ok(true)
    }

    // GET Helpers
    pub async fn get_one_item<
        U: Structural + Serialize + DeserializeOwned + Unpin + Send + Sync,
    >(
        &self,
        filter: &Document,
        collection_name: &str,
    ) -> Result<Option<U>, error::Error> {
        let col = AppInstance::col_helper::<U>(&self, &collection_name);
        let item = col.find_one(filter.to_owned(), None).await;

        if let Err(e) = item {
            return Err(error::ErrorInternalServerError(e));
        }

        Ok(item.unwrap())
    }

    pub async fn get_many_items<
        U: Structural + Serialize + DeserializeOwned + Unpin + Send + Sync,
    >(
        &self,
        filter: &Document,
        options: &Option<FindOptions>,
        collection_name: &str,
    ) -> Result<Cursor<U>, error::Error> {
        let col = AppInstance::col_helper::<U>(&self, &collection_name);
        let cursors = col.find(filter.to_owned(), options.clone().unwrap()).await;

        if let Err(e) = cursors {
            return Err(error::ErrorInternalServerError(e));
        }

        Ok(cursors.unwrap())
    }

    pub async fn get_many_items_by_aggregate<
        U: Structural + Serialize + DeserializeOwned + Unpin + Send + Sync,
    >(
        &self,
        filter: &Document,
        collection_name: &str,
        limit_results: i32,
    ) -> Result<Cursor<Document>, error::Error> {
        let sort = doc! {
            // sort by creation date, descending: -1 = descending, 1 = ascending
            "$sort": {
               "_id": -1
            }
        };
        let limit = doc! { "$limit": limit_results };
        let col = AppInstance::col_helper::<U>(&self, &collection_name);
        let cursors = col
            .aggregate(vec![filter.to_owned(), sort, limit], None)
            .await;

        if let Err(e) = cursors {
            return Err(error::ErrorInternalServerError(e));
        }

        Ok(cursors.unwrap())
    }

    pub async fn get_many_items_by_aggregate_randomized<
        U: Structural + Serialize + DeserializeOwned + Unpin + Send + Sync,
    >(
        &self,
        filter: &Document,
        collection_name: &str,
        limit_results: i32,
    ) -> Result<Cursor<Document>, error::Error> {
        let randomizer = doc! {
            // randomize the results
            "$sample": {
               "size": limit_results
            }
        };
        let limit = doc! { "$limit": limit_results };
        let col = AppInstance::col_helper::<U>(&self, &collection_name);
        let cursors = col
            .aggregate(vec![filter.to_owned(), randomizer, limit], None)
            .await;

        if let Err(e) = cursors {
            return Err(error::ErrorInternalServerError(e));
        }

        Ok(cursors.unwrap())
    }

    pub async fn get_many_items_by_aggregate_plain<
        U: Structural + Serialize + DeserializeOwned + Unpin + Send + Sync,
    >(
        &self,
        filter: &Vec<Document>,
        collection_name: &str,
    ) -> Result<Cursor<Document>, error::Error> {
        let col = AppInstance::col_helper::<U>(&self, &collection_name);
        let cursors = col.aggregate(filter.to_owned(), None).await;

        if let Err(e) = cursors {
            return Err(error::ErrorInternalServerError(e));
        }

        Ok(cursors.unwrap())
    }

    // INSERT Helpers
    pub async fn insert_one_item<U: Structural + Serialize>(
        &self,
        new_item: &U,
        collection_name: &str,
    ) -> Result<InsertOneResult, error::Error> {
        let col = AppInstance::col_helper::<U>(&self, &collection_name);
        let data = col.insert_one(new_item, None).await;

        if let Err(e) = data {
            return Err(error::ErrorInternalServerError(e));
        }
        Ok(data.unwrap())
    }

    pub async fn _insert_many_items<U: Structural + Serialize>(
        &self,
        new_items: &Vec<&U>,
        collection_name: &str,
    ) -> Result<InsertManyResult, error::Error> {
        let col = AppInstance::col_helper::<U>(&self, &collection_name);
        let data = col.insert_many(new_items.to_owned(), None).await;

        if let Err(e) = data {
            return Err(error::ErrorInternalServerError(e));
        }

        Ok(data.unwrap())
    }

    // UPDATE Helpers

    /** e.g usage
     * let filter = doc!{"_id":"1"};
     * let update = doc!{"$set": {"temp":"5"}};
     */
    pub async fn update_one_item<U: Structural + Serialize>(
        &self,
        filter: &Document,
        update: &Document,
        collection_name: &str,
    ) -> Result<UpdateResult, error::Error> {
        let col = AppInstance::col_helper::<U>(&self, &collection_name);
        let data = col
            .update_one(filter.to_owned(), update.to_owned(), None)
            .await;

        if let Err(e) = data {
            return Err(error::ErrorInternalServerError(e));
        }

        Ok(data.unwrap())
    }

    /** e.g usage
     * let filter = doc!{"free_api_calls_used": { "$gt": 0 }};
     * let update = doc!{"$set": {"free_api_calls_used": 0}};
     */
    pub async fn update_many_items<U: Structural + Serialize>(
        &self,
        filter: &Document,
        update: &Document,
        collection_name: &str,
    ) -> Result<UpdateResult, error::Error> {
        let col = AppInstance::col_helper::<U>(&self, &collection_name);
        let data = col
            .update_many(filter.to_owned(), update.to_owned(), None)
            .await;

        if let Err(e) = data {
            return Err(error::ErrorInternalServerError(e));
        }

        Ok(data.unwrap())
    }

    // DELETE Helpers

    /** e.g usage
     * let filter = doc!{"_id":"1"}; OR
     * let filter = doc!{ "category" : "Inshallah" }; // i.e delete one item with specified category
     */
    pub async fn delete_one_item<U: Structural + Serialize>(
        &self,
        filter: &Document,
        collection_name: &str,
    ) -> Result<DeleteResult, error::Error> {
        let col = AppInstance::col_helper::<U>(&self, &collection_name);
        let data = col.delete_one(filter.to_owned(), None).await;

        if let Err(e) = data {
            return Err(error::ErrorInternalServerError(e));
        }
        Ok(data.unwrap())
    }

    /** e.g usage
     * let filter = doc!{"_id":"1"}; OR
     * let filter = doc!{ "category" : "Inshallah" }; // i.e delete all items with specified category
     */
    pub async fn delete_many_items<U: Structural + Serialize>(
        &self,
        filter: &Document,
        collection_name: &str,
    ) -> Result<DeleteResult, error::Error> {
        let col = AppInstance::col_helper::<U>(&self, &collection_name);
        let data = col.delete_many(filter.to_owned(), None).await;
        if let Err(e) = data {
            return Err(error::ErrorInternalServerError(e));
        }
        Ok(data.unwrap())
    }
}
