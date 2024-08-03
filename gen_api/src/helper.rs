use std::collections::{HashMap, VecDeque};

use scraper::Html;
use tera::{Context, Tera};

use crate::model::{ContentSchema, InnerSchema};

pub fn convert_to_desc(desc: &str) -> String {
    desc.split('\n')
        .map(|x| x.trim())
        .filter(|x| x != &"///")
        .filter(|x| !x.is_empty())
        .map(|x| format!("/// {}", x))
        .collect::<Vec<_>>()
        .join("\n///\n")
}

pub fn convert_simple_type(
    schema: &InnerSchema,
    suffix: &str,
    is_req_body: bool,
) -> Result<String, String> {
    let type_name = schema.type_field.as_str();
    let mut type_name = match type_name {
        "string" => "String".to_string(),
        "int" => "i64".to_string(),
        "boolean" => "bool".to_string(),
        "integer" => "i64".to_string(),
        "number" => "f64".to_string(),
        "map" => "std::collections::HashMap<String, serde_json::Value>".to_string(),
        "array" => format!(
            "Vec<{}>",
            heck::AsUpperCamelCase(&schema.name).to_string() + suffix
        ),
        _ => {
            if is_list(schema) {
                change_to_list(schema, suffix, is_req_body)
            } else {
                return Err(format!("unsupported type: {}", type_name));
            }
        }
    };
    if is_needed(schema, is_req_body) == Some(false) {
        type_name = format!("Option<{}>", type_name);
    }
    Ok(type_name)
}

pub fn convert_type(schema: &InnerSchema, suffix: &str, is_req_body: bool) -> String {
    let type_name = schema.type_field.as_str();
    if type_name == "object" {
        let mut type_name = heck::AsUpperCamelCase(&schema.name).to_string() + suffix;
        if schema.items.is_none()
            && (schema.properties.is_none() || schema.properties.as_ref().unwrap().is_empty())
        {
            // https://open.feishu.cn/document/server-docs/docs/bitable-v1/advanced-permission/app-role/update
            type_name = "serde_json::Value".to_string();
        } else if let Some(object_name) = schema.object_name.as_ref() {
            type_name = heck::AsUpperCamelCase(object_name).to_string() + suffix;
        }
        if is_needed(schema, is_req_body) == Some(false) {
            type_name = format!("Option<{}>", type_name);
        }
        return type_name;
    }

    match convert_simple_type(schema, suffix, is_req_body) {
        Ok(x) => x,
        Err(_) => {
            let mut type_name = heck::AsUpperCamelCase(type_name).to_string() + suffix;
            if is_needed(schema, is_req_body) == Some(false) {
                type_name = format!("Option<{}>", type_name);
            }
            type_name
        }
    }
}

pub fn get_schema_desc(param: &InnerSchema) -> String {
    let mut desc = vec![];

    if let Some(x) = param.description.as_ref() {
        desc.push(x.clone())
    }

    if let Some(needed) = param.required {
        desc.push(format!(
            "**是否必填**: {}",
            if needed { "是" } else { "否" }
        ));
    }

    if let Some(example) = param.example.as_ref() {
        desc.push(format!("**示例值**: \"{}\"", example));
    }

    if let Some(opt) = param.options.as_ref() {
        if !opt.is_empty() {
            let mut opts = vec![];
            for op in opt {
                let name = op.name.clone();
                let desc = get_opt_enum(&op.description);

                opts.push(format!("`{}`: {}", name, desc));
            }
            desc.push(format!("**可选值**:\n{}", opts.join("\n")));
        }
    }

    if let (Some(minn), Some(maxn)) = (&param.min_length, &param.max_length) {
        desc.push(format!(
            "**数据校验规则**：\n- **长度范围**: `{}` 字符- `{}` 字符",
            minn, maxn
        ));
    } else if let Some(minn) = &param.min_length {
        desc.push(format!(
            "**数据校验规则**：\n- **最小长度**: `{}` 字符",
            minn
        ));
    } else if let Some(maxn) = &param.max_length {
        desc.push(format!(
            "**数据校验规则**：\n- **最大长度**: `{}` 字符",
            maxn
        ));
    }

    convert_to_desc(&desc.join("\n"))
}

fn get_opt_enum(desc: &str) -> String {
    let desc = Html::parse_fragment(desc);
    let mut res = "".to_string();
    for child in desc.root_element().descendants() {
        if let Some(text) = child.value().as_text() {
            res.push_str(&(text.trim().to_string() + "\n"));
        } else if let Some(element) = child.value().as_element() {
            match element.name() {
                "md-perm" => {
                    let support_app_types = element.attr("support_app_types").expect(
                        "md-perm must have support_app_types
                        ",
                    );
                    let support_app_types = match support_app_types {
                        "custom" => "仅自建应用",
                        _ => panic!("unknown support_app_types: {}", support_app_types),
                    };
                    res.push_str(&format!("(**{}**) ", support_app_types));
                }
                _ => {
                    continue;
                }
            }
        } else {
            panic!("unknown child type: {:?}", child.value());
        }
    }
    res
}

pub fn is_list(schema: &InnerSchema) -> bool {
    let type_name = schema.type_field.as_str();
    type_name.ends_with("[]") || type_name.ends_with("\\[\\]") || type_name == "array"
}

