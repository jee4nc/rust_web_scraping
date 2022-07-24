mod services;

fn main() {
    services::scraper::hello_world();
    let a = services::scraper::suma(1, 2);
    println!("{}", a);
    let url = "https://www.google.com";
    services::scraper::srapy_url(url).unwrap();
}

