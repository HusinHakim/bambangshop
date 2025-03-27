use rocket::response::status::Created;
use rocket::serde::json::Json;

use bambangshop::Result;
use crate::model::subscriber::Subscriber;
use crate::service::notification::NotificationService;

#[post("/<product_type>/subscribe", data = "<subscriber>")]
pub fn subscribe(product_type: &str, subscriber: Json<Subscriber>) -> Result<Created<Json<Subscriber>>> {
    return match NotificationService::subscribe(product_type, subscriber.into_inner()) {
        Ok(s) => Ok(Created::new("/").body(Json::from(s))),
        Err(e) => Err(e)
    };
}

#[post("/unsubscribe/<product_type>/<url>")]
pub fn unsubscribe(product_type: &str, url: &str) -> Result<Json<Subscriber>> {
    return match NotificationService::unsubscribe(product_type, url) {
        Ok(s) => Ok(Json::from(s)),
        Err(e) => Err(e)
    };
}