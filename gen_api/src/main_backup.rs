use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    env,
    fs::File,
    io::Write,
    rc::Rc,
};

use regex::Regex;
use rust_format::{Formatter, RustFmt};
use scraper::{selectable::Selectable, Html, Selector};
use tera::{Context, Tera};

mod model;

#[derive(Debug, serde::Deserialize, Clone)]
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

        let service_lower = heck::AsLowerCamelCase(&data.service_name).to_string();
        let service_dir = path.join(service_lower.clone());

        if !created_service.contains_key(&data.service_name) {
            let mut context = Context::new();

            context.insert("service_lower", &service_lower);
            let result = tera
                .render("service.tpl", &context)
                .expect("gen mod file false");

            mod_file
                .write_all(format!("pub mod {};\n", service_lower).as_bytes())
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

        let d = data.clone();
        let join_handle: tokio::task::JoinHandle<String> =
            tokio::spawn(
                async move { gen_api(&data.url, &data.service_name, &data.func_name).await },
            );

        match join_handle.await {
            Ok(res) => {
                created_service
                    .get_mut(&d.service_name)
                    .unwrap()
                    .write_all(format!("pub mod {};\n", d.func_name).as_bytes())
                    .expect("write mod error");

                let p = service_dir.join(format!("{}.rs", d.func_name));
                std::fs::write(p, res).expect("write file error");
            }
            Err(e) => {
                println!("gen api error: {:?}", e);
                return;
            }
        }
    }
}

async fn gen_api(url: &str, service_name: &str, func_name: &str) -> String {
    println!("url == {}", url);
    let url = url::Url::parse(url).expect("parse url failed, please check url");

    let api = reqwest::Url::parse_with_params(
        "https://open.feishu.cn/document_portal/v1/document/get_detail",
        [("fullPath", url.path().trim_start_matches("/document"))],
    )
    .expect("parse api url error");
    println!("api == {}", api);
    let data: model::Resp = reqwest::get(api.clone())
        .await
        .expect("get api data failed")
        .json()
        .await
        .expect("get api data text failed");

    let service = service_name;
    let service_lower = service.to_lowercase();
    let func_name = heck::AsSnakeCase(func_name).to_string();
    let upper_camel_func = heck::AsUpperCamelCase(&func_name).to_string();
    let req_type = upper_camel_func.clone() + "Req";
    let resp_type = upper_camel_func.clone() + "Resp";

    let mut context = Context::new();
    context.insert("service", &service);
    context.insert("service_lower", &service_lower);
    context.insert("func_name", &func_name);
    context.insert("UpperCamelFunc", &upper_camel_func);
    context.insert("req_type", &req_type);
    context.insert("resp_type", &resp_type);
    context.insert("doc_path", &url.path());
    context.insert("api_url", &api.to_string());
    context.insert("resp_empty_struct", "");

    let update_time = chrono::DateTime::from_timestamp_millis(data.data.update_time)
        .expect("parse update time failed")
        .to_rfc3339_opts(chrono::SecondsFormat::Secs, false);
    context.insert("update_time", &update_time);
    context.insert("fullPath", &data.data.full_path);

    parse_content(data.data.content, &mut context);

    let tera = Tera::new("./**.tpl").expect("load dir error");
    let result = tera.render("api.tpl", &context).expect("gen api false");

    RustFmt::default()
        .format_str(result.clone())
        .unwrap_or_else(|e| panic!("result = {}\nerror: {}", result, e))
}

lazy_static::lazy_static! {
    static ref TIPS_SPLIT: Regex = Regex::new(r"\{尝试一下\}\(.*\)\n").unwrap();
    static ref RE: Regex = Regex::new(r"\n#").unwrap();
    static ref TAG: Regex = Regex::new(r"#+\s*(.*?)\n").unwrap();
    static ref HTML: Regex = Regex::new(r":::html\n([\s\S]*?)\n:::").unwrap();
    static ref GET_PATH: Regex = Regex::new(r"https://open.feishu.cn(.*)").unwrap();
    static ref PATH_VAR: Regex = Regex::new(r":(\w+)|\{(\w+)\}").unwrap();
    static ref ARRAY_TYPE: Regex = Regex::new(r"array<(\w+)>").unwrap();
    static ref OLD_MARKDOWN_TABLE: Regex = Regex::new(r"\|(.*)\|").unwrap();
    static ref JSON_DATA: Regex = Regex::new(r"```json([\s\S]*?)```").unwrap();
}

