use std::path::Path;
use std::fs;
use std::io;


fn rename_dir_files(path: &Path) -> io::Result<()> {
    match fs::read_dir(path) {
        Ok(entries) => {
            // if entries.filter_map(Result::ok).any(|entry| entry.file_type().map(|ft| !ft.is_file()).unwrap_or(true)) {
            //     // At least one entry is not a file
            //     println!("Directory contains non-file entries.");
            //     std::process::exit(-1);
            // }

            println!("Please enter your filename preface:");

            let mut name = String::new();

            io::stdin()
                .read_line(&mut name)
                .expect("Failed to read line");

            let preface = name.trim();
            let mut counter = 1;


            for entry in entries {
                let entry = entry?;
                let entry_path = entry.path();
                let extension = entry_path.extension().and_then(|e| e.to_str()).unwrap_or_default();
                let new_name = format!("{}-{}.{}", preface, counter, extension);
                let new_file_path = entry_path.with_file_name(new_name);

                let result = fs::rename(entry_path, new_file_path);
                
                match result {
                    Ok(_) => {
                        counter += 1;
                    },
                    Err(e) => {
                        println!("could not rename {}", e)
                    }
                }
            }

            println!("All files renamed!");

        },
        Err(e) => {
            println!("Error reading directory: {}", e);
        }
    }
    Ok(())
}


fn main() -> io::Result<()> {
    let arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    let path = Path::new(&arg);

    if !path.exists() || !path.is_dir() {
        println!("The folder does not exist.");
        std::process::exit(-1);
    }

    rename_dir_files(path)?;

    Ok(())
}
