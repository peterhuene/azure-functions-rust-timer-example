use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::env;
use std::net::Ipv4Addr;
use warp::{Filter, Rejection, Reply};

/// Request data about how the function was triggered
#[derive(Debug, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct InvokeRequest {
    data: HashMap<String, Value>,
    metadata: HashMap<String, Value>,
}

/// Response data for the timer function
#[derive(Debug, Serialize)]
#[serde(rename_all = "PascalCase")]
struct InvokeResponse {
    #[serde(skip_serializing_if = "HashMap::is_empty")]
    outputs: HashMap<String, Value>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    logs: Vec<String>,
    return_value: Value,
}

/// Implements the timer function
async fn timer_func(req: InvokeRequest) -> Result<impl Reply, Rejection> {
    let response = InvokeResponse {
        // This dummy output value supresses an extraneous warning from the Azure Functions Host
        // See: https://github.com/Azure/azure-functions-host/issues/6717
        outputs: [("output".to_string(), Value::Null)].into(),
        logs: vec![format!("Invoke request data: {:#?}", req)],
        return_value: "This value returned from a timer function!".into(),
    };

    Ok(warp::reply::json(&response))
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let route = warp::post()
        .and(warp::path("TimerExample"))
        .and(warp::body::json())
        .and_then(timer_func)
        .with(warp::log("handler"));

    let port_key = "FUNCTIONS_CUSTOMHANDLER_PORT";
    let port: u16 = match env::var(port_key) {
        Ok(val) => val.parse().expect("Custom Handler port is not a number!"),
        Err(_) => 3000,
    };

    log::info!("starting handler on port {}...", port);

    warp::serve(route).run((Ipv4Addr::UNSPECIFIED, port)).await
}
