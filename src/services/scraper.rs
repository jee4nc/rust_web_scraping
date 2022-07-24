pub fn hello_world() {
    println!("Hello, world!");
}

pub fn suma(a: i32, b: i32) -> i32 {
    a + b
}

pub async fn srapy_url(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    println!("body = {:?}", body);
}