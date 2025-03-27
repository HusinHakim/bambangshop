use rocket::serde::{Deserialize, Serialize};
use rocket::serde::json::to_string;
use rocket::log;
use bambangshop::REQWEST_CLIENT;
use crate::model::notification::Notification;

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Subscriber {
    pub url: String,
    pub name: String,
}

impl Subscriber {
    #[tokio::main]
    pub async fn update(&self, payload: Notification) {
        REQWEST_CLIENT
            .post(&self.url)
            .header("Content-Type", "JSON")
            .post(to_string(&payload).unwrap())
            .send().await.ok();
            
        log::warn!("<Sent {} notification of: {{}}, type: {}, title: {}, to: {}>", 
            payload.status, payload.product_type, payload.product_title, self.url);
    }
} 