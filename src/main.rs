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

fn get_user_lib<F: FnMut(String)>(args: Option<&String>, mut f: F) {
	match args {
		Some(c) => {
			return f(c.clone())
		}
		None => {
		    let stdin = io::stdin();
		    for line in stdin.lock().lines() {
		        if let Ok(content) = line {
		            f(content);
		        }
		    }		
		}
	}
}

fn main() {
	let args: Vec<String> = std::env::args().collect();
	if args.len() == 1 {
		println!("Please atleast give us a path to a dir...");
		return;
	}
	let first_arg = args[1].clone();
	match first_arg.as_str() {
		"init" => {
			match std::fs::create_dir_all(".link") {
				Ok(_) => {
					println!("Initialized .link folder")			
				}
				Err(e) => {
					println!("Error!: {}", e);
				}
			}
		}
		path => {
			match std::fs::read_dir(path) {
				Err(_) => {
					println!("Path [{}] does not exist!", &args[1]);
				}
				Ok(read_dir) => {
					match args.get(2).map(String::as_str) {
						None => {
							for entry in read_dir {
	    						let entry = entry.unwrap();
	    						let dir_name = entry.path();
								println!("{}", dir_name.file_name().unwrap().display())
	    					}
						}
						Some("register") => {
							get_user_lib(args.get(3), |lib| {
								let mut path = std::path::PathBuf::from(".link");
				    			path.push(&lib);
				    			let mut crate_path = std::path::PathBuf::from(&first_arg);
								crate_path.push(&lib);
								match symlink(&crate_path, &path) {
									Err(e) => {
										println!("Error!: {}", e);
									}
									Ok(_) => {
										println!("Successfully created a symlink for {} in {}", crate_path.display(), path.display());
									}
								}
							});
							// match get_user_lib(args.get(3)) {
							// 	None => {
							// 		println!("Atleast give us a name of your lib!");
							// 	}
							// 	Some(libs) => {
							// 		println!("{}", libs);
							// 		for lib in libs.split("\n") {
						 //    				
							// 		}
							// 	}
							// }
						}
						Some(s) => {
							println!("Error: Unknown argument {}!", s);
						}
					}
				}
			}		
		}
	}
}
