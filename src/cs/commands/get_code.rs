pub async fn get_code(project: String, name: String, url: bool, sid: String) -> Result<(), Box<dyn std::error::Error>> {
  let endpoint = format!("https://scrapbox.io/api/code/{}/{}", project, name);


  if url {
      println!("{}", endpoint);
      return Ok(());
  }

  let client = reqwest::Client::new();
  let response = client.get(endpoint)
      .header("Cookie", format!("connect.sid={}", sid))
      .send().await?;
  if response.status().is_success() {
      let code: String = response.text().await?;
      println!("{}", code);
  }

  Ok(())
}
