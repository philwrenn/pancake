#![windows_subsystem = "windows"]
#[macro_use]
extern crate diesel;
extern crate dotenv;

use crate::config::Browser;
use serde::{Deserialize, Serialize};
use simplelog::*;
use std::fs::OpenOptions;
use std::process::Command;
use web_view::*;

mod config;
mod database;
mod models;
mod schema;

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "cmd")]
enum Cmd {
    Init,
    LaunchBrowser {
        exec: Vec<String>,
        envs: Vec<(String, String)>,
        key: String,
        remember: String,
    },
    Exit,
}

fn inline_style(s: &str) -> String {
    format!(r#"<style type="text/css">{}</style>"#, s)
}

fn inline_script(s: &str) -> String {
    format!(r#"<script type="text/javascript">{}</script>"#, s)
}

fn html_src() -> String {
    format!(
        r#"
        <html>
        <head>
            <script>if (window.external == undefined) {{window.external={{invoke:function(x){{window.webkit.messageHandlers.external.postMessage(x);}}}};}}</script>
            <link id="dark-theme-style" rel="stylesheet" />
        </head>
        <body id="pancake" class="light">
            {styles}
            {bootstrap}
            <select class="form-select" id="browserSelect" multiple="true"></select>
            <div id="rememberRow">
                <select class="form-select" id="rememberSelect">
                    <option value="">Don't Remember</option>
                    <option value="remember_exact">Remember Exact URL</option>
                    <option value="remember_domain">Remember Domain</option>
                </select>
            </div>
            <div id="buttonRow">
            <button id="openBrowser" class="btn btn-primary">Open Browser</button>
            <button id="exitButton" class="btn btn-secondary">Exit</button>
            </div>
            {scripts}
        </body>
        </html>
        "#,
        styles = inline_style(include_str!("../assets/style.css")),
        bootstrap = inline_style(include_str!("../assets/bootstrap.min.css")),
        scripts = inline_script(include_str!("../assets/index.js"))
    )
}

fn main() {
    let log_file = OpenOptions::new()
        .read(true)
        .write(true)
        .append(true)
        .open("/tmp/pancake.log");

    match log_file {
        Ok(l) => {
            WriteLogger::init(LevelFilter::Info, simplelog::Config::default(), l).unwrap();
        }
        Err(_) => {}
    }

    let url = match std::env::args().nth(1) {
        Some(u) => u,
        None => String::from(""),
    };

    builder()
        .title("Pancake - Browser Launcher")
        .content(Content::Html(html_src()))
        .size(600, 400)
        .resizable(true)
        .debug(true)
        .user_data(())
        .frameless(false)
        .invoke_handler(|webview, arg| match serde_json::from_str(arg).unwrap() {
            Cmd::LaunchBrowser {
                mut exec,
                envs,
                key,
                remember,
            } => {
                if &url != "" {
                    exec.push(url.clone());
                    if remember == "remember_domain" || remember == "remember_exact" {
                        let conn = database::get_connection();
                        let remember_exact: i32 = match &remember[..] {
                            "remember_exact" => 1,
                            _ => 0,
                        };
                        database::add_open_preference(&conn, &url, &key, remember_exact);
                    }
                }
                let args: Vec<_> = exec.drain(1..).collect();

                Command::new(&exec[0])
                    .args(args)
                    .envs(envs)
                    .spawn()
                    .unwrap();
                Ok(())
            }
            Cmd::Init => {
                let config = config::Config::from_file();
                let mut browser: Option<&Browser> = None;

                if &url != "" {
                    let conn = database::get_connection();
                    let exact_match = database::get_open_preference(&conn, url.as_str(), 1);

                    if exact_match.len() == 1 {
                        browser = config
                            .browsers
                            .iter()
                            .filter(|b| b.key == exact_match[0].browser_key)
                            .next();
                    } else {
                        let pattern_match = database::get_open_preference(&conn, url.as_str(), 0);
                        if pattern_match.len() >= 1 {
                            browser = config
                                .browsers
                                .iter()
                                .filter(|b| b.key == pattern_match[0].browser_key)
                                .next();
                        }
                    }
                }

                if let Some(b) = browser {
                    let mut exec = b.exec.clone();
                    if &url != "" {
                        exec.push(url.clone());
                    }
                    let args: Vec<_> = exec.drain(1..).collect();
                    Command::new(&b.exec[0])
                        .args(args)
                        .envs(b.envs.clone())
                        .spawn()
                        .unwrap();
                    webview.exit();
                } else {
                    webview
                        .eval(&format!(
                            "app.init({})",
                            serde_json::to_string(&config).unwrap()
                        ))
                        .unwrap();
                }
                Ok(())
            }
            Cmd::Exit => {
                webview.exit();
                Ok(())
            }
        })
        .run()
        .unwrap();
}
