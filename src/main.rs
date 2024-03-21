pub mod fuzzy;

use fuzzy::system::infer;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[macro_use]
extern crate rocket;

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Input {
    pub txprodplan: f64,
    pub mse: f64,
}
#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct Conclusion {
    pub conc: f64,
}

#[post("/inferir", format = "json", data = "<input>")]
fn inferir(input: Json<Input>) -> Json<Conclusion> {
    let conc = infer(input.txprodplan, input.mse);
    Json(Conclusion { conc })
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![inferir])
}
