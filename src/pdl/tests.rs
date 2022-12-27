#[cfg(test)]
mod tests {
    #[tokio::main]
    async fn test_download() -> Result<(), Box<dyn std::error::Error>> {
        let url = "https://official-package.wpscdn.cn/wps/download/WPS_Installer.exe";
        let cli = reqwest::Client::new();
        let resp = cli.head(url).send().await?;
        println!("{:?}", resp.headers());

        // let resp = reqwest::get(url).await?.bytes().await?;
        // println!("{:?}", resp);
        Ok(())
    }

    #[test]
    fn reqwest_download_test() {
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(async {
            let url = "https://official-package.wpscdn.cn/wps/download/WPS_Installer.exe";
            let cli = reqwest::Client::new();
            match cli.head(url).send().await {
                Ok(resp) => println!("{:?}", resp),
                Err(err) => println!("{}", err),
            }
        });
    }
}