fn parse_content(content: String, context: &mut Context) {
    let content = content.replace("/ssl:ttdoc/", "https://open.feishu.cn/document/");
    let mut iter = RE.split(&content);

    let tips = iter.next().expect("expect tips part");
    parse_tips(tips, context);

    context.insert("req_fields_path", "");
    context.insert("req_fields_query", "");
    context.insert("req_fields_body", "");
    context.insert("req_fields_body_sub_type", "");
    context.insert("resp_fields", "");
    context.insert("resp_fields_sub_type", "");
    context.insert("req_test_data", "\"{}\"");
    context.insert("resp_test_data", "\"{}\"");

    for part in iter {
        let part_string = part.to_string() + "\n";
        let tag = TAG
            .captures(&part_string)
            .expect("get tag error")
            .get(1)
            .unwrap()
            .as_str();
        // println!("tag: {}", tag);
        match tag {
            "请求" => parse_request_path_and_method(part, context),
            "请求头" => parse_request_header(part, context),
            "路径参数" => parse_request_path(part, context),
            "查询参数" => parse_request_query(part, context),
            "请求体" => parse_request_body(part, context),
            "响应体" => parse_response(part, context),
            "请求体示例" => parse_req_test_data(part, context),
            "响应体示例" => parse_resp_test_data(part, context),
            _ => continue,
        }
    }
}

fn parse_tips(tips: &str, context: &mut Context) {
    let mut tips = TIPS_SPLIT.split(tips);

    let description = ("#".to_string() + tips.next().unwrap())
        .split('\n')
        .map(|x| {
            if x.is_empty() {
                return x.to_string();
            }
            format!("/// {}", x).replace('\n', "\n/// ")
        })
        .filter(|x| !x.is_empty())
        .collect::<Vec<_>>()
        .join("\n///\n");
    context.insert("description", &description.trim());

    if let Some(tips) = tips.next() {
        let html = HTML.captures_iter(tips);
        let md_alert_selector = Selector::parse("md-alert").unwrap();
        let mut all_tips = vec![];
        for x in html {
            let html = x.get(1).unwrap().as_str();
            let document = Html::parse_document(html);
            let tips = document
                .select(&md_alert_selector)
                .map(|tip| tip.inner_html().trim().to_string())
                .filter(|x| !x.is_empty())
                .map(|tip| format!("/// {}", tip.replace('\n', "\n/// ")))
                .filter(|x| !x.is_empty())
                .collect::<Vec<_>>();
            all_tips.extend_from_slice(&tips);
        }
        let mut tips = all_tips.join("\n///\n");
        if tips.is_empty() {
            tips = "///\n".into();
        } else {
            tips = "///\n".to_string() + &tips + "\n///";
        }
        context.insert("tips", &tips.trim());
    } else {
        context.insert("tips", "");
    }
}

fn parse_request_path_and_method(part: &str, context: &mut Context) {
    let html = HTML.captures_iter(part).map(|x| x.get(1).unwrap().as_str());
    let md_tr_selector = Selector::parse("md-tr").unwrap();
    let md_th_selector = Selector::parse("md-th").unwrap();
    let md_td_selector = Selector::parse("md-td").unwrap();
    for h in html {
        let html = Html::parse_document(h);
        for tr in html.select(&md_tr_selector) {
            if let (Some(th), Some(td)) = (
                tr.select(&md_th_selector).next(),
                tr.select(&md_td_selector).next(),
            ) {
                match th.inner_html().as_str() {
                    "HTTP URL" => context.insert(
                        "path",
                        GET_PATH
                            .captures(td.inner_html().as_str())
                            .unwrap()
                            .get(1)
                            .unwrap()
                            .as_str(),
                    ),
                    "HTTP Method" => context.insert("method", td.inner_html().as_str()),
                    _ => continue,
                }
            }
        }
    }
}

