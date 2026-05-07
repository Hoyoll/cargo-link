use std::path::Path;
fn symlink<P: AsRef<Path>, Q: AsRef<Path>>(
    src: P,
    dst: Q,
) -> std::io::Result<()> {
    #[cfg(unix)]
    {
        std::os::unix::fs::symlink(src, dst)
    }

    #[cfg(windows)]
    {
        let src = src.as_ref();
        if src.is_dir() {
            std::os::windows::fs::symlink_dir(src, dst)
        } else {
            std::os::windows::fs::symlink_file(src, dst)
        }
    }
}

use std::io::{self, BufRead};

fn get_user_crate(args: &[String]) -> Option<String> {
    if let Some(c) = args.get(2) {
        return Some(c.clone());
    }
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        if let Ok(content) = line {
            return Some(content);
        }
    }
    None
}

fn main() {
	let mut args: Vec<String> = std::env::args().collect();
	match args.get(1) {
		None => {}
		Some(arg) => {
			match arg.as_str() {
				"list" => {
					match home::cargo_home() {
						Err(_) => {
							println!("[Error]: Could not found cargo!")
						}
						Ok(mut cargo_home) => {
						    cargo_home.push("registry");
						   	cargo_home.push("src");
							match std::fs::read_dir(cargo_home) {
						    	Ok(dir) => {
						    		for entry in dir {
						    			let entry = entry.unwrap();
						    			let dir_name = entry.path();
						    			if !dir_name.file_name().unwrap().to_str().unwrap().starts_with("index.crates.io") {
						    				continue
						    			}
						    			match std::fs::read_dir(dir_name) {
						    				Err(_) => {}
						    				Ok(dir) => {
						    					for entry in dir {
						    						let entry = entry.unwrap();
						    						let dir_name = entry.path();
													println!("{}", dir_name.file_name().unwrap().display())
						    					}	
						    				}
						    			} 
						    		}
						    	}
						    	Err(_) => ()
						    }
						}
					}		
				}
				"custom" => {
				}
				"register" => {
					match get_user_crate(&args) {
						None => {
							println!("here!");
						},
						Some(user_crate) => {
							match home::cargo_home() {
								Err(_) => {
									println!("[Error]: Could not found cargo!")
								}
								Ok(mut cargo_home) => {
								    cargo_home.push("registry");
								   	cargo_home.push("src");
									match std::fs::read_dir(cargo_home) {
								    	Ok(dir) => {
								    		for entry in dir {
								    			let entry = entry.unwrap();
								    			let mut dir_name = entry.path();
								    			if !dir_name.file_name().unwrap().to_str().unwrap().starts_with("index.crates.io") {
								    				continue
								    			}
								    			dir_name.push(&user_crate);
								    			std::fs::create_dir_all(".link");
								    			let mut path = std::path::PathBuf::from(".link");
								    			path.push(&user_crate);
								    			symlink(dir_name, path);
								    		}
								    	}
								    	Err(_) => ()
								    }
								}
							}
						}
					}					
				}
				_ => ()			
			}
		}
	}
}			
