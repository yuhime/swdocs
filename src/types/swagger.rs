use serde::{Deserialize, Serialize};
use std::collections::HashMap;

impl PathItem {
    pub fn operations_mut(&mut self) -> Vec<&mut Operation> {
        let mut v = Vec::new();
        if let Some(o) = self.get.as_mut() {
            v.push(o);
        }
        if let Some(o) = self.post.as_mut() {
            v.push(o);
        }
        if let Some(o) = self.put.as_mut() {
            v.push(o);
        }
        if let Some(o) = self.patch.as_mut() {
            v.push(o);
        }
        if let Some(o) = self.delete.as_mut() {
            v.push(o);
        }
        if let Some(o) = self.head.as_mut() {
            v.push(o);
        }
        if let Some(o) = self.options.as_mut() {
            v.push(o);
        }
        if let Some(o) = self.trace.as_mut() {
            v.push(o);
        }
        v
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenApi {
    pub openapi: String,
    pub info: Info,

    #[serde(default)]
    pub servers: Vec<Server>,

    #[serde(default)]
    pub paths: HashMap<String, PathItem>,

    #[serde(default)]
    pub components: Components,

    #[serde(default)]
    pub security: Vec<HashMap<String, Vec<String>>>,

    #[serde(default)]
    pub tags: Vec<Tag>,

    #[serde(default)]
    pub external_docs: Option<ExternalDocs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Info {
    pub title: String,
    pub version: String,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub terms_of_service: Option<String>,

    #[serde(default)]
    pub contact: Option<Contact>,

    #[serde(default)]
    pub license: Option<License>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contact {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub email: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct License {
    pub name: String,
    #[serde(default)]
    pub identifier: Option<String>,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Server {
    pub url: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub variables: HashMap<String, ServerVariable>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerVariable {
    #[serde(default, rename = "enum")]
    pub enum_values: Vec<String>,
    #[serde(default)]
    pub default: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tag {
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub external_docs: Option<ExternalDocs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ExternalDocs {
    pub url: String,
    #[serde(default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PathItem {
    #[serde(default)]
    pub get: Option<Operation>,
    #[serde(default)]
    pub post: Option<Operation>,
    #[serde(default)]
    pub put: Option<Operation>,
    #[serde(default)]
    pub patch: Option<Operation>,
    #[serde(default)]
    pub delete: Option<Operation>,
    #[serde(default)]
    pub head: Option<Operation>,
    #[serde(default)]
    pub options: Option<Operation>,
    #[serde(default)]
    pub trace: Option<Operation>,

    #[serde(default)]
    pub summary: Option<String>,
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub parameters: Vec<Parameter>,
    #[serde(default)]
    pub servers: Vec<Server>,

    #[serde(rename = "$ref", default)]
    pub reference: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Operation {
    #[serde(default)]
    pub tags: Vec<String>,

    #[serde(default)]
    pub summary: Option<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub operation_id: Option<String>,

    #[serde(default)]
    pub parameters: Vec<Parameter>,

    #[serde(default)]
    pub request_body: Option<RequestBody>,

    #[serde(default)]
    pub responses: HashMap<String, Response>,

    #[serde(default)]
    pub deprecated: bool,

    #[serde(default)]
    pub security: Vec<HashMap<String, Vec<String>>>,

    #[serde(default)]
    pub servers: Vec<Server>,

    #[serde(default)]
    pub callbacks: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub external_docs: Option<ExternalDocs>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    #[serde(default)]
    pub name: Option<String>,

    #[serde(rename = "in", default)]
    pub location: Option<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub required: bool,

    #[serde(default)]
    pub deprecated: bool,

    #[serde(default)]
    pub allow_empty_value: Option<bool>,

    #[serde(default)]
    pub style: Option<String>,

    #[serde(default)]
    pub explode: Option<bool>,

    #[serde(default)]
    pub allow_reserved: Option<bool>,

    #[serde(default)]
    pub schema: Option<Schema>,

    #[serde(default)]
    pub example: Option<serde_json::Value>,

    #[serde(default)]
    pub examples: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub content: HashMap<String, MediaType>,

    #[serde(rename = "$ref", default)]
    pub reference: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct RequestBody {
    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub content: HashMap<String, MediaType>,

    #[serde(default)]
    pub required: bool,

    #[serde(rename = "$ref", default)]
    pub reference: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Response {
    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub headers: HashMap<String, Header>,

    #[serde(default)]
    pub content: HashMap<String, MediaType>,

    #[serde(default)]
    pub links: HashMap<String, serde_json::Value>,

    #[serde(rename = "$ref", default)]
    pub reference: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Header {
    #[serde(default)]
    pub description: Option<String>,
    #[serde(default)]
    pub required: bool,
    #[serde(default)]
    pub deprecated: bool,
    #[serde(default)]
    pub schema: Option<Schema>,
    #[serde(default)]
    pub content: HashMap<String, MediaType>,
    #[serde(rename = "$ref", default)]
    pub reference: Option<String>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct MediaType {
    #[serde(default)]
    pub schema: Option<Schema>,

    #[serde(default)]
    pub example: Option<serde_json::Value>,

    #[serde(default)]
    pub examples: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub encoding: HashMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Schema {
    #[serde(rename = "$ref", default)]
    pub reference: Option<String>,

    #[serde(rename = "type", default)]
    pub schema_type: Option<String>,

    #[serde(default)]
    pub format: Option<String>,

    #[serde(default)]
    pub title: Option<String>,

    #[serde(default)]
    pub description: Option<String>,

    #[serde(default)]
    pub default: Option<serde_json::Value>,

    #[serde(rename = "enum", default)]
    pub enum_values: Vec<serde_json::Value>,

    #[serde(rename = "const", default)]
    pub const_value: Option<serde_json::Value>,

    #[serde(default)]
    pub example: Option<serde_json::Value>,

    #[serde(default)]
    pub nullable: Option<bool>,

    #[serde(default)]
    pub deprecated: Option<bool>,

    #[serde(default)]
    pub read_only: Option<bool>,

    #[serde(default)]
    pub write_only: Option<bool>,

    #[serde(default)]
    pub properties: HashMap<String, Schema>,

    #[serde(default)]
    pub required: Vec<String>,

    #[serde(default)]
    pub additional_properties: Option<Box<Schema>>,

    #[serde(default)]
    pub min_properties: Option<u64>,

    #[serde(default)]
    pub max_properties: Option<u64>,

    #[serde(default)]
    pub items: Option<Box<Schema>>,

    #[serde(default)]
    pub prefix_items: Vec<Schema>,

    #[serde(default)]
    pub min_items: Option<u64>,

    #[serde(default)]
    pub max_items: Option<u64>,

    #[serde(default)]
    pub unique_items: Option<bool>,

    #[serde(default)]
    pub min_length: Option<u64>,

    #[serde(default)]
    pub max_length: Option<u64>,

    #[serde(default)]
    pub pattern: Option<String>,

    // Number
    #[serde(default)]
    pub minimum: Option<f64>,

    #[serde(default)]
    pub maximum: Option<f64>,

    #[serde(default)]
    pub exclusive_minimum: Option<bool>,

    #[serde(default)]
    pub exclusive_maximum: Option<bool>,

    #[serde(default)]
    pub multiple_of: Option<f64>,

    #[serde(default)]
    pub all_of: Vec<Schema>,

    #[serde(default)]
    pub any_of: Vec<Schema>,

    #[serde(default)]
    pub one_of: Vec<Schema>,

    #[serde(default)]
    pub not: Option<Box<Schema>>,

    #[serde(default)]
    pub discriminator: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Components {
    #[serde(default)]
    pub schemas: HashMap<String, Schema>,

    #[serde(default)]
    pub responses: HashMap<String, Response>,

    #[serde(default)]
    pub parameters: HashMap<String, Parameter>,

    #[serde(default)]
    pub examples: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub request_bodies: HashMap<String, RequestBody>,

    #[serde(default)]
    pub headers: HashMap<String, Header>,

    #[serde(default)]
    pub security_schemes: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub links: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub callbacks: HashMap<String, serde_json::Value>,

    #[serde(default)]
    pub path_items: HashMap<String, PathItem>,
}
