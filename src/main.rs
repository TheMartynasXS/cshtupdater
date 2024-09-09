use dialoguer::{theme::ColorfulTheme, MultiSelect, Select};
use native_dialog::FileDialog;
use std::{
	env::args,
	fs::read_to_string,
	path::{PathBuf}
};

fn main() {
	let cwd = std::env::current_dir().unwrap();

	if !exists("hash_dirs.txt"){
		std::fs::write("hash_dirs.txt", "").unwrap();
	}

	let shortcut_path = format!("{}\\force_update.bat", cwd.display());
	if std::fs::metadata(&shortcut_path).is_err() {
		let content = "cshtupdater.exe --force --skip";
		std::fs::write(&shortcut_path, content).unwrap();
		println!("Shortcut created: {}", shortcut_path);
	}

	let actions = &["Hashtable settings", "Update", "Settings", "Exit"];


	if check_flag(args().collect(), "--skip") {
		let dirs = read_lines("hash_dirs.txt");
		for dir in dirs {
			let path = PathBuf::from(dir);
			update(&path);
		}
		return;
	}

	loop {
		print!("{esc}c", esc = 27 as char);
		println!(
			"Usage: \narrows - navigate between options\nspace - select option\nenter - confirm selection"
		);
		let action = Select::with_theme(&ColorfulTheme::default())
			.with_prompt("Pick an option")
			.default(0)
			.items(&actions[..])
			.interact()
			.unwrap();

		println!("Selection: {}", action);
		if action == 0 {
			hash_setup(&cwd);
		} else if action == 1 {
			 // for each directory in hash_dirs.txt run update function
			let dirs = read_lines("hash_dirs.txt");
			for dir in dirs {
				let path = PathBuf::from(dir);
				update(&path);
			}
		} else if action == 2{

			let startup_path = format!(
				"{}\\Microsoft\\Windows\\Start Menu\\Programs\\Startup",
				std::env::var("APPDATA").unwrap()
			);
			let shortcut_path = format!("{}\\cshtupdater.bat", startup_path);

			let setting_options = &["Run on startup"];
			let selection = MultiSelect::with_theme(&ColorfulTheme::default())
					.with_prompt("Select wanted features")
					.items(&setting_options[..])
					.defaults(&[exists(&shortcut_path)])
					.interact()
					.unwrap();

			if selection.contains(&0){
				if std::fs::metadata(&shortcut_path).is_err() {
					let content = format!("cd {}\ncshtupdater.exe --skip", cwd.display());
					std::fs::write(&shortcut_path, content).unwrap();
					println!("Shortcut created: {}", shortcut_path);
				}
			}
			else {
				if exists(&shortcut_path){
					std::fs::remove_file(&shortcut_path).unwrap();
				}
			}
		}else {
			break;
		}
	}
}

fn hash_setup(cwd: &PathBuf) {
	'outer: loop {
		let dirs = read_lines("hash_dirs.txt");
		let actions = &["Add a directory", "Remove a directory", "Back"];

		let action = Select::with_theme(&ColorfulTheme::default())
				.with_prompt("Pick an option")
				.default(2)
				.items(&actions[..])
				.interact()
				.unwrap();

		match action {
			0 => {
				println!("Select a directory to add");
				let path = FileDialog::new()
						.set_location(&cwd)
						.show_open_single_dir()
						.unwrap();
				if path.is_none() {
					break 'outer;
				}
				if !(path.clone().unwrap().ends_with("hashes")
						|| path.clone().unwrap().ends_with("wad_hashtables"))
				{
					return println!("Invalid directory");
				}

				let mut new_dirs = dirs.clone();

				new_dirs.push(path.unwrap().to_str().unwrap().to_string());
				std::fs::write("hash_dirs.txt", new_dirs.join("\n")).unwrap();
			}
			1 => {
				let dirs = read_lines("hash_dirs.txt");
				if dirs.len() == 0 {
					println!("No directories to remove");
					break;
				}
				let selection = MultiSelect::with_theme(&ColorfulTheme::default())
						.with_prompt("Select directories to remove")
						.items(&dirs[..])
						.interact()
						.unwrap();
				let mut new_dirs = Vec::new();
				for (i, dir) in dirs.iter().enumerate() {
					if !selection.contains(&i) {
						new_dirs.push(dir.clone());
					}
				}
				std::fs::write("hash_dirs.txt", new_dirs.join("\n")).unwrap();
				break;
			}
			2 => {
				break 'outer;
			}
			_ => {
				println!("Invalid selection");
				break;
			}
		}
	}
}

