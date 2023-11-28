use crate::core::path::ProjPath;
use anyhow::Result;
use scraper::{ElementRef, Html, Selector};
use std::io::Read;

pub fn scp(page: String) -> Result<()> {
    let main_div = {
        let div_selector = Selector::parse("div").expect("div Selector Error");
        let td_selector = Selector::parse("td").expect("td Selector Error");
        let a_selector = Selector::parse("a").expect("'a' Selector Error");
        let doc = Html::parse_document(&page);
        let divs: Vec<ElementRef> = doc
            .select(&div_selector)
            .filter(|div| div.attr("class") == Some("mainbox"))
            .collect();
        let tds: Vec<ElementRef> = divs
            .iter()
            .flat_map(|div| div.select(&td_selector))
            .filter(|td| td.attr("style") == Some("vertical-align:top;height=100px;"))
            .collect();
        let a: Vec<Option<&str>> = tds
            .iter()
            .flat_map(|td| td.select(&a_selector))
            .filter(|a| a.attr("href").is_some())
            .map(|a| a.attr("href"))
            .collect();
        for atag in a {
            dbg!(atag);
        }
    };
    Ok(())
}
