use serde::Serialize;
use std::fs;
use toml::Value;

#[derive(Serialize)]
pub struct Config {
    pub theme: String,
    pub browsers: Vec<Browser>,
}

#[derive(Serialize)]
pub struct Browser {
    pub key: String,
    pub display: String,
    pub exec: Vec<String>,
    pub envs: Vec<(String, String)>,
}

impl Config {
    pub fn from_file() -> Config {
        let mut config_path = dirs::config_dir().unwrap().join("pancake");
        if !config_path.exists() {
            fs::create_dir(&config_path).unwrap();
        }

        config_path = config_path.join("pancake.toml");

        if !config_path.exists() {
            fs::File::create(&config_path).unwrap();
        }

        let contents = fs::read_to_string(config_path.to_str().unwrap())
            .expect("Something went wrong reading the file");
        let toml_value = contents.parse::<Value>().unwrap();
        Config::from_toml(toml_value)
    }

    pub fn from_toml(toml_value: Value) -> Config {
        let mut theme = String::from("default");
        let mut browsers = vec![];

        match &toml_value {
            Value::Table(s) => {
                for (config_name, config_val) in s.into_iter() {
                    match config_name.as_str() {
                        "theme" => theme = Config::parse_theme_value(config_val),
                        "browsers" => browsers = Config::parse_browser_list(config_val),
                        _ => {}
                    }
                }
            }
            _ => println!("Non-Table as top level item in toml. {:?}", toml_value),
        }
        Config { theme, browsers }
    }

    fn parse_theme_value(config_val: &Value) -> String {
        match config_val {
            Value::String(s) => s.to_owned(),
            _ => String::from("default"),
        }
    }

    fn parse_browser_list(config_val: &Value) -> Vec<Browser> {
        let mut browsers = vec![];

        match config_val {
            Value::Array(browser_array) => {
                for browser in browser_array {
                    browsers.push(Config::parse_browser(browser));
                }
            }
            _ => {}
        }
        browsers
    }

    fn parse_browser(config_val: &Value) -> Browser {
        let mut key = String::from("");
        let mut display = String::from("");
        let mut exec = vec![];
        let mut envs = vec![];

        match config_val {
            Value::Table(s) => {
                for (browser_config_name, browser_config_value) in s.into_iter() {
                    match browser_config_value {
                        Value::String(s) => match browser_config_name.as_str() {
                            "display" => {
                                display = s.to_owned();
                            }
                            "key" => {
                                key = s.to_owned();
                            }
                            _ => {}
                        },
                        Value::Array(a) => match browser_config_name.as_str() {
                            "exec" => {
                                for exec_part in a {
                                    match exec_part {
                                        Value::String(s) => exec.push(s.to_owned()),
                                        _ => {}
                                    }
                                }
                            }
                            "envs" => {
                                for env in a {
                                    match env {
                                        Value::String(s) => {
                                            let env_parts: Vec<&str> = s.split("=").collect();
                                            if env_parts.len() == 2 {
                                                envs.push((
                                                    String::from(env_parts[0]),
                                                    String::from(env_parts[1]),
                                                ))
                                            }
                                        }
                                        _ => {}
                                    }
                                }
                            }
                            _ => {}
                        },
                        _ => {}
                    }
                }
            }
            _ => {}
        }
        Browser {
            key,
            display,
            exec,
            envs,
        }
    }
}
