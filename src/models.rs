use crate::schema::files;
use diesel::prelude::{Insertable, Queryable};

#[derive(Queryable)]
pub struct File {
	pub id: i32,
	pub path: String,
}

#[derive(Insertable)]
#[table_name = "files"]
pub struct NewFile<'a> {
	pub path: &'a str,
}
