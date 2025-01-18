use crate::*;

#[allow(unused)]
pub fn send_async_request<R, F>(
    state: &mut State,
    destination: &Address,
    body: R,
    callback: F,
) -> anyhow::Result<()>
where
    R: serde::Serialize + Into<Vec<u8>>,
    F: FnOnce(&[u8], &mut State) -> anyhow::Result<()> + Send + 'static,
{
    let correlation_id = generate_job_handle();

    state
        .pending_callbacks
        .insert(correlation_id.clone(), Box::new(callback));

    Request::to(destination)
        .context(correlation_id.as_bytes())
        .body(body)
        .expects_response(10)
        .send()?;

    Ok(())
}

pub fn generate_job_handle() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[macro_export]
macro_rules! send_async {
    (
        $state:expr,       // The mutable State
        $destination:expr, // The Address to send to
        $body:expr,        // The body (request) being sent
        ( $resp:ident, $st:ident ) $callback_block:block
    ) => {{
        // Step 1: Generate correlation id
        let correlation_id = $crate::generate_job_handle();

        // Step 2: Insert callback in the pending_callbacks map
        $state.pending_callbacks.insert(
            correlation_id.clone(),
            Box::new(
                move |$resp: &[u8], $st: &mut $crate::State| -> anyhow::Result<()> {
                    // The user’s callback code goes here
                    $callback_block
                },
            ),
        );

        // Step 3: Send the message, associating correlation_id as context
        $crate::Request::to($destination)
            .context(correlation_id.as_bytes())
            .body($body)
            .expects_response(10) // if you want a response timeout
            .send()?;

        // Macro itself returns anyhow::Result<()>
        Ok(())
    }};
}

pub fn handle_step_a(response_body: &[u8], state: &mut State) -> anyhow::Result<()> {
    kiprintln!(
        "Got StepA response: {:?}",
        String::from_utf8_lossy(response_body)
    );
    state.my_lego_stack.push("Got StepA result!".into());

    let address: Address = ("our", "async-receiver", "async-callbacks", "template.os").into();
    send_async!(
        state,
        &address,
        AsyncRequest::StepB("French Fries".to_string()),
        (response_body, st) { handle_step_b(response_body, st) }
    )
}

pub fn handle_step_b(response_body: &[u8], state: &mut State) -> anyhow::Result<()> {
    kiprintln!(
        "Got StepB response: {:?}",
        String::from_utf8_lossy(response_body)
    );
    state.my_lego_stack.push("Got StepB result!".into());

    let address: Address = ("our", "async-receiver", "async-callbacks", "template.os").into();
    send_async!(
        state,
        &address,
        AsyncRequest::StepC("Gravy".to_string()),
        (response_body, st) { handle_step_c(response_body, st) }
    )
}

pub fn handle_step_c(response_body: &[u8], state: &mut State) -> anyhow::Result<()> {
    kiprintln!(
        "Got StepC response: {:?}",
        String::from_utf8_lossy(response_body)
    );
    state.my_lego_stack.push("Got StepC result!".into());
    Ok(())
}
