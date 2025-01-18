use serde::{Deserialize, Serialize};
use process_macros::SerdeJsonInto;


#[derive(Debug, Serialize, Deserialize, SerdeJsonInto, Clone)]
pub enum AsyncRequest {
    StepA(String),
    StepB(String),
    StepC(String),
}

#[derive(Debug, Serialize, Deserialize, SerdeJsonInto, Clone)]
pub enum AsyncResponse {
    StepA(String),
    StepB(String),
    StepC(String),
}

