use std::io::ErrorKind;
use std::path::PathBuf;
use std::process::Command;
use std::{env, fs};

use diesel::{Connection, QueryDsl, RunQueryDsl, SqliteConnection};
use rand::Rng;

use crate::models::NewFile;
use crate::schema::files::dsl::files;

mod models;
mod schema;

/// 视频的后缀
const EXTENSIONS: [&str; 10] = [
	"avi", "flv", "m2ts", "mkv", "mov", "mp4", "rmvb", "ts", "webm", "wmv",
];

fn main() {
	// 当前路径
	let current_dir = env::current_dir().unwrap();

	let connection = establish_connection();
	// 删除旧数据
	diesel::delete(schema::files::table)
		.execute(&connection)
		.expect("删除失败");

	// 遍历目录
	iterator_dir(&connection, &current_dir);

	// 加载所有视频的路径
	let videos_path: Vec<String> = files
		.select(schema::files::columns::path)
		.load::<String>(&connection)
		.expect("无法加载 videos_path");

	let mut rng = rand::thread_rng();
	let len = videos_path.len().to_owned();

	if len < 1 {
		println!("未找到视频文件");
		return;
	}
	for _ in 1..100 {
		let n = rng.gen_range(0..len);
		play(&videos_path[n]);
	}
}

fn play(file: &str) {
	println!("{}", file);
	Command::new("mpv").arg(file).status().expect("错误");
}

#[allow(deprecated)]
pub fn establish_connection() -> SqliteConnection {
	let mut home = env::home_dir().unwrap();
	home.push(".config/RandomPlay.sqlite");
	let database_url = home.to_str().unwrap();
	SqliteConnection::establish(database_url)
		.unwrap_or_else(|_| panic!("无法连接： {}", database_url))
}

/// 保存找到的视频路径
#[allow(unused_must_use)]
fn create_file(conn: &SqliteConnection, path: &str) {
	diesel::insert_into(schema::files::table)
		.values(NewFile { path })
		.execute(conn);
}

/// 遍历目录
fn iterator_dir(conn: &SqliteConnection, path: &PathBuf) {
	let read_dir = fs::read_dir(&path);
	let read_dir = match read_dir {
		Ok(file) => file,
		Err(err) => {
			if err.kind() != ErrorKind::PermissionDenied {
				println!("错误类型：{:?}\n错误信息：{}", err.kind(), err);
			}
			return;
		}
	};

	for result_entry in read_dir {
		// 如果错误则跳过本次操作
		if result_entry.is_err() {
			continue;
		}

		let file = result_entry.unwrap().path();
		if file.is_dir() {
			// 目录，继续遍历
			iterator_dir(conn, &file);
		} else {
			// 文件，判断后缀
			let option_extension = file.extension();
			// 如果存在后缀
			if let Some(extension) = option_extension {
				let match_extension = &EXTENSIONS.iter().position(|&x| x == extension);
				// 如果是视频的后缀
				if match_extension.is_some() {
					// 保存路径
					create_file(conn, &file.to_str().unwrap());
				}
			}
		}
	}
}
