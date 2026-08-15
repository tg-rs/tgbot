use std::{
    collections::HashMap,
    error::Error,
    fmt,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use mockito::{Matcher, Request, Server, ServerGuard};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use serde_json::Value;
use serde_json_assert::{CompareMode, Config as JsonConfig, assert_json_matches_no_panic as json_matches};
use tgbot::api::{Client, Method};

pub type SpecBasename = &'static str;
pub type CheckResult<R> = fn(R);

pub struct Cx {
    server: ServerGuard,
    client: Client,
}

impl Cx {
    pub async fn new() -> Self {
        let server = Server::new_async().await;
        let host = server.url();
        let client = Client::new("-token").unwrap().with_host(host);
        Self { server, client }
    }
}

impl Cx {
    pub async fn execute_batch<M, R, const N: usize>(&mut self, items: [(SpecBasename, M, CheckResult<R>); N])
    where
        M: Method<Response = R>,
        R: DeserializeOwned + Send + Serialize + 'static,
    {
        for (spec, method, check_result) in items {
            self.execute(spec, method, check_result).await
        }
    }

    pub async fn execute<M, R>(&mut self, spec_basename: SpecBasename, method: M, check_result: CheckResult<R>)
    where
        M: Method<Response = R>,
        R: DeserializeOwned + Send + Serialize + 'static,
    {
        let path = Path::new(file!())
            .parent()
            .and_then(|x| x.parent())
            .ok_or_else(|| CxError::build_spec_path(spec_basename))
            .unwrap()
            .join("data")
            .join(format!("{spec_basename}.json"));
        if path.exists() {
            self.execute_match(path, method, check_result).await;
        } else {
            self.execute_gen(path, method).await;
        }
    }

    async fn execute_match<M, R>(&mut self, path: PathBuf, method: M, check_result: CheckResult<R>)
    where
        M: Method<Response = R>,
        R: DeserializeOwned + Send + Serialize + 'static,
    {
        let spec = Spec::new(path.clone());
        let url_path = &format!("/bot-token/{}", spec.data.url_path);
        let request_matcher = RequestMatcher::from(&spec);
        let mock = self
            .server
            .mock(spec.data.http_method.as_str(), url_path.as_str())
            .match_request(move |request| request_matcher.match_request(request))
            .with_body(
                serde_json::to_string(&spec.data.rep_body)
                    .map_err(|err| CxError::serialize_response_body(path.as_ref(), err))
                    .unwrap(),
            )
            .create();
        let raw_result = self
            .client
            .execute(method)
            .await
            .map_err(|err| CxError::new(format!("{}", path.display()), err))
            .unwrap();
        spec.match_result(&raw_result);
        mock.remove();
        check_result(raw_result);
    }

    async fn execute_gen<M, R>(&mut self, path: PathBuf, method: M)
    where
        M: Method<Response = R>,
        R: DeserializeOwned + Send + Serialize + 'static,
    {
        let result = serde_json::to_string(&serde_json::json!({"ok": false}))
            .map_err(|err| CxError::serialize_response_body(path.as_ref(), err))
            .unwrap();
        let path_get = path.clone();
        let mock_get = self
            .server
            .mock("GET", Matcher::Any)
            .match_request(move |request| generate_request(path_get.as_ref(), request))
            .with_body(result.clone())
            .create();
        let path_post = path.clone();
        let mock_post = self
            .server
            .mock("POST", Matcher::Any)
            .match_request(move |request| generate_request(path_post.as_ref(), request))
            .with_body(result.clone())
            .create();

        let Err(_err) = self.client.execute(method).await else {
            panic!("Expects an error")
        };

        mock_get.remove();
        mock_post.remove();
    }
}

#[derive(Debug)]
struct Spec {
    source: PathBuf,
    data: SpecData,
    json_config: JsonConfig,
}

impl Spec {
    fn new(path: impl AsRef<Path>) -> Self {
        let path = path.as_ref();
        let mut file = File::open(path)
            .map_err(|err| CxError::open_spec_file(path, err))
            .unwrap();
        let mut buf = String::new();
        file.read_to_string(&mut buf)
            .map_err(|err| CxError::read_spec_file(path, err))
            .unwrap();

        let data = serde_json::from_str(&buf)
            .map_err(|err| CxError::parse_spec_file(path, err))
            .unwrap();

        let json_config = JsonConfig::new(CompareMode::Strict);

        Self {
            source: path.to_path_buf(),
            data,
            json_config,
        }
    }

    fn match_result(&self, raw_result: impl Serialize) {
        let result = serde_json::to_value(&raw_result)
            .map_err(|err| CxError::serialize_execute_result(self.source.as_ref(), err))
            .unwrap();
        json_matches(&self.data.rep_body["result"], &result, &self.json_config)
            .map_err(|err| CxError::unexpected_execute_result(self.source.as_ref(), err))
            .unwrap();
    }
}

type Multipart = HashMap<String, Value>;

#[derive(Debug, Deserialize, Serialize)]
struct SpecData {
    http_method: String,
    url_path: String,
    req_body: Option<SpecDataReqBody>,
    rep_body: Value,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
enum SpecDataReqBody {
    Json(Value),
    Multipart(Multipart),
}

enum RequestBody {
    Empty,
    Json(Value, JsonConfig),
    Multipart(Multipart, JsonConfig),
}

struct RequestMatcher {
    source: PathBuf,
    expected_body: RequestBody,
}

impl From<&Spec> for RequestMatcher {
    fn from(value: &Spec) -> Self {
        let config = value.json_config.clone();
        let expected_body = match &value.data.req_body {
            Some(SpecDataReqBody::Json(req_body)) => RequestBody::Json(req_body.clone(), config),
            Some(SpecDataReqBody::Multipart(multipart)) => RequestBody::Multipart(multipart.clone(), config),
            None => RequestBody::Empty,
        };
        Self {
            source: value.source.clone(),
            expected_body,
        }
    }
}

impl RequestMatcher {
    fn match_request(&self, request: &Request) -> bool {
        let body = request
            .body()
            .map_err(|err| CxError::read_request_body(self.source.as_ref(), err))
            .unwrap();
        match &self.expected_body {
            RequestBody::Empty => {
                if body.is_empty() {
                    true
                } else {
                    eprintln!("Expects empty body, got: {:?}", body);
                    false
                }
            }
            RequestBody::Json(expected, config) => {
                let actual: Value = serde_json::from_slice(body)
                    .map_err(|err| CxError::deserialize_request_body(self.source.as_ref(), err))
                    .unwrap();

                match json_matches(&expected, &actual, config) {
                    Ok(()) => true,
                    Err(err) => {
                        eprintln!("{err}");
                        false
                    }
                }
            }
            RequestBody::Multipart(expected, config) => {
                let mut actual = parse_multipart(body.clone());
                let mut errors = Vec::new();
                for (k, a) in expected {
                    match actual.remove(k) {
                        Some(b) => {
                            if let Err(err) = json_matches(&a, &b, config) {
                                errors.push(format!("{k}:\n{err}"));
                            }
                        }
                        None => {
                            errors.push(format!("{k} is not found"));
                        }
                    }
                }
                for (k, v) in actual {
                    errors.push(format!("Unmatched: {k}={v}"));
                }
                if errors.is_empty() {
                    true
                } else {
                    let errors = errors.join("\n");
                    eprintln!("{errors}");
                    false
                }
            }
        }
    }
}

fn generate_request(path: &Path, request: &Request) -> bool {
    let method = request.path().split("/").last().map(String::from).unwrap();
    let data = match request.method() {
        "GET" => {
            serde_json::json!({
                "http_method": "GET",
                "url_path": method,
                "rep_body": {
                    "ok": true,
                    "result": {}
                }
            })
        }
        "POST" => {
            let body = request.body().unwrap();
            let mut data = serde_json::json!({
                "http_method": "POST",
                "url_path": method,
                "req_body": {},
                "rep_body": {
                    "ok": true,
                    "result": {}
                }
            });
            let content_type = request.header("content-type");
            if content_type[0].to_str().unwrap().starts_with("application/json") {
                data["req_body"]["json"] = serde_json::from_slice(body).unwrap();
            } else {
                let mut multipart = serde_json::json!({});
                for (k, v) in parse_multipart(body.clone()) {
                    multipart[k] = v;
                }
                data["req_body"]["multipart"] = multipart;
            }
            data
        }
        x => {
            panic!("Got an unexpected {x} request");
        }
    };
    let mut file = File::options()
        .create(true)
        .truncate(true)
        .write(true)
        .open(path)
        .unwrap();
    let buf = serde_json::to_vec_pretty(&data).unwrap();
    file.write_all(&buf).unwrap();
    false
}

#[derive(Debug, PartialEq)]
struct MultipartBoundary(String);

impl MultipartBoundary {
    fn new(value: &str) -> Option<Self> {
        if value.starts_with("--") {
            Some(Self(String::from(if value.ends_with("--") {
                value.strip_suffix("--").unwrap()
            } else {
                value
            })))
        } else {
            None
        }
    }
}

enum MultipartState {
    Boundary,
    Disposition(MultipartBoundary),
    Name(String, MultipartBoundary),
    Value(String, MultipartBoundary, String),
}

fn parse_multipart(data: Vec<u8>) -> Multipart {
    let data = String::from_utf8(data).unwrap();
    let mut result = HashMap::new();
    let mut state = MultipartState::Boundary;
    for line in data.lines() {
        state = match state {
            MultipartState::Boundary => MultipartState::Disposition(MultipartBoundary::new(line).unwrap()),
            MultipartState::Disposition(boundary) => {
                let name = parse_multipart_field_name(line);
                MultipartState::Name(name, boundary)
            }
            MultipartState::Name(name, boundary) => MultipartState::Value(name, boundary, String::new()),
            MultipartState::Value(name, boundary, value) => {
                if let Some(boundary_end) = MultipartBoundary::new(line) {
                    assert_eq!(boundary_end, boundary);
                    let value = parse_multipart_field_value(value);
                    result.insert(name, value);
                    MultipartState::Disposition(boundary)
                } else {
                    let value = value + line;
                    MultipartState::Value(name, boundary, value)
                }
            }
        }
    }
    result
}

fn parse_multipart_field_name(data: &str) -> String {
    let mut parts = data.split("; ");
    let disposition = parts.next().unwrap();
    assert_eq!(disposition, "Content-Disposition: form-data");
    let name = parts.next().unwrap();
    let mut name = name.split("=");
    assert_eq!(name.next().unwrap(), "name");
    let name = name.next().unwrap();
    assert!(name.starts_with("\""));
    assert!(name.ends_with("\""));
    String::from(name.strip_prefix("\"").unwrap().strip_suffix("\"").unwrap())
}

fn parse_multipart_field_value(data: String) -> Value {
    if data == "true" {
        Value::Bool(true)
    } else if data == "false" {
        Value::Bool(false)
    } else if data == "null" {
        Value::Null
    } else {
        let mut chars = data.chars();
        let first = chars.next().unwrap();
        let mut is_json = first == '[';
        is_json = is_json || first == '{';
        is_json = is_json || first == '"';
        is_json = is_json || (first.is_ascii_digit() && chars.all(|x| x.is_ascii_digit()));
        let data = if is_json { data } else { format!(r#""{data}""#) };
        serde_json::from_str(&data)
            .map_err(|err| format!("{err}: {data}"))
            .unwrap()
    }
}

#[derive(Debug)]
struct CxError {
    message: String,
}

impl CxError {
    fn new(prefix: impl fmt::Display, err: impl fmt::Display) -> Self {
        Self {
            message: format!("{prefix}: {err}"),
        }
    }

    fn build_spec_path(spec: SpecBasename) -> Self {
        Self::new(format!("Can not build spec path ({spec})"), "parent is not found")
    }

    fn deserialize_request_body(spec: &Path, err: impl fmt::Display) -> Self {
        Self::new(format!("Can not deserialize request body ({})", spec.display()), err)
    }

    fn open_spec_file(spec: &Path, err: impl fmt::Display) -> Self {
        Self::new(format!("Can not open spec file ({})", spec.display()), err)
    }

    fn parse_spec_file(spec: &Path, err: impl fmt::Display) -> Self {
        Self::new(format!("Can not parse spec file ({})", spec.display()), err)
    }

    fn read_request_body(spec: &Path, err: impl fmt::Display) -> Self {
        Self::new(format!("Can not read request body ({})", spec.display()), err)
    }

    fn read_spec_file(spec: &Path, err: impl fmt::Display) -> Self {
        Self::new(format!("Can not read spec file ({})", spec.display()), err)
    }

    fn serialize_execute_result(spec: &Path, err: impl fmt::Display) -> Self {
        Self::new(format!("Can not serialize execute result ({})", spec.display()), err)
    }

    fn serialize_response_body(spec: &Path, err: impl fmt::Display) -> Self {
        Self::new(format!("Can not serialize response body ({})", spec.display()), err)
    }

    fn unexpected_execute_result(spec: &Path, data: String) -> Self {
        Self {
            message: format!("Got an unexpected execute result ({}):\n{data}", spec.display()),
        }
    }
}

impl fmt::Display for CxError {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        write!(out, "{}", self.message)
    }
}

impl Error for CxError {}
