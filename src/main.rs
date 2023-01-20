/* This file contains code for a simple OBD code search tool
*
* Author: Josh McIntyre
*/
use std::io;
use std::io::*;
use std::fs::File;

// This is the main entry point for the command-line program
fn main() 
{
	menu_loop();
}

// Command line menu
fn menu_loop()
{
	loop
	{
		println!("Enter a command. Enter 'help' for help");
		let mut cmd_raw = String::new();
		io::stdin().read_line(&mut cmd_raw).expect("Unable to read input");
		
		let cmd = cmd_raw.trim();

		if cmd == "obd2"
		{
			println!("Enter the OBDII code to search");
			let mut search_code_raw = String::new();
			io::stdin().read_line(&mut search_code_raw).expect("Unable to read search code");

			let search_code = search_code_raw.trim();
			search_obd(search_code, "./obd2.txt");
		}
		if cmd == "obd1"
		{
			println!("Enter the OBDI make to search (FORD, TOYOTA, GM)");
			let mut search_make_raw = String::new();
			io::stdin().read_line(&mut search_make_raw).expect("Unable to read search make");

			let search_make = search_make_raw.trim();
			
			println!("Enter the OBDI code to search");
			let mut search_code_raw = String::new();
			io::stdin().read_line(&mut search_code_raw).expect("Unable to read search code");

			let search_code = search_code_raw.trim();

			if search_make == "FORD"
			{
				search_obd(search_code, "./obd1ford.txt");
			}
			if search_make == "TOYOTA"
			{
				search_obd(search_code, "./obd1toyota.txt");
			}
			if search_make == "GM"
			{
				search_obd(search_code, "./obd1gm.txt");
			}

		}		
		if cmd == "help"
		{
			show_help();
		}
		if cmd == "quit"
		{
			break;
		}

	}
}

// Search OBD codes
fn search_obd(search_code: &str, data_file_name: &str)
{ 
	let data_file = File::open(data_file_name).unwrap();
	let data_reader = BufReader::new(data_file);
	
	for line_raw in data_reader.lines()
	{
		let line = line_raw.unwrap();
		let (code_raw, message_raw) = line.split_once(" ").unwrap();
		
		let code = code_raw.trim();
		let message = message_raw.trim();
		
		if code == search_code
		{
			println!("Found code: {}", code);
			println!("Message is: {}", message);
			return;
		}
	}
	
	println!("Code not found: {}", search_code);
	
}

// Print help
fn show_help()
{
	println!("Commands:");
	println!("'obd2': Search OBDII codes");
	println!("'obd1': Search OBDI codes");
	println!("'help': Show this help menu");
	println!("'quit': Exit the program");
}