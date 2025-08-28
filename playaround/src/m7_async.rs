#[cfg(test)]
mod tests {

    use std::io::{Error, ErrorKind};

    async fn my_async_call(url: &str) -> Result<serde_json::Value, reqwest::Error> {
        let response: serde_json::Value =
            reqwest::get(url).await?.json::<serde_json::Value>().await?;
        Ok(response)
    }

    async fn my_async_call_error_handling(url: &str) -> Result<serde_json::Value, std::io::Error> {
        let response = reqwest::get(url)
            .await
            .map_err(|_| Error::new(ErrorKind::Other, "Could not retrieve data"))?;

        let json_resp = response
            .json::<serde_json::Value>()
            .await
            .map_err(|_| Error::new(ErrorKind::Other, "Invalid JSON data"))?;

        Ok(json_resp)
    }

    #[tokio::test]
    async fn test_async_fn() {
        let api_url = "https://pokeapi.co/api/v2/pokemon/ditto";
        let result: Result<serde_json::Value, reqwest::Error> = my_async_call(api_url).await;
        match result {
            Ok(data) => {
                assert!(data.get("name").is_some());
                assert_eq!(data["name"], "ditto");
                dbg!(data);
            }
            Err(_) => panic!("API call failed"),
        }
    }

    #[tokio::test]
    async fn test_async_fn_error_handling() {
        let api_url = "https://pokeapi.co/api/v2/pokemon/invalid";
        let result: Result<serde_json::Value, std::io::Error> =
            my_async_call_error_handling(api_url).await;
        match result {
            Ok(data) => {
                assert!(data.get("name").is_some());
                assert_eq!(data["name"], "ditto");
                dbg!(data);
            }
            Err(_) => panic!("API call failed"),
        }
    }
}
