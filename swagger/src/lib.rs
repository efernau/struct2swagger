#![recursion_limit = "128"]

extern crate proc_macro;
extern crate proc_macro2;
extern crate syn;
#[macro_use]
extern crate quote;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

mod impl_data_types;
mod impl_swagger_trait;
pub mod swagger_object;

#[derive(Debug)]
struct Field {
    name: String,
    ty: Vec<proc_macro2::TokenTree>,
}

pub use impl_swagger_trait::implements_swagger_trait;

pub trait JsonSchemaDefinition {
    fn get_json_schema_definition() -> serde_json::Value;
}
pub trait QueryDefinition {
    fn get_query_definition() -> serde_json::Value;
}

#[macro_export]
macro_rules! swagger_add_router {
    ($swagger_object:expr, "GET", $path:literal, 200, $response:ident) => {
        $swagger_object.add_route(
            "GET",
            String::from($path),
            None,
            None,
            vec![(200 as u16, ("", $response::get_json_schema_definition()))],
        )
    };
    ($swagger_object:expr, "POST", $path:literal, "request_body", $req: ident, 200, $response:ident) => {
        let mut content_hash_map = HashMap::new();
        content_hash_map.insert(
            "application/json".to_owned(),
            MediaTypeObject {
                schema: Some(SchemaObjectOrReferenceObject::SchemaObject(Box::new(
                    $req::get_json_schema_definition(),
                ))),
                example: None,
                examples: None,
                encoding: None,
            },
        );
        $swagger_object.add_route(
            "POST",
            String::from($path),
            None,
            Some(RequestBodyObject {
                description: None,
                content: content_hash_map,
                required: Some(true),
            }),
            vec![(200 as u16, ("", $response::get_json_schema_definition()))],
        )
    };
}
