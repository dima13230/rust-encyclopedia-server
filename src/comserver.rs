use crate::settings::Settings;

use async_std::fs;
use std::fs::metadata;

use anyhow::Result;
use tokio_graceful_shutdown::SubsystemHandle;

use std::str;
use std::collections::HashMap;
use warp::{Filter, http::Response};

use async_walkdir::WalkDir;
use futures_lite::stream::StreamExt;

use iuliia_rust::parse_by_schema_name;
use urlencoding::{encode, decode};

async fn article_out(url: String) -> Result<impl warp::Reply, warp::Rejection> {
    let settings: &Settings = &*crate::settings::CONFIG;

    let mut entries = WalkDir::new(&settings.server.articles_path);
    loop {
        match entries.next().await {
            Some(Ok(entry)) => {
                let p = entry.path().display().to_string().clone();

                let md = metadata(&p).unwrap();
                if md.is_file() {
                    let contents = fs::read_to_string(&p)
                        .await
                        .expect("Failed to obtain contents of article file!");
                       
                    let mut value = (*encode(&iuliia_rust::parse_by_schema_name(&p, "wikipedia"))).to_string();
                    value = value[11..value.chars().count() - 3].to_string();

                    if url.eq(&value) {
                        return Ok(warp::reply::json(&contents));
                    }
                }
            }
            Some(Err(e)) => {
                eprintln!("error: {}", e);
                break;
            }
            None => break,
        }
    }
    Err(warp::reject())
}

async fn articles_list_out() -> Result<impl warp::Reply, warp::Rejection> {
    let settings: &Settings = &*crate::settings::CONFIG;
    let mut result: HashMap<String, String> = HashMap::new();

    let mut entries = WalkDir::new(&settings.server.articles_path);
    loop {
        match entries.next().await {
            Some(Ok(entry)) => {
                let p = entry.path().display().to_string().clone();

                let md = metadata(&p).unwrap();
                if md.is_file() {
                    let value = (*encode(&iuliia_rust::parse_by_schema_name(&p, "wikipedia"))).to_string();
                    result.insert(value[11..value.len() - 3].to_string(), p[9..p.len() - 3].to_string());
                }
            }
            Some(Err(e)) => {
                eprintln!("error: {}", e);
                break;
            }
            None => break,
        }
    }
    Ok(warp::reply::json(&result))
}

// Listen for the requests from the site and operate accordingly
async fn listen() -> Result<()> {
    let settings: &Settings = &*crate::settings::CONFIG;

    let articles = warp::path!("api" / "articles")
    .and_then(articles_list_out);

    let article = warp::path!("api" / "articles" / String)
    .and_then(|s| {
        article_out(s)
    });

    let routes = articles
    .or(article);

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
    Ok(())
}

// Handle execution of the communication server
pub async fn comserver(subsys: SubsystemHandle) -> Result<()> {
    log::info!("comserver is created");
    tokio::select! {
        _ = subsys.on_shutdown_requested() => {
            log::info!("comserver stopped by external shutdown");
        },
        _ = listen() => {
            log::info!("comserver stopped on its own");
        }
    }

    Ok(())
}