fn parse_request_header(part: &str, context: &mut Context) {
    let html = HTML.captures_iter(part).map(|x| x.get(1).unwrap().as_str());
    let md_tag_selector = Selector::parse("md-tag").unwrap();
    let mut auth_fields = Vec::new();
    for h in html {
        let html = Html::parse_document(h);
        for tag in html.select(&md_tag_selector) {
            match tag.inner_html().as_str() {
                "tenant_access_token" => auth_fields.push("need_tenant_access_token: true,"),
                "app_access_token" => auth_fields.push("need_app_access_token: true,"),
                "user_access_token" => auth_fields.push("need_user_access_token: true,"),
                _ => panic!("unknown request header type: {}", tag.inner_html()),
            }
        }
    }
    context.insert("auth", &auth_fields.join("\n"));
}

fn parse_request_path(part: &str, context: &mut Context) {
    let html = HTML.captures_iter(part).map(|x| x.get(1).unwrap().as_str());

    let mut all_path_p = vec![];
    for h in html {
        let html = Html::parse_document(h);

        let path_params = parse_md_table(html.html().as_str());

        for p in path_params {
            let p = p.borrow();
            all_path_p.push(format!(
                "{}\n#[api(kind=\"path\",name=\"{}\")] pub {}: {}, ",
                p.desc,
                p.name,
                heck::AsSnakeCase(&p.name),
                p.type_name
            ));
        }
    }
    context.insert("req_fields_path", &all_path_p.join("\n"));
}

fn parse_request_query(part: &str, context: &mut Context) {
    let mut path_params = vec![];

    // 使用的是老的表格结构
    if HTML.captures_iter(part).count() == 0 {
        path_params = parse_old_table(part);
    } else {
        let html = HTML.captures_iter(part).map(|x| x.get(1).unwrap().as_str());

        for h in html {
            let html = Html::parse_document(h);
            path_params.extend(parse_md_table(html.html().as_str()));
        }
    }
    let mut all_query_p = vec![];
    for p in path_params {
        let p = p.borrow();
        let mut v_type = "var";
        let mut type_name = p.type_name.clone();
        if let Some(cap) = ARRAY_TYPE.captures(&type_name) {
            v_type = "list";
            type_name = format!("Vec<{}>", cap.get(1).unwrap().as_str());
        } else if p.desc.contains("用逗号分隔") {
            v_type = "list";
            type_name = "Vec<String>".to_string();
        }
        let mut option_tag = "false";
        if !p.is_needed.unwrap() {
            type_name = format!("Option<{}>", type_name);
            option_tag = "true";
        }

        all_query_p.push(format!(
            "{}\n#[api(kind=\"query\",name=\"{}\",v_type=\"{}\",option=\"{}\")] pub {}: {}, ",
            p.desc,
            p.name,
            v_type,
            option_tag,
            heck::AsSnakeCase(&p.name),
            type_name
        ));
    }
    context.insert("req_fields_query", &all_query_p.join("\n"));
}

fn parse_request_body(part: &str, context: &mut Context) {
    let html = HTML.captures_iter(part).map(|x| x.get(1).unwrap().as_str());
    let mut all_body_p = vec![];
    let mut body_params = vec![];
    for h in html {
        let html = Html::parse_document(h);
        body_params.extend(parse_md_table(html.html().as_str()));
    }
    let sub_types = parse_param_data_sub_type(&mut body_params, "Req");
    for p in &body_params {
        let p = p.borrow();
        let mut type_name = p.type_name.clone();
        if p.params.is_some() {
            type_name = heck::AsUpperCamelCase(type_name).to_string() + "Req";
        }
        all_body_p.push(format!(
            "{}\n#[api(kind=\"body\",name=\"{}\")] pub {}: {}, ",
            p.desc,
            p.name,
            heck::AsSnakeCase(&p.name),
            type_name
        ));
    }
    context.insert("req_fields_body", &all_body_p.join("\n"));
    context.insert("req_fields_body_sub_type", &sub_types.join("\n"));
}

