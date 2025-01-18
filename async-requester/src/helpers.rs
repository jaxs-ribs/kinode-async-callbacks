use crate::*;


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