use std::{collections::HashMap, env, fs::File, io::Write};

use lazy_static::lazy_static;
use model::Schema;
use rust_format::{Formatter, RustFmt};
use serde::{Deserialize, Serialize};
use tera::{Context, Tera};

mod helper;
mod model;
use helper::*;

#[derive(Debug, serde::Deserialize, Clone, Serialize)]
struct ApiData {
    #[serde(rename = "doc链接")]
    url: String,
    #[serde(rename = "serviceName")]
    service_name: String,
    #[serde(rename = "funcName")]
    func_name: String,
    #[serde(rename = "doc链接版本")]
    _doc_url_type: String,
}

#[derive(Debug, serde::Deserialize, Clone, Serialize)]
struct TestDataFix {
    #[serde(rename = "service")]
    service: String,
    #[serde(rename = "func")]
    func_name: String,
    #[serde(rename = "req")]
    req: String,
    #[serde(rename = "resp")]
    resp: String,
}

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    println!("args: {:?}", args);

    if args[3] == "csv" {
        gen_api_from_csv(args[4].clone()).await;
        return;
    }

    assert_eq!(args.len(), 6);
    println!("{}", gen_api(&args[3], &args[4], &args[5]).await);
}

async fn gen_api_from_csv(csv_path: String) {
    println!("gen api from csv");

    let path = std::path::Path::new("./gen");

    if !path.exists() {
        std::fs::create_dir(path).expect("create dir error");
    } else {
        std::fs::remove_dir_all(path).expect("remove dir error");
        std::fs::create_dir(path).expect("create dir error");
    }

    let tera = Tera::new("./**.tpl").expect("load dir error");

    let mut mod_file = File::create_new(path.join("mod.rs")).expect("create mod file error");
    let mut created_service = HashMap::new();

    let mut rdr = csv::Reader::from_path(&csv_path).expect("read csv file error");
    for result in rdr.deserialize() {
        let data: ApiData = result.expect("parse csv data error");
        let data = ApiData {
            url: data.url.trim().to_string(),
            service_name: heck::AsSnakeCase(data.service_name).to_string(),
            func_name: heck::AsSnakeCase(data.func_name).to_string(),
            _doc_url_type: data._doc_url_type,
        };

        let service_upper_camel_case = heck::AsUpperCamelCase(&data.service_name).to_string();
        let service_snake_case = heck::AsSnakeCase(&data.service_name).to_string();
        let service_dir = path.join(&data.service_name);

        if !created_service.contains_key(&data.service_name) {
            let mut context = Context::new();

            context.insert("service_snake_case", &service_snake_case);
            context.insert("serviceUpperCamelCase", &service_upper_camel_case);
            let result = tera
                .render("service.tpl", &context)
                .expect("gen mod file false");

            mod_file
                .write_all(format!("#[cfg_attr(docsrs, doc(cfg(feature = \"{}\")))]\n#[cfg(feature = \"{}\")]\npub mod {};\n",data.service_name,data.service_name, data.service_name).as_bytes())
                .expect("write mod error");

            std::fs::create_dir(service_dir.clone()).expect("create dir error");
            let service_mod_dir = service_dir.clone().join("mod.rs");
            let mut service_mod_file =
                File::create_new(service_mod_dir).expect("create mod file error");
            service_mod_file
                .write_all(result.as_bytes())
                .expect("write service mod filed");

            created_service.insert(data.service_name.clone(), service_mod_file);
        }

        let res = gen_api(&data.url, &data.service_name, &data.func_name).await;
        created_service
            .get_mut(&data.service_name)
            .unwrap()
            .write_all(format!("pub mod {};\n", data.func_name).as_bytes())
            .expect("write mod error");

        let p = service_dir.join(format!("{}.rs", data.func_name));
        std::fs::write(p, res).expect("write file error");
        if !USE_STORE {
            std::thread::sleep(std::time::Duration::from_millis(200));
        }
    }
    if STORE_API_DATA {
        unsafe {
            let mut wtr = csv::Writer::from_path("./api_data.csv").expect("write csv file error");
            for record in API_DATA_CACHE.iter() {
                wtr.serialize(record).expect("write csv record error");
            }
            wtr.flush().expect("flush csv error");
        }
    }
    unsafe {
        if !NOT_NEW_API.is_empty() {
            let mut wtr =
                csv::Writer::from_path("./not_new_api.csv").expect("write csv file error");
            for record in NOT_NEW_API.iter() {
                wtr.serialize(record).expect("write csv record error");
            }
            wtr.flush().expect("flush csv error");
        }
    }
    unsafe {
        println!("err cnt: {}", WRONG_GEN_CNT);
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Record {
    #[serde(rename = "doc链接")]
    url: String,
    #[serde(rename = "serviceName")]
    service_name: String,
    #[serde(rename = "funcName")]
    func_name: String,
    #[serde(rename = "api_data")]
    data: String,
}

lazy_static! {
    static ref API_DATA: Option<HashMap<String, Record>> = {
        if !USE_STORE {
            return None;
        }
        let mut map = HashMap::new();
        let rdr = csv::Reader::from_path("./api_data.csv");
        if rdr.is_err() {
            return None;
        }
        for result in rdr.unwrap().deserialize() {
            let data: Record = result.expect("parse csv data error");
            map.insert(data.url.clone(), data);
        }
        Some(map)
    };
    static ref TEST_DATA_FIX: Option<HashMap<(String, String), TestDataFix>> = {
        let mut map = HashMap::new();
        let rdr = csv::Reader::from_path("./test_data_fix.csv");
        if rdr.is_err() {
            return None;
        }
        for result in rdr.unwrap().deserialize() {
            let data: TestDataFix = result.expect("parse csv data error");
            map.insert((data.service.clone(), data.func_name.clone()), data);
        }
        Some(map)
    };
}

static mut API_DATA_CACHE: Vec<Record> = vec![];
const USE_STORE: bool = true;
const STORE_API_DATA: bool = true;
static mut NOT_NEW_API: Vec<ApiData> = vec![];
static mut WRONG_GEN_CNT: i32 = 0;

async fn gen_api(url_str: &str, service_name: &str, func_name: &str) -> String {
    println!("url == {}", url_str);
    let url = url::Url::parse(url_str).expect("parse url failed, please check url");

    let api = reqwest::Url::parse_with_params(
        "https://open.feishu.cn/document_portal/v1/document/get_detail",
        [("fullPath", url.path().trim_start_matches("/document"))],
    )
    .expect("parse api url error");
    println!("api == {}", api);

    let data = if let Some(store) = API_DATA.as_ref().and_then(|x| x.get(url_str)) {
        store.data.clone()
    } else {
        loop {
            let res = reqwest::get(api.clone()).await.map(|x| x.text());
            if let Ok(data) = res {
                if let Ok(data) = data.await {
                    break data;
                }
            }
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    };

    if STORE_API_DATA {
        unsafe {
            API_DATA_CACHE.push(Record {
                url: url_str.to_string(),
                service_name: service_name.to_string(),
                func_name: func_name.to_string(),
                data: data.clone(),
            });
        }
    }

    let data = data.replace("/ssl:ttdoc/", "https://open.feishu.cn/document/");

    let data: model::Resp = serde_json::from_str(&data).expect("parse data failed");

    if data.data.schema.is_none() {
        unsafe {
            NOT_NEW_API.push(ApiData {
                url: url_str.to_string(),
                service_name: service_name.to_string(),
                func_name: func_name.to_string(),
                _doc_url_type: "".to_string(),
            });
        }
        return format!(
            "/*! api dose not have schema, might need manually define type\nurl: <{}>\napi: <{}>\n */",
            url_str, api
        );
    }

    let service = service_name;
    let service_snake_case = heck::AsSnakeCase(service).to_string();
    let service_upper_camel_case = heck::AsUpperCamelCase(service).to_string();
    let func_name = heck::AsSnakeCase(func_name).to_string();
    let upper_camel_func = heck::AsUpperCamelCase(&func_name).to_string();
    let req_type = upper_camel_func.clone() + "Req";
    let resp_type = upper_camel_func.clone() + "Resp";

    let mut context = Context::new();
    context.insert("service", &service);
    context.insert("service_snake_case", &service_snake_case);
    context.insert("serviceUpperCamelCase", &service_upper_camel_case);
    context.insert("func_name", &func_name);
    context.insert("UpperCamelFunc", &upper_camel_func);
    context.insert("req_type", &req_type);
    context.insert("resp_type", &resp_type);
    context.insert("doc_path", &url.path());
    context.insert("api_url", &api.to_string());

    let update_time = chrono::DateTime::from_timestamp_millis(data.data.update_time)
        .expect("parse update time failed")
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, false);
    context.insert("update_time", &update_time);
    context.insert("fullPath", &data.data.full_path);

    parse_schema(data.data.schema.unwrap(), &mut context);

    let tera = Tera::new("./*.tpl").expect("load dir error");

    let template = context
        .get("tpl")
        .expect("missing tpl")
        .as_str()
        .expect("not str");

    let result = tera.render(template, &context).expect("gen api false");

    RustFmt::default()
        .format_str(result.clone())
        .unwrap_or_else(|e| panic!("result = {}\nerror: {}", result, e))
}

fn parse_schema(schema: Schema, context: &mut Context) {
    context.insert("req_fields_path", "");
    context.insert("req_fields_query", "");
    context.insert("req_fields_body", "");
    context.insert("req_fields_body_sub_type", "");
    context.insert("resp_fields", "");
    context.insert("resp_fields_sub_type", "");
    context.insert("req_test_data", "\"{}\"");
    context.insert("resp_test_data", "\"{}\"");
    context.insert("req_test_type_body", "()");
    context.insert("tpl", "api.tpl");

    parse_desc(schema.title.clone(), &schema, context);
    parse_method_and_path(&schema, context);
    parse_auth(&schema, context);
    parse_req_path(&schema, context);
    parse_req_query(&schema, context);
    parse_req_body(&schema, context);
    parse_req_stream(&schema, context);
    parse_resp(&schema, context);
    parse_resp_stream(&schema, context);

    parse_req_test_data(&schema, context);
    parse_resp_test_data(&schema, context);
}

fn parse_desc(name: String, schema: &Schema, context: &mut Context) {
    let desc = convert_to_desc(&(format!("## {}\n", name) + &schema.description));
    context.insert("description", &desc);

    let tips = schema
        .tips
        .iter()
        .map(|tip| tip.tip_info.trim())
        .filter(|x| !x.is_empty())
        .map(convert_to_desc)
        .collect::<Vec<_>>()
        .join("\n///\n");
    context.insert("tips", &tips);
}

fn parse_method_and_path(schema: &Schema, context: &mut Context) {
    context.insert(
        "method",
        &("http::Method::".to_owned() + &schema.api_schema.http_method.to_uppercase()),
    );
    context.insert("path", &schema.api_schema.path);
}

fn parse_auth(schema: &Schema, context: &mut Context) {
    let mut auth_type = vec![];
    for auth in &schema.api_schema.security.supported_access_token {
        match auth.as_str() {
            "tenant_access_token" => {
                auth_type.push("need_tenant_access_token: true,");
            }
            "app_access_token" => {
                auth_type.push("need_app_access_token: true,");
            }
            "user_access_token" => {
                auth_type.push("need_user_access_token: true,");
            }
            _ => {
                panic!("unsupported auth type: {}", auth);
            }
        }
    }
    context.insert("auth", &auth_type.join("\n"));
}

fn parse_req_path(schema: &Schema, context: &mut Context) {
    let path_p = schema
        .api_schema
        .parameters
        .iter()
        .filter(|x| x.in_field == "path")
        .map(|x| x.clone().schema)
        .collect::<Vec<_>>();

    let mut all_path_p = vec![];
    for p in path_p {
        let param = parse_schema_to_params(&p, "", "path", false);
        all_path_p.push(format!(
            "{}\n#[api(kind=\"path\",name=\"{}\")] pub {}: {}, ",
            param.desc, param.origin_name, param.field_name, param.type_name,
        ));
    }
    context.insert("req_fields_path", &all_path_p.join("\n"));
}

fn parse_req_query(schema: &Schema, context: &mut Context) {
    let query_p = schema
        .api_schema
        .parameters
        .iter()
        .filter(|x| x.in_field == "query")
        .map(|x| x.clone().schema)
        .collect::<Vec<_>>();

    let mut all_query_p = vec![];
    for p in query_p {
        if p.name.is_empty() {
            // case: https://open.feishu.cn/document/server-docs/docs/bitable-v1/app/create?appId=cli_a03454f9c2789013
            continue;
        }

        let param = parse_schema_to_params(&p, "", "query", false);

        let mut v_type = "var";
        if param.is_list {
            v_type = "list";
        }

        let option_tag = if param.is_needed.unwrap_or(true) {
            "false"
        } else {
            "true"
        };

        all_query_p.push(format!(
            "{}\n#[api(kind=\"query\",name=\"{}\",v_type=\"{}\",option=\"{}\")] pub {}: {}, ",
            param.desc, param.origin_name, v_type, option_tag, param.field_name, param.type_name,
        ));
    }
    context.insert("req_fields_query", &all_query_p.join("\n"));
}

fn parse_req_body(schema: &Schema, context: &mut Context) {
    let json = &schema.api_schema.request_body.content.application_json;
    if json.is_none() {
        return;
    }
    let schema = &json.as_ref().unwrap().schema;

    let (top, sub_types) = parse_body_schema(schema, "SubReq", true);
    let mut all_body_p = vec![];
    for p in top {
        all_body_p.push(format!(
            "{}\n#[api(kind=\"body\",name=\"{}\")] pub {}: {}, ",
            p.desc, p.origin_name, p.field_name, p.type_name
        ));
    }
    context.insert("req_fields_body", &all_body_p.join("\n"));
    context.insert("req_fields_body_sub_type", &sub_types.join("\n"));
}

fn parse_req_stream(schema: &Schema, context: &mut Context) {
    let json = &schema.api_schema.request_body.content.req_bin;
    if json.is_none() {
        return;
    }
    context.insert("tpl", "upload_api.tpl");
    let schema = &json.as_ref().unwrap().schema;

    let (top, sub_types) = parse_body_schema(schema, "SubReq", true);
    if !sub_types.is_empty() {
        panic!("stream body should not have sub type");
    }
    let mut all_body_p = vec![];
    for p in top {
        if p.field_name == "data" || p.field_name == "file" {
            continue;
        }
        let option_tag = if p.is_needed.unwrap_or(true) {
            "false"
        } else {
            "true"
        };
        all_body_p.push(format!(
            "{}\n#[api(kind=\"stream\",name=\"{}\", option=\"{}\")] pub {}: {}, ",
            p.desc, p.origin_name, option_tag, p.field_name, p.type_name
        ));
    }
    context.insert("req_fields_stream", &all_body_p.join("\n"));
}

fn parse_resp(schema: &Schema, context: &mut Context) {
    let json = &schema.api_schema.responses.n200.content.application_json;
    if json.is_none() {
        return;
    }
    let schema = &json.as_ref().unwrap().schema;

    let (top, sub_types) = parse_body_schema(schema, "SubResp", false);
    let mut all_body_p = vec![];
    for p in top {
        if ["code", "msg", "err"].contains(&p.origin_name.as_str()) {
            continue;
        }
        all_body_p.push(format!(
            "{}\n#[serde(rename=\"{}\", deserialize_with = \"crate::utils::serde_helper::null_to_default\")] pub {}: {}, ",
            p.desc, p.origin_name, p.field_name, p.type_name
        ));
    }
    context.insert("resp_fields", &all_body_p.join("\n"));
    context.insert("resp_fields_sub_type", &sub_types.join("\n"));
}

fn parse_resp_stream(schema: &Schema, context: &mut Context) {
    let json = &schema.api_schema.responses.n200.content.resp_bin;
    if json.is_none() {
        return;
    }
    context.insert("tpl", "download_api.tpl");
}

fn parse_req_test_data(schema: &Schema, context: &mut Context) {
    let service = context.get("service_snake_case").unwrap().as_str().unwrap();
    let func_name = context.get("func_name").unwrap().as_str().unwrap();
    if let Some(data) = TEST_DATA_FIX
        .as_ref()
        .and_then(|x| x.get(&(service.to_string(), func_name.to_string())))
    {
        if !data.req.is_empty() {
            let req_type = context.get("req_type").unwrap().as_str().unwrap();
            context.insert("req_test_type_body", &format!("super::{}Body", req_type));
            context.insert("req_test_data", &format!("r#\"{}\"#", &data.req));
            return;
        }
    }

    if let Some(example) = schema
        .api_schema
        .request_body
        .content
        .application_json
        .as_ref()
        .and_then(|x| x.example.as_ref())
    {
        if context
            .get("req_fields_body")
            .unwrap()
            .as_str()
            .unwrap()
            .is_empty()
        {
            return;
        }
        let req_type = context.get("req_type").unwrap().as_str().unwrap();
        context.insert("req_test_type_body", &format!("super::{}Body", req_type));
        context.insert("req_test_data", &format!("r#\"{}\"#", example));
    }
}

fn parse_resp_test_data(schema: &Schema, context: &mut Context) {
    let service = context.get("service_snake_case").unwrap().as_str().unwrap();
    let func_name = context.get("func_name").unwrap().as_str().unwrap();
    if let Some(data) = TEST_DATA_FIX
        .as_ref()
        .and_then(|x| x.get(&(service.to_string(), func_name.to_string())))
    {
        if !data.resp.is_empty() {
            context.insert("resp_test_data", &format!("r#\"{}\"#", &data.resp));
            return;
        }
    }

    if let Some(example) = schema
        .api_schema
        .responses
        .n200
        .content
        .application_json
        .as_ref()
        .and_then(|x| x.example.as_ref())
    {
        context.insert("resp_test_data", &format!("r#\"{}\"#", example));
    }
}