fn parse_response(part: &str, context: &mut Context) {
    let html = HTML.captures_iter(part).map(|x| x.get(1).unwrap().as_str());
    let mut all_body_p = vec![];
    let mut body_params = vec![];
    for h in html {
        let html = Html::parse_document(h);
        body_params.extend(parse_md_table(html.html().as_str()));
    }
    let sub_types = parse_param_data_sub_type(&mut body_params, "Resp");
    for p in &body_params {
        let p = p.borrow();
        if ["code", "msg", "err"].contains(&p.name.as_str()) {
            continue;
        }
        let mut type_name = p.type_name.clone();
        if p.params.is_some() {
            type_name = heck::AsUpperCamelCase(type_name).to_string() + "Resp";
        }
        if ["\\-", "-", "\\\\-"].contains(&type_name.as_str()) {
            context.insert(
                "resp_empty_struct",
                "#[derive(Debug, Serialize, Deserialize, Clone, Default)]\n pub struct DataResp;",
            );
            type_name = "DataResp".to_string();
        }
        all_body_p.push(format!(
            "{}\n#[serde(rename=\"{}\")] pub {}: {}, ",
            p.desc,
            p.name,
            heck::AsSnakeCase(&p.name),
            type_name
        ));
    }
    context.insert("resp_fields", &all_body_p.join("\n"));
    context.insert("resp_fields_sub_type", &sub_types.join("\n"));
}

fn parse_req_test_data(part: &str, context: &mut Context) {
    if let Some(html) = HTML
        .captures_iter(part)
        .map(|x| x.get(1).unwrap().as_str())
        .next()
    {
        let md_json = Selector::parse("md-code-json").unwrap();
        let html = Html::parse_document(html);
        let json = html.select(&md_json).next().unwrap().inner_html();
        context.insert("req_test_data", &format!("r#\"{}\"#", json.as_str()));
        return;
    }

    if let Some(json) = JSON_DATA.captures(part).map(|x| x.get(1).unwrap()) {
        context.insert("req_test_data", &format!("r#\"{}\"#", json.as_str()));
    }
}

fn parse_resp_test_data(part: &str, context: &mut Context) {
    if let Some(html) = HTML
        .captures_iter(part)
        .map(|x| x.get(1).unwrap().as_str())
        .next()
    {
        let md_json = Selector::parse("md-code-json").unwrap();
        let html = Html::parse_document(html);
        let json = html.select(&md_json).next().unwrap().inner_html();
        context.insert("resp_test_data", &format!("r#\"{}\"#", json.as_str()));
        return;
    }

    if let Some(json) = JSON_DATA.captures(part).map(|x| x.get(1).unwrap()) {
        context.insert("resp_test_data", &format!("r#\"{}\"#", json.as_str()));
    }
}

#[derive(Debug, Clone)]
struct ParamData {
    name: String,
    type_name: String,
    is_needed: Option<bool>,
    desc: String,
    params: Option<Vec<Rc<RefCell<ParamData>>>>,
}

