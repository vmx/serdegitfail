use serde::Serialize;
use serde_json::to_vec;

#[derive(Serialize)]
struct MyStruct {
    data: bool,
}

fn main() {
    to_vec(&MyStruct { data: true }).unwrap();
}