pub fn change_to_list(schema: &InnerSchema, suffix: &str, is_req_body: bool) -> String {
    let type_name = schema.type_field.as_str();
    if type_name.ends_with("[]") {
        let type_name = type_name
            .strip_suffix(&type_name[type_name.len() - 2..])
            .unwrap();
        format!(
            "Vec<{}>",
            match convert_simple_type(schema, suffix, is_req_body) {
                Ok(x) => x,
                Err(_) => {
                    heck::AsUpperCamelCase(&type_name).to_string()
                }
            }
        )
    } else if type_name.ends_with("\\[\\]") {
        let type_name = type_name
            .strip_suffix(&type_name[type_name.len() - 4..])
            .unwrap();
        format!(
            "Vec<{}>",
            match convert_simple_type(schema, suffix, is_req_body) {
                Ok(x) => x,
                Err(_) => {
                    heck::AsUpperCamelCase(&type_name).to_string()
                }
            }
        )
    } else {
        panic!("unsupported type: {}", type_name)
    }
}

#[derive(Debug, Clone)]
pub struct ParamData {
    pub origin_name: String,
    pub field_name: String,
    pub type_name: String,
    pub is_needed: Option<bool>,
    pub is_list: bool,
    pub desc: String,
}

const TEMPLATE: &str = r#"
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
#[serde(default)]
pub struct {{ subTypeUpperName }} {
    {{ fields }}
}
"#;

macro_rules! field_attr {() => ("{}\n#[serde(rename=\"{}\",deserialize_with = \"crate::utils::serde_helper::null_to_default\")] pub {}: {},")}

pub fn parse_body_schema(
    schema: &ContentSchema,
    suffix: &str,
    is_req_body: bool,
) -> (Vec<ParamData>, Vec<String>) {
    let mut top = vec![];

    let mut sub_types = VecDeque::new();
    for p in &schema.properties {
        let mut params = parse_schema_to_params(p, suffix, "body", is_req_body);

        if p.items.is_some()
            || p.properties
                .as_ref()
                .map(|x| !x.is_empty())
                .unwrap_or(false)
        {
            sub_types.push_back(p);
        } else if p.type_field == "object" {
            params.type_name = "()".into();
        }

        top.push(params);
    }

    let mut sub_types_str = HashMap::new();
    while let Some(sub_type) = sub_types.pop_front() {
        let mut context = Context::new();
        let mut fields = vec![];
        if let Some(properties) = &sub_type.properties {
            for p in properties {
                let schema = parse_schema_to_params(p, suffix, "body", is_req_body);
                fields.push(format!(
                    field_attr!(),
                    schema.desc, schema.origin_name, schema.field_name, schema.type_name
                ));
                if p.items.is_some()
                    || p.properties
                        .as_ref()
                        .map(|x| !x.is_empty())
                        .unwrap_or(false)
                {
                    sub_types.push_back(p);
                }
            }
        }
        if let Some(item) = &sub_type.items {
            let schema = parse_schema_to_params(item, suffix, "body", is_req_body);
            fields.push(format!(
                field_attr!(),
                schema.desc, schema.origin_name, schema.field_name, schema.type_name
            ));
            if item.items.is_some()
                || item
                    .properties
                    .as_ref()
                    .map(|x| !x.is_empty())
                    .unwrap_or(false)
            {
                sub_types.push_back(item.as_ref());
            }
        }
        if sub_type.items.is_none() {
            let struct_name = if let Some(object_name) = sub_type.object_name.as_ref() {
                heck::AsUpperCamelCase(object_name.as_str()).to_string() + suffix
            } else {
                heck::AsUpperCamelCase(sub_type.name.as_str()).to_string() + suffix
            };
            context.insert("subTypeUpperName", &struct_name);
            context.insert("fields", &fields.join("\n"));
            let res = Tera::one_off(TEMPLATE, &context, false).unwrap();
            let len = sub_types_str.len();
            sub_types_str
                .entry(struct_name)
                .or_insert_with(|| (res, len));
        }
    }

    let mut sub_types_str = sub_types_str
        .into_iter()
        .map(|(_, (x, y))| (x, y))
        .collect::<Vec<_>>();
    sub_types_str.sort_by_key(|x| x.1);

    (top, sub_types_str.into_iter().map(|x| x.0).collect())
}

pub fn parse_schema_to_params(
    p: &InnerSchema,
    suffix: &str,
    dup_field_prefix: &str,
    is_req_body: bool,
) -> ParamData {
    let name = p.name.clone();
    let field_name = if name == "type" {
        dup_field_prefix.to_string() + "_type"
    } else {
        heck::AsSnakeCase(&name).to_string()
    };

    let is_needed = is_needed(p, is_req_body);

    let mut type_name = convert_type(p, suffix, is_req_body);
    if suffix == "Resp" && ["\\-", "-", "\\\\-"].contains(&p.type_field.as_str()) {
        if p.items.is_none() && p.properties.is_none() {
            type_name = "()".to_string();
        } else {
            type_name = heck::AsUpperCamelCase(&name).to_string() + suffix;
        }
    }
    if let Some(item) = &p.items {
        let schema = parse_schema_to_params(item, suffix, dup_field_prefix, is_req_body);
        type_name = format!("Vec<{}>", schema.type_name);
    }

    let desc = get_schema_desc(p);

    ParamData {
        origin_name: name,
        field_name,
        type_name,
        is_needed,
        is_list: is_list(p),
        desc,
    }
}

pub fn is_needed(p: &InnerSchema, is_req_body: bool) -> Option<bool> {
    let mut is_needed = p.required;
    if is_req_body && is_needed.is_none() {
        is_needed = Some(false);
    }
    if p.description
        .as_ref()
        .map(|x| x.contains("非必填") || x.contains("选填"))
        .unwrap_or(false)
    {
        is_needed = Some(false);
    }
    if p.example
        .as_ref()
        .map(|x| x.contains("非必填") || x.contains("选填"))
        .unwrap_or(false)
    {
        is_needed = Some(false);
    }
    is_needed
}