fn parse_param_data_sub_type(data: &mut Vec<Rc<RefCell<ParamData>>>, suffix: &str) -> Vec<String> {
    let template = r#"
        #[derive(Debug, Serialize, Deserialize, Clone, Default)]
        pub struct {{ subTypeUpperName }} {
            {{ fields }}
        }
    "#;
    let mut deq = VecDeque::new();
    for d in data {
        if d.borrow().params.is_some() {
            deq.push_back(d.clone());
        }
    }
    let mut res = vec![];
    while let Some(now) = deq.pop_front() {
        let mut context = Context::new();
        let now = now.borrow();

        let mut type_name = now.type_name.clone();
        let mut type_name1 = heck::AsUpperCamelCase(&type_name).to_string();
        if type_name.starts_with("Vec<") {
            type_name1 = type_name[4..type_name.len() - 1].to_owned();
        }
        type_name = type_name1 + suffix;
        context.insert("subTypeUpperName", &type_name);

        let mut fields = vec![];
        for p in now.params.as_ref().unwrap() {
            if ["\\-", "-", "\\\\-"].contains(&p.borrow().type_name.as_str()) {
                let name = p.borrow().name.clone();
                p.borrow_mut().type_name = heck::AsUpperCamelCase(name).to_string() + suffix;
            } else {
                let type_name = p.borrow().type_name.clone();
                if type_name.starts_with("Vec<") {
                    p.borrow_mut().type_name = "Vec<".to_string()
                        + &heck::AsUpperCamelCase(&type_name[4..type_name.len() - 1]).to_string()
                        + suffix
                        + ">";
                }
            }

            let mut type_name = p.borrow().type_name.clone();
            if p.borrow().params.is_some() {
                deq.push_back(p.clone());

                if type_name.starts_with("Vec<") {
                    type_name = "Vec<".to_string()
                        + &heck::AsUpperCamelCase(&type_name[4..type_name.len() - 1]).to_string()
                        + suffix
                        + ">";
                } else {
                    type_name = heck::AsUpperCamelCase(type_name).to_string() + suffix;
                }
            }
            fields.push(format!(
                "{}\n#[serde(rename=\"{}\")] pub {}: {},",
                p.borrow().desc,
                p.borrow().name,
                heck::AsSnakeCase(&p.borrow().name),
                type_name
            ));
        }
        context.insert("fields", &fields.join("\n"));
        res.push(Tera::one_off(template, &context, false).unwrap());
    }

    res
}

