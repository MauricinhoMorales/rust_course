use rocket::{
    response::status,
    serde::json::{Value, json},
};

#[macro_use]
extern crate rocket;

#[get("/item")]
fn get_item_list() -> Value {
    json!([{"name":"test"}])
}

#[get("/item/<id>")]
fn get_item(id: u32) -> Value {
    json!([{"name":id}])
}

#[post("/item", format = "json")]
fn create_item() -> Value {
    json!([{"name":"test"}])
}

#[put("/item/<id>", format = "json")]
fn update_item(id: u32) -> Value {
    json!([{"name":id}])
}

#[delete("/item/<id>")]
fn delete_item(id: u32) -> status::NoContent {
    status::NoContent
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount(
        "/hello",
        routes![
            get_item_list,
            get_item,
            update_item,
            create_item,
            delete_item
        ],
    )
}
