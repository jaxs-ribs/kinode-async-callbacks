use std::collections::HashMap;

use kinode_process_lib::await_message;
use kinode_process_lib::call_init;
use kinode_process_lib::kiprintln;
use kinode_process_lib::logging::error;
use kinode_process_lib::logging::init_logging;
use kinode_process_lib::logging::Level;
use kinode_process_lib::Address;
use kinode_process_lib::Message;
use kinode_process_lib::Request;
use serde::Deserialize;
use serde::Serialize;
use shared::AsyncRequest;

wit_bindgen::generate!({
    path: "target/wit",
    world: "async-callbacks-template-dot-os-v0",
    generate_unused_types: true,
    additional_derives: [serde::Deserialize, serde::Serialize, process_macros::SerdeJsonInto],
});

mod helpers;
mod structs;

use helpers::*;
use structs::*;

type Callback = Box<dyn FnOnce(&[u8], &mut State) -> anyhow::Result<()> + Send + 'static>;

fn handle_message(_our: &Address, message: &Message, state: &mut State) -> anyhow::Result<()> {
    match message {
        Message::Request { source, .. } => handle_request(state, source),
        Message::Response {
            context,
            body,
            source,
            ..
        } => handle_response(body, source, state, context),
    }
}

fn handle_request(state: &mut State, _source: &Address) -> anyhow::Result<()> {
    kiprintln!("Got a request to trigger an async operation");
    let address: Address = ("our", "async-receiver", "async-callbacks", "template.os").into();

    send_async!(
        state,
        &address,
        AsyncRequest::StepA("Mashed Potatoes".to_string()),
        (response_body, st) {
            kiprintln!("Got a response: {:?}",
                String::from_utf8_lossy(response_body)
            );
            st.my_lego_stack.push("Got StepA result!".into());
            Ok(())
        }
    )
}

fn handle_response(
    body: &[u8],
    _source: &Address,
    state: &mut State,
    context: &Option<Vec<u8>>,
) -> anyhow::Result<()> {
    if let Some(context_id) = context {
        let correlation_id = String::from_utf8_lossy(context_id); // or parse as needed
        if let Some(callback) = state.pending_callbacks.remove(&correlation_id.to_string()) {
            callback(body, state)?;
        } else {
            kiprintln!("Got a response, but no matching callback found");
        }
    } else {
        // This is truly an unexpected response with no context
        kiprintln!(
            "Received unexpected message (no context): {:?}",
            String::from_utf8_lossy(body)
        );
    }
    Ok(())
}

call_init!(init);
fn init(our: Address) {
    init_logging(&our, Level::DEBUG, Level::INFO, None, None).unwrap();
    kiprintln!("begin");

    let mut state = State::default();

    loop {
        match await_message() {
            Err(send_error) => error!("got SendError: {send_error}"),
            Ok(ref message) => match handle_message(&our, message, &mut state) {
                Ok(_) => {}
                Err(e) => error!("got error while handling message: {e:?}"),
            },
        }
    }
}

// m our@async-requester:async-callbacks:template.os '{}'