fn parse_md_table(html: &str) -> Vec<Rc<RefCell<ParamData>>> {
    let md_dt_table = Selector::parse("md-dt-table").unwrap();
    let md_table = Selector::parse("md-table").unwrap();

    let mut md_tr = Selector::parse("md-dt-tr").unwrap();
    let mut md_td = Selector::parse("md-dt-td").unwrap();
    let md_text = Selector::parse("md-text").unwrap();
    let mut md_tbody = Selector::parse("md-dt-tbody").unwrap();

    let mut is_old_table = false;

    let html = Html::parse_document(html);
    let mut res = vec![];

    let tables = if html.select(&md_dt_table).count() > 0 {
        html.select(&md_dt_table)
    } else {
        // 有些 api 的路径参数 table 是用 md-table 标签包裹的 case: https://open.feishu.cn/document/server-docs/docs/sheets-v3/protect-range/retrieve-protection-scopes
        is_old_table = true;
        md_tbody = Selector::parse("md-tbody").unwrap();
        md_td = Selector::parse("md-td").unwrap();
        md_tr = Selector::parse("md-tr").unwrap();
        html.select(&md_table)
    };

    let mut now_level = 0;
    let mut stack: Vec<Rc<RefCell<ParamData>>> = vec![];
    let mut pre: Option<Rc<RefCell<ParamData>>> = None;
    for table in tables {
        let body = table
            .select(&md_tbody)
            .next()
            .expect("table must have body");

        for tr in body.select(&md_tr) {
            let count = tr.select(&md_td).count();
            if count != 3 && count != 4 {
                panic!(
                    "table td count must have 3 or 4 td, tr: {}, count: {}",
                    tr.html(),
                    count
                );
            }
            let mut td = tr.select(&md_td);

            let name = if !is_old_table {
                td.next()
                    .unwrap()
                    .select(&md_text)
                    .next()
                    .unwrap()
                    .inner_html()
            } else {
                td.next().unwrap().inner_html()
            };

            let mut type_name = if !is_old_table {
                td.next()
                    .unwrap()
                    .select(&md_text)
                    .next()
                    .unwrap()
                    .inner_html()
            } else {
                td.next().unwrap().inner_html()
            }
            .replace("string", "String")
            .replace("int", "i64")
            .replace("boolean", "bool");
            if type_name.ends_with("\\[\\]") {
                type_name = format!("Vec<{}>", &type_name[..type_name.len() - 4]);
            }

            let is_needed = if count == 4 {
                if td.next().unwrap().inner_html().trim() == "是" {
                    Some(true)
                } else {
                    Some(false)
                }
            } else {
                None
            };

            let desc = td.next().unwrap();
            let mut temp = "".to_string();
            for child in desc.descendants() {
                if let Some(text) = child.value().as_text() {
                    temp.push_str(&(text.trim().to_string() + "\n"));
                } else if let Some(element) = child.value().as_element() {
                    match element.name() {
                        "md-enum-item" => {
                            let key = element.attr("key").unwrap_or("");
                            if !key.is_empty() {
                                temp.push_str(&format!("`{}`: ", key));
                            }
                        }
                        "md-perm" => {
                            let support_app_types = element.attr("support_app_types").expect(
                                "md-perm must have support_app_types
                                ",
                            );
                            let support_app_types = match support_app_types {
                                "custom" => "仅自建应用",
                                _ => panic!("unknown support_app_types: {}", support_app_types),
                            };
                            temp.push_str(&format!("(**{}**) ", support_app_types));
                        }
                        _ => {
                            continue;
                        }
                    }
                } else {
                    panic!("unknown child type: {:?}", child.value());
                }
            }
            let desc = temp
                .split('\n')
                .filter(|x| !x.is_empty())
                .map(|x| format!("/// {}", x))
                .collect::<Vec<_>>()
                .join("\n///\n");

            let param_data = ParamData {
                name,
                type_name,
                is_needed,
                desc,
                params: None,
            };

            let param_data = Rc::new(RefCell::new(param_data));

            if let Some(level) = tr.attr("level") {
                let level = level.parse::<usize>().expect("parse level error");
                if level > now_level {
                    now_level = level;

                    pre.clone().unwrap().borrow_mut().params = Some(vec![param_data.clone()]);

                    stack.push(pre.unwrap());
                } else if level < now_level {
                    while level < now_level {
                        stack.pop().expect("must have father");
                        now_level -= 1;
                    }
                    if stack.is_empty() {
                        res.push(param_data.clone());
                    } else {
                        stack
                            .last()
                            .unwrap()
                            .borrow_mut()
                            .params
                            .as_mut()
                            .unwrap()
                            .push(param_data.clone());
                    }
                } else if let Some(father) = stack.last() {
                    let mut father = father.borrow_mut();

                    if father.params.is_none() {
                        father.params = Some(vec![]);
                    }

                    father.params.as_mut().unwrap().push(param_data.clone());
                } else {
                    res.push(param_data.clone());
                }
                pre = Some(param_data.clone());
            } else {
                res.push(param_data.clone());
            }
        }
    }

    res
}

// 旧版 API 用的是 markdown 列表展示的 case: https://open.feishu.cn/document/server-docs/docs/sheets-v3/conditionformat/condition-format-get
fn parse_old_table(table: &str) -> Vec<Rc<RefCell<ParamData>>> {
    let table = table.split('\n').skip(3);
    let mut res = vec![];
    for line in table {
        let mut cols = OLD_MARKDOWN_TABLE
            .captures(line)
            .expect("parse table col error")
            .get(1)
            .unwrap()
            .as_str()
            .split('|')
            .map(|x| x.trim());
        let count = cols.clone().count();
        if count != 3 && count != 4 {
            panic!("table col must have 3 or 4");
        }
        let name = cols.next().unwrap().to_string();
        let type_name = cols
            .next()
            .unwrap()
            .to_string()
            .replace("string", "String")
            .replace("int", "i64");
        let is_needed = if count == 4 {
            if cols.next().unwrap() == "是" {
                Some(true)
            } else {
                Some(false)
            }
        } else {
            None
        };
        let desc = cols
            .next()
            .unwrap()
            .trim()
            .split('\n')
            .map(|x| format!("/// {}", x))
            .collect::<Vec<_>>()
            .join("\n");
        let param_data = ParamData {
            name,
            type_name,
            is_needed,
            desc,
            params: None,
        };
        res.push(param_data);
    }
    res.into_iter().map(|x| Rc::new(RefCell::new(x))).collect()
}