fn exists(file_path: &str) -> bool {
	std::fs::metadata(file_path).is_ok()
}

fn join_files(file1: &str, file2: &str, file3: &str) {
	let content1 = std::fs::read_to_string(file1).unwrap();
	let content2 = std::fs::read_to_string(file2).unwrap();
	let content3 = format!("{}\n{}", content1, content2);
	std::fs::write(file3, content3).unwrap();
}

fn check_flag(args: Vec<String>, flag: &str) -> bool {
	args.iter().any(|arg| arg == flag)
}

fn read_lines(filename: &str) -> Vec<String> {
	let mut result = Vec::new();
	for line in read_to_string(filename).unwrap().lines() {
		if !result.contains(&line.to_string()) {
			result.push(line.to_string());
		}
	}

	result
}

fn update(path: &PathBuf) {
	let links =
			vec![
				"https://raw.githubusercontent.com/CommunityDragon/Data/master/hashes/lol/hashes.binentries.txt",
				"https://raw.githubusercontent.com/CommunityDragon/Data/master/hashes/lol/hashes.binfields.txt",
				"https://raw.githubusercontent.com/CommunityDragon/Data/master/hashes/lol/hashes.binhashes.txt",
				"https://raw.githubusercontent.com/CommunityDragon/Data/master/hashes/lol/hashes.bintypes.txt",
				"https://raw.githubusercontent.com/CommunityDragon/Data/master/hashes/lol/hashes.game.txt.0",
				"https://raw.githubusercontent.com/CommunityDragon/Data/master/hashes/lol/hashes.game.txt.1",
				"https://raw.githubusercontent.com/CommunityDragon/Data/master/hashes/lol/hashes.lcu.txt",
				"https://raw.githubusercontent.com/CommunityDragon/Data/master/hashes/lol/hashes.rst.txt"
			];
	let files: Vec<&str> = vec![
		"hashes.binentries.txt",
		"hashes.binfields.txt",
		"hashes.binhashes.txt",
		"hashes.bintypes.txt",
		"hashes.game.txt.0",
		"hashes.game.txt.1",
		"hashes.lcu.txt",
		"hashes.rst.txt",
	];

	let force = check_flag(args().collect(), "--force");
	for (i, link) in links.iter().enumerate() {
		//concat working directory with filename
		let file = format!("{}\\{}", path.display(), files[i]);
		let exists: bool = exists(&file);

		if !exists || force {
			let response = reqwest::blocking::get(*link).unwrap();
			std::fs::write(&file, response.text().unwrap()).unwrap();
			if force {
				println!("File updated: {}", file);
			} else {
				println!("File downloaded: {}", file);
			}
		} else {
			let recently_updated: bool = check_if_recently_updated(&file);
			if !recently_updated {
				let response = reqwest::blocking::get(*link).unwrap();
				std::fs::write(&file, response.text().unwrap()).unwrap();
				println!("File updated: {}", file);
			} else {
				println!("File updated in the last 7 days: {}", file);
			}
		}
	}
	let file1 = format!("{}\\{}", path.display(), "hashes.game.txt.0");
	let file2 = format!("{}\\{}", path.display(), "hashes.game.txt.1");
	let file3 = format!("{}\\{}", path.display(), "hashes.game.txt");
	join_files(&file1, &file2, &file3);
	println!("Compatibility files created: {}", file3);
}

fn check_if_recently_updated(file_path: &str) -> bool {
	let metadata = std::fs::metadata(file_path).unwrap();
	let modified = metadata.modified().unwrap();
	let now = std::time::SystemTime::now();
	let duration = now.duration_since(modified).unwrap();
	let secs = duration.as_secs();
	secs < 60 * 60 * 24 * 7
}