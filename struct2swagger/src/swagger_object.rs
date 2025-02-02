use std::collections::HashMap;

use serde::{Serialize, Serializer};

use schemars::JsonSchema;

use serde_json::value::Value;

#[derive(Debug, Clone)]
pub enum SwaggerVersion {
    V300,
}
impl Serialize for SwaggerVersion {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(match *self {
            SwaggerVersion::V300 => "3.0.0",
        })
    }
}

type HttpStatusCode = u16;

#[derive(Serialize, Debug, Clone)]
pub struct ContactObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct LicenseObject {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ServerVariableObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#enum: Option<Vec<String>>,
    pub r#default: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ServerObject {
    pub url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub variables: Option<HashMap<String, ServerVariableObject>>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InfoObject {
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contact: Option<ContactObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<LicenseObject>,
    pub version: String,
}

macro_rules! or_reference {
    ($name: ident, $ty: ident) => {
        #[derive(Serialize, Debug, Clone)]
        #[serde(untagged)]
        pub enum $name {
            $ty(Box<$ty>),
            ReferenceObject(ReferenceObject),
        }
    };
}

or_reference!(SchemaObjectOrReferenceObject, SchemaObject);
or_reference!(ResponseObjectOrReferenceObject, ResponseObject);
or_reference!(ParameterObjectOrReferenceObject, ParameterObject);
or_reference!(ExampleObjectOrReferenceObject, ExampleObject);
or_reference!(RequestBodyObjectOrReferenceObject, RequestBodyObject);
or_reference!(HeaderObjectOrReferenceObject, HeaderObject);
or_reference!(SecuritySchemeObjectOrReferenceObject, SecuritySchemeObject);
or_reference!(LinkObjectOrReferenceObject, LinkObject);
or_reference!(CallbackObjectOrReferenceObject, CallbackObject);

#[derive(Serialize, Debug, Clone)]
#[serde(untagged)]
pub enum AnyOrExpression {
    Any(serde_json::Value),
    Expression(String),
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ComponentsObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    // pub schemas: Option<HashMap<String, SchemaObjectOrReferenceObject>>,
    // we need a generic
    pub schemas: Option<HashMap<String, Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub responses: Option<HashMap<String, ResponseObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, ParameterObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<HashMap<String, ExampleObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_bodies: Option<HashMap<String, RequestBodyObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, HeaderObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security_schemes: Option<HashMap<String, SecuritySchemeObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<HashMap<String, LinkObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<HashMap<String, CallbackObjectOrReferenceObject>>,
}

pub type PathsObject = HashMap<String, PathItemObject>;

#[derive(Serialize, Debug, Clone)]
pub struct PathItemObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub get: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub put: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delete: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub head: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub patch: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<OperationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ServerObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<Vec<ParameterObjectOrReferenceObject>>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OperationObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentationObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    // pub parameters: Option<Vec<ParameterObjectOrReferenceObject>>,
    pub parameters: Option<Vec<Value>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<RequestBodyObjectOrReferenceObject>,
    responses: ResponsesObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub callbacks: Option<HashMap<String, CallbackObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<Vec<SecurityRequirementObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ServerObject>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ExternalDocumentationObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub url: String,
}

#[derive(Clone, Serialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ParameterIn {
    Query,
    Header,
    Path,
    Cookie,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ParameterObject {
    pub name: String,
    #[serde(rename = "in")]
    pub where_in: ParameterIn,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_empty_value: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<SchemaObjectOrReferenceObject>,
}

#[derive(Serialize, Debug, Clone)]
pub struct RequestBodyObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub content: HashMap<String, MediaTypeObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

