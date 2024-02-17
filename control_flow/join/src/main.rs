// A join operation waits until all of a set of futures are ready, and returns a collection of
// their results. This is similar to `Promise.all` in javascript or `asyncio.gather` in Python
use anyhow::Result;
use futures::future;
use reqwest;
use std::collections::HashMap;

async fn size_of_page(url: &str) -> Result<usize> {
    let resp = reqwest::get(url).await?;
    Ok(resp.text().await?.len())
}

#[tokio::main]
async fn main() {
    let urls: [&str; 4] = [
        "https://google.com",
        "https://httpbin.org/ip",
        "https://play.rust-lang.org/",
        "BAD_URL",
    ];
    let futures_iter = urls.into_iter().map(size_of_page);
    let results = future::join_all(futures_iter).await;
    let page_sizes_dict: HashMap<&str, Result<usize>> = 
        urls.into_iter().zip(results.into_iter()).collect();
    println!("{:#?}", page_sizes_dict);
}

// For multiple disjoint types, you can use `std::future::join!` but you must know how many futures
// you will have at compile time - Currently in the `futures` create, soon to be stabilised in
// `std::future`
//
// The risk of `join` is that one of the futures may never resolve, this would cause your program
// to stall
//
// You can also combine `join_all` with `join!` for instance to join all requests to an http
// service as well as a database query. Try adding a `tokio::time::sleep` to the future, using
// `futures::join!`. This is not a timeout (that requires `select!`) but demonstrates `join`.
