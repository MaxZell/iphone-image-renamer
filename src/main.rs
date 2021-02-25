use std::io::{stdin,stdout,Write};
use std::fs;

use std::time::{Instant};
use chrono;
use floating_duration::TimeAsFloat;

fn main() -> std::io::Result<()>{
	
	/*
		Time measurence(12 images):
			python: 0.0 1 5 0 4 7 5 5 0
			rust:   0.0 0 5 4 8 1 8 7 4
			0.015047550 / 0.005481874 = 2.7418068851
	*/

	//colors: https://bixense.com/clicolors/
	println!("\x1b[0;32m START");
	//get user input
    let mut my_path=String::new();
    print!("\x1b[0;34m Please enter images path: \x1b[0m");
    let _=stdout().flush();
    stdin().read_line(&mut my_path).expect("Did not enter a correct string");
    if let Some('\n')=my_path.chars().next_back() {
        my_path.pop();
    }
    if let Some('\r')=my_path.chars().next_back() {
        my_path.pop();
    }

    //start timer
	let now = Instant::now();
	let _path = fs::read_dir(my_path).unwrap();

	//loop all files
    for image in _path {
		let rename_from = format!("{}", image.unwrap().path().display());
		let cur_date = chrono::offset::Local::now().format("%Y_%m_%d_%H_%M_%S");// println!("{:?}", chrono::offset::Local::now());
		let rename_to = format!("{}{}_", "IMG_", cur_date).to_string();
		let new_name = rename_from.replace("IMG_", &rename_to);
		// println!("old name: {}", rename_from);
		// println!("new name: {}", new_name);
		fs::rename(rename_from, new_name)?;
    }

    println!("\x1b[0;32m FINISH");
	println!("\x1b[0;31m execute time: {} seconds\x1b[0m", now.elapsed().as_fractional_secs());
	Ok(())
}