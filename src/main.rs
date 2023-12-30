use fzmovies::core::client::client;
use fzmovies::file_quality::{build_url, db, scraper};
use fzmovies::movie_page::{build_url::build_url, db::db_main, scraper::scp};

fn main() {
    let _ = download_link_scraper();
}
fn download_link_scraper() {
    let read = db::db_main();
    let urls = match read {
        Ok(val) => build_url::build_url(&val),
        Err(e) => panic!("{e}"),
    };
    let resp = match urls {
        Ok(val) => client(&val, true, Some("quality.html")),
        Err(e) => panic!("{}", e),
    };
    match resp {
        Ok(val) => scraper::scp_main(val.as_str()),
        Err(e) => panic!("{e}"),
    }
}
fn movie_page_scraper() {
    for n in 1..=272 {
        let page = format!("{}", n);
        println!("page: {}", &page);
        let url = build_url("Action", "downloads", page.as_str());
        let resp = match url {
            Ok(val) => client(&val, false, None),
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
