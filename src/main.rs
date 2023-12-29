use fzmovies::movie_page::{build_url::build_url, db::db_main, scraper::scp};
use fzmovies::core::client::client;
use fzmovies::download_link::db;

fn main() {
    let read = db::db_main();
    if let Ok(val) = read {
        println!("{:?}", val);
    }
}
fn movie_page_scraper() {
    for n in 1..=272 {
    let page = format!("{}", n);
    println!("page: {}", &page);
    let url = build_url("Action", "downloads", page.as_str());
    let resp = match url {
        Ok(val) => client(&val),
        Err(e) => panic!("{e}"),
    };
    let data = match resp {
        Ok(val) => scp(val),
        Err(e) => panic!("{e}"),
    };
    match db_main(data) {
        Ok(_) => println!("Success!"),
        Err(e) => println!("Error: \n\n {e}"),
    }
}   
}
