use select::document::Document;
use select::predicate::Name;

pub fn scp_link(page: &str) {
    let doc = Document::from(page);
    let uls = doc.find(Name("ul"));

    let mut parent_uls = Vec::with_capacity(5);
    for ul in uls {
        if ul.attr("class").unwrap_or("None") == "moviesfiles" {
            parent_uls.push(ul);
        }
    }

    let mut a_tags = Vec::with_capacity(5);
    for ul in parent_uls.iter() {
        for a in ul.find(Name("a")) {
            if a.attr("itemprop").unwrap_or("None") == "url" {
                a_tags.push(a);
            }
        }
    }

    let mut links = Vec::with_capacity(5);
    for a in a_tags.iter() {
        let text = a.text();
        let href = a.attr("href").unwrap_or("None").to_string();
        links.push((text, href))
    }
}
