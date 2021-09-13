use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{json, Value};

#[tokio::main]
async fn main() -> Result<(), Error> {
    println!("cold start");
    let processor = handler_fn(handler);
    lambda_runtime::run(processor).await?;
    Ok(())
}

async fn handler(
    _: Value,
    _: Context,
) -> Result<Value, Error> {
    println!("handler");
    Ok(json!({"body": "Boop!"}))
}
