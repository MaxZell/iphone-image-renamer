// extern crate time;
// use time::PreciseTime;
// use std::time::{Duration, Instant};
// use std::io::{self};
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


	//start timer
	// let start_time = PreciseTime::now();

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

    // println!("Images Path: {}",my_path);
	/*
		//old
		let start_time = PreciseTime::now();
		println!("Images Path: ");
		let mut input = String::new();
		match io::stdin().read_line(&mut input) {
			Ok(n) => {
				println!("{}", input);
			}
			Err(error) => println!("error: {}", error),
		}
	*/
	let now = Instant::now();
	let _path = fs::read_dir(my_path).unwrap();
	
	// let rename_from = "IMG_";
	// let cur_date;
	// let rename_to = "IMG_" + cur_date + "_";
	
	// println!("{}", rename_to);

	//loop all files
    for image in _path {
        // println!("Name: {}", image.unwrap().path().display())
		let rename_from = format!("{}", image.unwrap().path().display());
		let cur_date = chrono::offset::Local::now().format("%Y_%m_%d_%H_%M_%S");// println!("{:?}", chrono::offset::Local::now());
		// let cur_date_format = cur_date.format("%Y_%m_%d_%H_%M_%S");
		let rename_to = format!("{}{}_", "IMG_", cur_date).to_string();
		let new_name = rename_from.replace("IMG_", &rename_to);
		// println!("old name: {}", rename_from);
		// println!("new name: {}", new_name);
		fs::rename(rename_from, new_name)?;// fs::rename("C:/Users/mzelensky/Downloads/img1.JPG", "C:/Users/mzelensky/Downloads/img2.JPG");
    }
	/*
		let d = time::now();
		let year = d.tm_year;
		let month = d.tm_mon;
		let day = d.tm_hour;
		let hour = d.tm_hour;
		let minute = d.min
		let second = d.tm_sec;
		println!("{:?}",d);
	*/

    println!("\x1b[0;32m FINISH");
	// println!(" {}  \x1b[0m", start_time.to(PreciseTime::now()));
	println!("\x1b[0;31m execute time: {} seconds\x1b[0m", now.elapsed().as_fractional_secs());
	// let in_ms = now.as_secs() * 1000 + now.subsec_nanos() as u64 / 1_000_000;
	// println!("in_ms: {}", in_ms);
	Ok(())
}