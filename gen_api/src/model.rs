use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Resp {
    pub code: i64,
    pub msg: String,
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Data {
    #[serde(rename = "fullPath")]
    pub full_path: String,
    #[serde(rename = "type")]
    pub type_field: String,
    // pub content: String,
    #[serde(rename = "updateTime")]
    pub update_time: i64,
    #[serde(rename = "directoryId")]
    pub directory_id: String,
    pub name: String,
    #[serde(rename = "allVisible")]
    pub all_visible: bool,
    pub schema: Option<Schema>,
    #[serde(rename = "redirectUri")]
    pub redirect_uri: String,
    pub id: String,
    #[serde(rename = "templateName")]
    pub template_name: String,
    pub title: String,
    #[serde(rename = "visibleTag")]
    pub visible_tag: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]

pub struct Schema {
    pub title: String,
    pub description: String,
    pub tips: Vec<Tip>,
    #[serde(rename = "apiSchema")]
    pub api_schema: ApiSchema,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Tip {
    #[serde(rename = "tipLevel")]
    pub tip_level: String,
    #[serde(rename = "tipInfo")]
    pub tip_info: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApiSchema {
    pub parameters: Vec<Parameter>,
    #[serde(rename = "endingContent")]
    pub ending_content: String,
    pub id: String,
    pub domain: String,
    #[serde(rename = "requestBody")]
    pub request_body: RequestBody,
    #[serde(rename = "responseBeforeContent")]
    pub response_before_content: String,
    pub path: String,
    #[serde(rename = "httpMethod")]
    pub http_method: String,
    pub responses: Responses,
    pub security: Security,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Parameter {
    #[serde(rename = "in")]
    pub in_field: String,
    pub schema: InnerSchema,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Opt {
    pub name: String,
    pub value: String,
    pub description: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RequestBody {
    pub content: Content,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Content {
    #[serde(rename = "application/json")]
    pub application_json: Option<ApplicationJson>,
    #[serde(rename = "multipart/form-data")]
    pub req_bin: Option<ApplicationJson>,
    #[serde(rename = "*/*")]
    pub resp_bin: Option<ApplicationJson>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ApplicationJson {
    pub schema: ContentSchema,
    pub example: Option<String>,
    // pub examples: Examples,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ContentSchema {
    pub properties: Vec<InnerSchema>,
    pub options: Vec<Opt>,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct InnerSchema {
    pub description: Option<String>,
    #[serde(rename = "type")]
    pub type_field: String,
    pub example: Option<String>,
    pub options: Option<Vec<Opt>>,
    pub name: String,
    pub required: Option<bool>,
    #[serde(rename = "minLength")]
    pub min_length: Option<String>,
    #[serde(rename = "maxLength")]
    pub max_length: Option<String>,
    // 当 type 为 object 时，properties 为对象的 schema
    pub properties: Option<Vec<InnerSchema>>,
    #[serde(rename = "objectName")]
    pub object_name: Option<String>,
    // 当 type 为 array 时，items 为数组元素的 schema
    pub items: Option<Box<InnerSchema>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Responses {
    #[serde(rename = "200")]
    pub n200: Code200,
    #[serde(rename = "errorCodeMapping")]
    pub error_code_mapping: Vec<ErrorCodeMapping>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Code200 {
    pub content: Content,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ErrorCodeMapping {
    #[serde(rename = "errorCode")]
    pub error_code: i64,
    #[serde(rename = "statusCode")]
    pub status_code: i64,
    pub description: String,
    #[serde(rename = "troubleShootingSuggestion")]
    pub trouble_shooting_suggestion: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Security {
    #[serde(rename = "scopeInfos")]
    pub scope_infos: Vec<ScopeInfo>,
    #[serde(rename = "rateLimitInfo")]
    pub rate_limit_info: RateLimitInfo,
    #[serde(rename = "requiredScopes")]
    pub required_scopes: Vec<String>,
    #[serde(rename = "fieldRequiredScopes")]
    pub field_required_scopes: Vec<Value>,
    #[serde(rename = "supportedAccessToken")]
    pub supported_access_token: Vec<String>,
    #[serde(rename = "supportedAppTypes")]
    pub supported_app_types: Vec<String>,
    #[serde(rename = "rateLimitTier")]
    pub rate_limit_tier: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct ScopeInfo {
    pub name: String,
    pub description: String,
    #[serde(rename = "supportedAppTypes")]
    pub supported_app_types: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct RateLimitInfo {
    #[serde(rename = "docUrl")]
    pub doc_url: String,
    pub description: String,
}