#[derive(Serialize, Debug, Clone)]
pub struct MediaTypeObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub schema: Option<SchemaObjectOrReferenceObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub example: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub examples: Option<HashMap<String, ExampleObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<HashMap<String, EncodingObject>>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EncodingObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content_type: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, HeaderObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub style: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explode: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_reserved: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct ResponsesObject {
    pub default: Option<ResponseObjectOrReferenceObject>,
    pub responses_per_http_status_codes:
        Option<HashMap<HttpStatusCode, ResponseObjectOrReferenceObject>>,
    // add title
    //  pub title : Option<String>,
}
impl Serialize for ResponsesObject {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut r: HashMap<String, ResponseObjectOrReferenceObject> = HashMap::new();
        if self.responses_per_http_status_codes.is_some() {
            for (k, value) in self
                .responses_per_http_status_codes
                .as_ref()
                .unwrap()
                .iter()
            {
                r.insert(k.to_string().to_owned(), value.clone());
            }
        }
        if self.default.is_some() {
            r.insert("default".to_owned(), self.default.as_ref().unwrap().clone());
        }

        r.serialize(serializer)
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct ResponseObject {
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<HashMap<String, HeaderObjectOrReferenceObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<HashMap<String, MediaTypeObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub links: Option<HashMap<String, LinkObjectOrReferenceObject>>,
}

type CallbackObject = HashMap<String, PathItemObject>;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ExampleObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_value: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LinkObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_ref: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub operation_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<HashMap<String, AnyOrExpression>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_body: Option<AnyOrExpression>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server: Option<ServerObject>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct HeaderObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deprecated: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allow_empty_value: Option<bool>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TagObject {
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentationObject>,
}

#[derive(Serialize, Debug, Clone)]
pub struct ReferenceObject {
    pub r#ref: String,
}

type SchemaObject = serde_json::Value;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DiscriminatorObject {
    pub property_name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mapping: Option<HashMap<String, String>>,
}

#[derive(Serialize, Debug, Clone)]
pub struct XMLObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub namespace: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prefix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub attribute: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wrapped: Option<bool>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SecuritySchemeObject {
    pub r#type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub r#in: Option<ParameterIn>,
    pub scheme: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bearer_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flows: Option<OAuthFlowsObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub open_id_connect_url: Option<String>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlowsObject {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub implicit: Option<OAuthFlowObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password: Option<OAuthFlowObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_credentials: Option<OAuthFlowObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<OAuthFlowObject>,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OAuthFlowObject {
    pub authorization_url: String,
    pub token_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refresh_url: Option<String>,
    pub scopes: HashMap<String, String>,
}

type SecurityRequirementObject = HashMap<String, Value>;

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SwaggerObject {
    pub openapi: SwaggerVersion,
    pub info: InfoObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub servers: Option<Vec<ServerObject>>,
    pub paths: PathsObject,
    // #[serde(skip_serializing_if = "Option::is_none")]
    pub components: ComponentsObject,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub security: Option<SecurityRequirementObject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<TagObject>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_docs: Option<ExternalDocumentationObject>,
}

impl SwaggerObject {
    pub fn new(
        title: &str,
        version: &str,
        servers: Vec<String>,
        description: &str,
        schemas: Option<Vec<Value>>,
    ) -> Self {
        let mut content_map = HashMap::new();
        if schemas != None {
            for schema in schemas.unwrap() {
                // This is a ugly $ref replace, better use Schemars SchemaGenerator?.
                let mut gen_schema =
                    format!("{}", schema).replace(r#"#/definitions/"#, r#"#/components/schemas/"#);
                // v3.1.0 allow null, we use v3.0.0 and remove null and replace with nullable: true
                gen_schema = gen_schema.replace(r#","null"]"#, r#"],"nullable":true"#);
                gen_schema = gen_schema.replace(r#","null"]"#, r#"],"nullable":true"#);
                gen_schema = gen_schema.replace(r#",{"type":"null"}]"#, r#"],"nullable":true"#);
                // normalisation
                gen_schema = gen_schema.replace(r#""type":["array"]"#, r#""type":"array""#);
                gen_schema = gen_schema.replace(r#""type":["object"]"#, r#""type":"object""#);
                gen_schema = gen_schema.replace(r#""type":["string"]"#, r#""type":"string""#);
                gen_schema = gen_schema.replace(r#""type":["boolean"]"#, r#""type":"boolean""#);
                gen_schema = gen_schema.replace(r#""type":["integer"]"#, r#""type":"integer""#);
                gen_schema = gen_schema.replace(r#""type":["number"]"#, r#""type":"number""#);
                // ip fix ;(
                gen_schema = gen_schema.replace(
                    r##"{"$ref":"#/components/schemas/Ipv6Net"}"##,
                    r#"{"type":"string"}"#,
                );
                gen_schema = gen_schema.replace(
                    r##"{"$ref":"#/components/schemas/IpNet"}"##,
                    r#"{"type":"string"}"#,
                );

                let new_schema = serde_json::from_str(&gen_schema).unwrap();

                content_map.insert(
                    schema
                        .get("title")
                        .unwrap()
                        .to_string()
                        .trim_matches('"')
                        .to_string(), // need the name
                    new_schema,
                );
            }
        }

        // Servers
        let mut new_servers = vec![];
        for server in servers {
            new_servers.push(ServerObject {
                url: server,
                description: None,
                variables: None,
            })
        }

        // set Auth
        let mut auth = HashMap::new();
        auth.insert(
            "bearerAuth".to_string(),
            SecuritySchemeObjectOrReferenceObject::SecuritySchemeObject(Box::new(
                SecuritySchemeObject {
                    r#type: "http".to_string(),
                    description: Some("Bearer Authentication See RFC 6750".to_string()),
                    name: None,
                    r#in: None,
                    scheme: "bearer".to_string(),
                    bearer_format: Some("JWT".to_string()),
                    flows: None,
                    open_id_connect_url: None,
                },
            )),
        );
        Self {
            openapi: SwaggerVersion::V300,
            info: InfoObject {
                title: title.to_owned(),
                version: version.to_owned(),
                description: Some(description.to_owned()),
                terms_of_service: None,
                contact: None,
                license: None,
            },
            servers: Some(new_servers),
            paths: HashMap::new(),
            // use the components add the schemas
            components: ComponentsObject {
                schemas: Some(content_map),
                callbacks: None,
                examples: None,
                headers: None,
                links: None,
                request_bodies: None,
                responses: None,
                parameters: None,
                security_schemes: Some(auth),
            },

            security: None,
            tags: None,
            external_docs: None,
        }
    }

    pub fn add_route(
        self: &mut Self,
        secure: bool,
        tag: &str,
        method: &str,
        path: String,
        parameters: Option<Vec<ParameterObjectOrReferenceObject>>,
        request_body: Option<(&str, RequestBodyObject)>,
        responses: Vec<(HttpStatusCode, (&str, &str, serde_json::Value))>,
    ) {
        if !self.paths.contains_key(&path) {
            self.paths.insert(
                path.clone(),
                PathItemObject {
                    r#ref: None,
                    summary: None,
                    description: None,
                    get: None,
                    put: None,
                    post: None,
                    delete: None,
                    options: None,
                    head: None,
                    patch: None,
                    trace: None,
                    servers: None,
                    parameters: None,
                },
            );
        }

        // if path contains parameters {} or :id return:
        // parameters:
        // - name: id
        //   in: path
        //   description: User ID
        //   required: true
        //   schema:
        //     type: integer
        //     format: int64

        // generate the paramter object from url {}
        let mut new_parameter_objects: Option<Vec<Value>> = None;
        if path.contains("{") {
            let patterns: &[_] = &[':', '{', '}'];
            let split_paths: Vec<&str> = path.split('/').collect();
            for data_path in split_paths {
                if data_path.contains("{") {
                    let new_path_name = data_path.trim_matches(patterns);
                    let parmeter = json!(ParameterObject {
                        name: new_path_name.to_string(),
                        description: Some(format!("use {} parameter", &new_path_name)),
                        required: Some(true),
                        schema: Some(SchemaObjectOrReferenceObject::SchemaObject(Box::new(
                            json!({
                                "type": "string"
                            }),
                        ))),
                        allow_empty_value: None,
                        deprecated: None,
                        where_in: ParameterIn::Path,
                    });
                    match new_parameter_objects {
                        Some(mut data_parameter) => {
                            data_parameter.push(parmeter);
                            new_parameter_objects = Some(data_parameter)
                        }
                        None => new_parameter_objects = Some(vec![parmeter]),
                    };
                }
            }
        }

        let path_object = self.paths.get_mut(&path).unwrap();

        let mut responses_per_http_status_codes = HashMap::new();
        for (status_code, (description, title, mut value)) in responses {
            let mut content_map = HashMap::new();

            let mut new_schema;
            if title == "String".to_string() {
                new_schema = Some(SchemaObjectOrReferenceObject::SchemaObject(Box::new(
                    value.clone(),
                )))
            } else {
                new_schema = Some(SchemaObjectOrReferenceObject::SchemaObject(Box::new(
                    json!({ "$ref": format!("#/components/schemas/{}", title) }),
                )))
            };

            content_map.insert(
                "application/json".to_owned(),
                MediaTypeObject {
                    // schema: Some(SchemaObjectOrReferenceObject::SchemaObject(Box::new(value))),
                    // use $ref components/schemas
                    //  format!("#/components/schemas/{}", value.get("title").unwrap())
                    schema: new_schema,
                    example: None,
                    examples: None,
                    encoding: None,
                },
            );
            responses_per_http_status_codes.insert(
                status_code,
                ResponseObjectOrReferenceObject::ResponseObject(Box::new(ResponseObject {
                    description: description.to_owned(),
                    headers: None,
                    content: Some(content_map),
                    links: None,
                })),
            );
        }

        let request_body = match request_body {
            Some(mut rq) => {
                // add $ref schemas
                let mut comp_map = HashMap::new();
                let mut new_schema;
                if rq.0 == "String".to_string() {
                    new_schema = Some(SchemaObjectOrReferenceObject::SchemaObject(Box::new(
                        json! ({
                            "title": "sting",
                            "type": "string"
                        }),
                    )))
                } else {
                    new_schema = Some(SchemaObjectOrReferenceObject::SchemaObject(Box::new(
                        json!({ "$ref": format!("#/components/schemas/{}", rq.0) }),
                    )))
                };
                comp_map.insert(
                    "application/json".to_owned(),
                    MediaTypeObject {
                        // schema: Some(SchemaObjectOrReferenceObject::SchemaObject(Box::new(value))),
                        // use $ref components/schemas
                        //  format!("#/components/schemas/{}", value.get("title").unwrap())
                        schema: new_schema,
                        example: None,
                        examples: None,
                        encoding: None,
                    },
                );
                rq.1.content = comp_map;
                Some(RequestBodyObjectOrReferenceObject::RequestBodyObject(
                    Box::new(rq.1),
                ))
            }
            None => None,
        };
        let mut sec_map = HashMap::new();
        sec_map.insert("bearerAuth".to_string(), json!([]));
        

        let operation_object = OperationObject {
            responses: ResponsesObject {
                default: None,
                responses_per_http_status_codes: Some(responses_per_http_status_codes),
            },
            tags: Some(vec![tag.to_string()]),
            summary: None,
            description: None,
            external_docs: None,
            operation_id: None,
            // parameters,
            parameters: new_parameter_objects,
            request_body,
            callbacks: None,
            deprecated: None,
            security: if secure { Some(vec![sec_map]) } else { None },
            servers: None,
        };

        match method {
            "GET" => path_object.get = Some(operation_object),
            "POST" => path_object.post = Some(operation_object),
            "PATCH" => path_object.patch = Some(operation_object),
            "DELETE" => path_object.delete = Some(operation_object),
            "PUT" => path_object.put = Some(operation_object),
            _ => unimplemented!("Unknown method: Send a PR!"),
        }
    }
}
