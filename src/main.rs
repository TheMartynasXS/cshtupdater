use std::env::args;

fn main() {
    //get current working directory
    let cwd = std::env::current_dir().unwrap();

    let first_run = check_file_exists("config.ini");
    if !first_run {
        //Ask whether to create compatibility files and store it in config.ini
        println!("Do you want to create compatibility files (obsidian <6.0.0)? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "y" {
            std::fs::write("config.ini", "compatibility=true").unwrap();
        } else {
            std::fs::write("config.ini", "compatibility=false").unwrap();
            println!("Compatibility files disabled.");
        }
        println!("Do you want to enable weekly updates? (y/n)");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();
        if input == "y" {
            let startup_path = format!(
                "{}\\Microsoft\\Windows\\Start Menu\\Programs\\Startup",
                std::env::var("APPDATA").unwrap()
            );
            let shortcut_path = format!("{}\\cshtupdater.bat", startup_path);
            if std::fs::metadata(&shortcut_path).is_err() {
                //join cwd with filename and
                let content = format!("{}\\cshtupdater.exe --skip", cwd.display());
                std::fs::write(&shortcut_path, content).unwrap();
                println!("Shortcut created: {}", shortcut_path);
            }
        } else {
            println!("Weekly updates disabled.");
        }
    }
    let compatibility: bool = std::fs::read_to_string("config.ini")
        .unwrap()
        .contains("compatibility=true");

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

    // Check if force_update.bat exists, if not, create it
    let shortcut_path = format!("{}\\force_update.bat", cwd.display());
    if std::fs::metadata(&shortcut_path).is_err() {
        let content = "cshtupdater.exe --force";
        std::fs::write(&shortcut_path, content).unwrap();
        println!("Shortcut created: {}", shortcut_path);
    }
    let force = args().any(|arg| arg == "--force");
    for (i, link) in links.iter().enumerate() {
        //concat working directory with filename
        let file = format!("{}\\{}", cwd.display(), files[i]);
        let exists: bool = check_file_exists(&file);

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

    if compatibility {
        let file1 = format!("{}\\{}", cwd.display(), "hashes.game.txt.0");
        let file2 = format!("{}\\{}", cwd.display(), "hashes.game.txt.1");
        let file3 = format!("{}\\{}", cwd.display(), "hashes.game.txt");
        join_files(&file1, &file2, &file3);
        println!("Compatibility files created: {}", file3);
    }

    println!("If you want to force update the files, run force_update.bat.");

    //wait for user input to close the program
    if !(args().any(|arg| arg == "--skip")) {
        println!("Press enter to close the program...");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
    }
}

fn check_file_exists(file_path: &str) -> bool {
    std::fs::metadata(file_path).is_ok()
}
fn check_if_recently_updated(file_path: &str) -> bool {
    let metadata = std::fs::metadata(file_path).unwrap();
    let modified = metadata.modified().unwrap();
    let now = std::time::SystemTime::now();
    let duration = now.duration_since(modified).unwrap();
    let secs = duration.as_secs();
    return secs < 60 * 60 * 24 * 7;
}
//join two files into third file
fn join_files(file1: &str, file2: &str, file3: &str) {
    let content1 = std::fs::read_to_string(file1).unwrap();
    let content2 = std::fs::read_to_string(file2).unwrap();
    let content3 = format!("{}\n{}", content1, content2);
    std::fs::write(file3, content3).unwrap();
}
