# struct2swagger [![Build Status](https://travis-ci.org/allevo/struct2swagger.svg?branch=master)](https://travis-ci.org/allevo/struct2swagger)

Utilities for generating OpenAPI Specification from your structures

This fork implement more types like enums by using [Schemars](https://crates.io/crates/schemars) instead of the own struct2swagger serialization.
Also has support for path operations like /users/{id}: and use of $ref to schema instead of schema
## Install
```
cargo add struct2swagger_derive struct2swagger
```

## Usage

```rust

#[macro_use]
extern crate struct2swagger_derive;
#[macro_use]
extern crate struct2swagger;
#[macro_use]
extern crate serde_json;

use struct2swagger::{JsonSchemaDefinition, QueryDefinition, swagger_object::SwaggerObject};
use serde_json::{ Result, value::Value};


#[derive(Deserialize, Swagger, JsonSchema)]
pub struct Who {
    pub name: Option<String>,
    pub my_enum: HelloWorldEnum,
}
#[derive(Deserialize, Swagger, JsonSchema)]
pub struct HelloWorldResponse {
    pub say: String,
}
#[derive(Deserialize, Swagger, JsonSchema)]
pub enum HelloWorldEnum {
    AA,
    BB,
    CC,
}

fn get_openapi_spec() -> String {
  let mut swagger_object = SwaggerObject::new(
        "the webserver name", // title
        "1.0.0" // version
        Some(vec![
            json!(&schema_for!(Who).schema),
            json!(&schema_for!(HelloWorldResponse).schema),
            json!(&schema_for!(HelloWorldEnum).schema),
        ]),
    );

  swagger_add_router!(
      swagger_object, // obj
      "GET", // method
      "/", // path
      Who, // query parameters
      200, // expected status code
      "say", //  description
      HelloWorldResponse // struct in output
  );

  let stringified = serde_json::to_string(&swagger_object).unwrap();

  stringified
}
```

For other examples see [tests](./struct2swagger_derive/tests/swagger.rs)

## Contributing

Every contribution is welcomed: Open an issue and fire a PR!

## License

MIT as described [here](./LICENSE)
