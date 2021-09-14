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

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn handler_handles() {
        let event = json!({});

        assert_eq!(
            handler(event.clone(), Context::default())
                .await
                .unwrap(),
            json!({"body": "Boop!"})
        )
    }
}
