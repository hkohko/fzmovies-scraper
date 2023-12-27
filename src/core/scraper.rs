use select::document::Document;
use select::predicate::{Attr, Name};
pub fn scp(page: String) {
    let document = Document::from(page.as_str());

    let main_divs = document.find(Attr("class", "mainbox"));

    let mut td_vec = Vec::new();
    for div in main_divs {
        let tds = div.find(Name("td"));
        for td in tds {
            if td.attr("style").unwrap_or("None") == "vertical-align:top;height=100px;" {
                td_vec.push(td);
            }
        }
    }

    let mut hrefs = Vec::new();
    for td in td_vec.iter() {
        let a_tags = td.find(Name("a"));
        for a in a_tags {
            hrefs.push(a.attr("href").unwrap_or("None"));
        }
    }
    dbg!(&hrefs);
}
