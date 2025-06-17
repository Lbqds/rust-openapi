use openapi::models::{BlockAndEvents, ContractEvent, TransactionTemplate};
use serde::{Deserialize, Serialize};
use serde_json::{Result, json};

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcRequest<T> {
    pub jsonrpc: String,
    pub method: String,
    pub params: T,
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum JsonRpcResponse {
    Success(ResponseSuccess),
    Failure(ResponseFailure),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseSuccess {
    pub jsonrpc: String,
    pub result: serde_json::Value,
    pub id: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ResponseFailure {
    pub jsonrpc: String,
    pub error: Error,
    pub id: Option<i64>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Error {
    pub code: i32,
    pub message: String,
    pub data: Option<String>,
}

pub enum SubscribeParams {
    Simple(SimpleEventType),
    Event(EventSubscribeParams),
}

#[derive(PartialEq, Debug)]
pub enum SimpleEventType {
    Block,
    Tx,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventSubscribeParams {
    pub addresses: Vec<String>,
    #[serde(rename = "eventIndex", skip_serializing_if = "Option::is_none")]
    pub event_index: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BlockNotification {
    pub subscription: String,
    pub result: BlockAndEvents,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TxNotification {
    pub subscription: String,
    pub result: TransactionTemplate,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EventNotification {
    pub subscription: String,
    pub result: ContractEvent,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub removed: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Notification {
    Block(BlockNotification),
    Tx(TxNotification),
    Event(EventNotification),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct JsonRpcNotification {
    pub method: String,
    pub params: Notification,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum WsResponse {
    Notification(JsonRpcNotification),
    JsonRpcResponse(JsonRpcResponse),
}

pub fn build_subscribe_request(params: SubscribeParams, id: i64) -> Result<String> {
    let params = match params {
        SubscribeParams::Simple(SimpleEventType::Block) => vec![json!("block")],
        SubscribeParams::Simple(SimpleEventType::Tx) => vec![json!("tx")],
        SubscribeParams::Event(params) => {
            vec![json!("contract"), serde_json::to_value(params).unwrap()]
        }
    };
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        method: "subscribe".to_string(),
        params,
        id,
    };
    serde_json::to_string(&request)
}
