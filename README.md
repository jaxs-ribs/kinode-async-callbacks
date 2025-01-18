# Async Requests with Callbacks

It's now possible to send async requests with a callback handler! This is extremely rudimentary, but we can now write macros to make this much more ergonomic!

```rust
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
)?;
```

The goal is to later be able to do something in the style of:

```js
from([1]) 
  .pipe(
    mergeMap(initial => makeRequest('http://api1.com', initial)),
    mergeMap(response1 => makeRequest('http://api2.com', response1.data)),
    mergeMap(response2 => makeRequest('http://api3.com', response2.data)),
    mergeMap(response3 => makeRequest('http://api4.com', response3.data)),
    mergeMap(response4 => makeRequest('http://api5.com', response4.data))
  )
  .subscribe({
    next: finalResponse => console.log('Final response:', finalResponse.data),
    error: error => console.error('Error in request chain:', error),
    complete: () => console.log('Request chain completed')
  });
```
