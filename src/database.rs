use diesel::prelude::*;
use diesel::sqlite::SqliteConnection;
use dotenv::dotenv;
use std::env;

use super::models::{NewOpenPreference, OpenPreference};

pub fn get_connection() -> SqliteConnection {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

pub fn add_open_preference<'a>(
    conn: &SqliteConnection,
    search: &'a str,
    browser_key: &'a str,
    exact: i32,
) {
    use super::schema::open_preferences;

    let search_domain = get_domain_from_url(String::from(search));
    let new_open_preference: NewOpenPreference;

    if exact == 0 {
        new_open_preference = NewOpenPreference {
            search: search_domain.as_str(),
            browser_key,
            exact,
        };
    } else {
        new_open_preference = NewOpenPreference {
            search,
            browser_key,
            exact,
        };
    }

    diesel::insert_into(open_preferences::table)
        .values(&new_open_preference)
        .execute(conn)
        .expect("Error saving open preference.");
}

pub fn get_open_preference(conn: &SqliteConnection, url: &str, e: i32) -> Vec<OpenPreference> {
    use super::schema::open_preferences::dsl::*;
    if e == 0 {
        let domain = get_domain_from_url(String::from(url));
        if domain == String::from("") {
            Vec::new()
        } else {
            open_preferences
                .filter(search.like(domain + "%"))
                .filter(exact.eq(e))
                .limit(1)
                .load::<OpenPreference>(conn)
                .expect("Error getting open preference.")
        }
    } else {
        open_preferences
            .filter(search.eq(url))
            .filter(exact.eq(e))
            .limit(1)
            .load::<OpenPreference>(conn)
            .expect("Error getting open preference.")
    }
}

fn get_domain_from_url(url: String) -> String {
    let url_parts: Vec<&str> = url.split("/").collect();
    if url_parts.len() >= 3 {
        if url_parts[0] == "http:" || url_parts[0] == "https:" {
            return String::from(url_parts[0].to_owned() + "//" + url_parts[2]);
        }
    }
    String::from("")
}
