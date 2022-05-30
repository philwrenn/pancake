use super::schema::open_preferences;

#[derive(Queryable)]
pub struct OpenPreference {
    pub id: i32,
    pub search: String,
    pub browser_key: String,
    pub exact: i32,
}

#[derive(Insertable)]
#[table_name = "open_preferences"]
pub struct NewOpenPreference<'a> {
    pub search: &'a str,
    pub browser_key: &'a str,
    pub exact: i32,
}
