use std::{
	env::current_dir,
	fs::read_dir,
	path::{Path, PathBuf},
	process::Command,
};

use rand::Rng;

/// 视频的后缀
const EXTENSIONS: [&str; 10] = [
	"avi", "flv", "m2ts", "mkv", "mov", "mp4", "rmvb", "ts", "webm", "wmv",
];

fn main() {
	let mut files = Vec::new();
	// 当前路径
	let current_dir = current_dir().unwrap();
	// 遍历目录
	walk_dir(&current_dir, &mut files);
	let len = files.len();
	println!("遍历完成，共 {} 个文件", len);
	if files.is_empty() {
		return;
	}

	let mut rng = rand::thread_rng();

	loop {
		let n = rng.gen_range(0..len);
		Command::new("mpv").arg(&files[n]).status().expect("错误");
	}
}

/// 遍历目录
fn walk_dir(path: &Path, files: &mut Vec<PathBuf>) {
	for result in read_dir(path).unwrap() {
		let path_buf = match result {
			Ok(entry) => entry.path(),
			Err(_) => return,
		};
		if path_buf.is_dir() {
			walk_dir(&path_buf, files);
		} else if let Some(ext) = path_buf.extension() {
			if EXTENSIONS.contains(&ext.to_str().unwrap()) {
				files.push(path_buf);
			}
		}
	}
}
