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

use schemars::{gen::SchemaGenerator, schema_for, JsonSchema};

#[derive(Debug)]
struct Field {
    name: String,
    ty: Vec<proc_macro2::TokenTree>,
}

pub use impl_swagger_trait::implements_swagger_trait;
pub use swagger_object::{ParameterIn, ParameterObject, SchemaObjectOrReferenceObject};

pub trait JsonSchemaDefinition {
    fn get_json_schema_definition() -> serde_json::Value;
}
pub trait QueryDefinition {
    fn get_query_definitions() -> Vec<ParameterObject>;
}

#[macro_export]
macro_rules! swagger_add_router {
    ($swagger_object:expr, "GET", $path:literal, $query_params: ident, 200, $description: expr, $response:ident) => {{
        $swagger_object.add_route(
            "GET",
            String::from($path),
            None, // TODO
            // Some(
            //     <$query_params>::get_query_definitions()
            //         .into_iter()
            //         .map(|p| {
            //             $crate::swagger_object::ParameterObjectOrReferenceObject::ParameterObject(
            //                 Box::new(p),
            //             )
            //         })
            //         .collect(),
            // ),
            None,
            vec![(
                200 as u16,
                // ($description, $response::get_json_schema_definition()),
                ($description, &json!(&schema_for!($response).schema)["title"].to_string().trim_matches('\"').to_string(), json!(&schema_for!($response).schema)),

            )],
        )

    }};
    ($swagger_object:expr, "GET", $path:literal, 200, $description: expr, $response:ident) => {{
        $swagger_object.add_route(
            "GET",
            String::from($path),
            // Check path if we need a ParameterObject
            None,
            None,
            vec![(
                200 as u16,
                // ($description, $response::get_json_schema_definition()),
                ($description, &json!(&schema_for!($response).schema)["title"].to_string().trim_matches('\"').to_string(), json!(&schema_for!($response).schema)),
            )],
        )
    }};
    ($swagger_object:expr, "DELETE", $path:literal, 200, $description: expr, $response:ident) => {{
        $swagger_object.add_route(
            "DELETE",
            String::from($path),
            None,
            None,
            vec![(
                200 as u16,
                // ($description, $response::get_json_schema_definition()),
                ($description, &json!(&schema_for!($response).schema)["title"].to_string().trim_matches('\"').to_string(), json!(&schema_for!($response).schema)),
            )],
        )
    }};
    ($swagger_object:expr, $method:literal, $path:literal, "request_body", $req: ident, 200, $description: expr, $response:ident) => {{
        use struct2swagger::swagger_object::{
            MediaTypeObject, RequestBodyObject, SchemaObjectOrReferenceObject,
        };
        let mut content_hash_map = HashMap::new();
        content_hash_map.insert(
            "application/json".to_owned(),
            MediaTypeObject {
                // use SchemaObjectOrReferenceObject::ReferenceObject for ref to Schemars, not SchemaObject?
                schema: None, // TODO
                // Some(SchemaObjectOrReferenceObject::SchemaObject(Box::new(
                //     $req::get_json_schema_definition(),
                // ))),
                example: None,
                examples: None,
                encoding: None,
            },
        );
        $swagger_object.add_route(
            $method,
            String::from($path),
            None,
            Some(
                (&json!(&schema_for!($req).schema)["title"].to_string().trim_matches('\"').to_string(),
                RequestBodyObject {
                    description: None,
                    content: content_hash_map,
                    required: Some(true),
                })
            ),
            vec![(
                200 as u16,
                // ($description, $response::get_json_schema_definition()),
                ($description, &json!(&schema_for!($response).schema)["title"].to_string().trim_matches('\"').to_string(), json!(&schema_for!($response).schema)),

            )],
        )
    }};
}
