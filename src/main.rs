use fzmovies::core::{build_url::build_url, client::client, scraper::scp, db::db_main};

fn main() {
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
