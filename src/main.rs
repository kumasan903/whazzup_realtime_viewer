use std::fs::File;
use std::io::prelude::*;
use std::{thread, time};
extern crate dirs;

fn main() {
	let mut path = dirs::home_dir().unwrap();
	path.push("AppData/Local/JoinFS/whazzup.txt");
	loop
	{
		let sleep_time = time::Duration::from_secs(1);
		thread::sleep(sleep_time);
		//let mut f = File::open("/mnt/c/Users/kawan/AppData/Local/JoinFS/whazzup.txt")
		//let mut f = File::open("~/AppData/Local/JoinFS/whazzup.txt")
		let mut f = File::open(&path)
			.expect("ファイルが見つかりません");
		println!("\x1B[2J");
		println!("\x1B[r");
		let mut contents = String::new();
		f.read_to_string(&mut contents)
			.expect("something went wrong reading the file");
		let v: Vec<&str> = contents.split('\n').collect();
		let mut i :usize;
		let clients_count_line :Vec<&str> = v[4].split(' ').collect();
		let clients_count :usize = clients_count_line[3].trim().parse().unwrap();
		println!("Callsign\tName\t\tP/C\tHDG\tAlt\tSpd\tAcType\tFreq\tLatitude\tLongitude\tSquawk\tRule");
		i = 7;
		while i < (clients_count + 7)
		{
			let tmp :Vec<&str> = v[i].split(':').collect();
			let call_len = tmp[0].chars().count();
			if call_len < 8 {
				print!("{}\t\t",tmp[0]);	//Callsigin
			} else {
				print!("{}\t",tmp[0]);	//Callsigin
			}
			let name_len = tmp[1].chars().count();
			if name_len < 8 {
				print!("{}\t\t",tmp[1]);	//Callsigin
			} else {
				print!("{}\t",tmp[1]);	//Callsigin
			}
			//print!("{}\t",tmp[1]);	//Name
			print!("{}\t",tmp[3]);	//pilot or controller
			print!("{}\t",tmp[38]);	//Heading
			print!("{}\t",tmp[7]);	//alt
			print!("{}\t",tmp[8]);	//spd
			print!("{}\t",tmp[9]);	//ACType
			print!("{}\t",tmp[4]);	//freq
			let lat :f32 = tmp[5].trim().parse().unwrap();
			print!("{:<10}\t",lat);	//lat
			let lon :f32 = tmp[6].trim().parse().unwrap();
			print!("{:<10}\t",lon);	//lon
			print!("{}\t",tmp[17]);	//Squawk
			print!("{}\t",tmp[21]);	//Rule
			//println!("{}",v[i].to_string());
			print!("\n");
			i += 1;
		}
	}
}