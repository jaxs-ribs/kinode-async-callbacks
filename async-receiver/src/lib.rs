use kinode_process_lib::kiprintln;
use kinode_process_lib::logging::{error, info, init_logging, Level};
use kinode_process_lib::{await_message, call_init, Address, Message, Response};
use shared::AsyncRequest;
use shared::AsyncResponse;

wit_bindgen::generate!({
    path: "target/wit",
    world: "async-callbacks-template-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

fn handle_message(_our: &Address, message: &Message) -> anyhow::Result<()> {
    match message {
        Message::Request { body, .. } => {
            let async_request: AsyncRequest = serde_json::from_slice(body)?;
            std::thread::sleep(std::time::Duration::from_secs(3));
            let response_body = match async_request {
                AsyncRequest::StepA(_) => {
                    AsyncResponse::StepA("Hello from the other side A".to_string())
                }
                AsyncRequest::StepB(_) => {
                    AsyncResponse::StepB("Hello from the other side B".to_string())
                }
                AsyncRequest::StepC(_) => {
                    AsyncResponse::StepC("Hello from the other side C".to_string())
                }
            };

            Response::new()
                .body(response_body)
                .send()?;
            Ok(())
        }
        Message::Response { .. } => {
            kiprintln!("Got a response");
            Ok(())
        }
    }
}

call_init!(init);
fn init(our: Address) {
    init_logging(&our, Level::DEBUG, Level::INFO, None, None).unwrap();
    info!("begin");

    loop {
        match await_message() {
            Err(send_error) => error!("got SendError: {send_error}"),
            Ok(ref message) => match handle_message(&our, message) {
                Ok(_) => {}
                Err(e) => error!("got error while handling message: {e:?}"),
            },
        }
    }
}
