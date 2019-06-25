#[macro_use]
extern crate lazy_static;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::io::{BufWriter, Write};

struct Date {
    y: u32,
    m: u8,
    d: u8,
}

//#[derive(Debug)]
struct Profile {
    id: i32,
    name: String,
    birthday:Date,
    home: String,
    comment: String,
}

//mod app_context {
    use std::sync::RwLock;

    lazy_static! {
        
         static ref Profile_data_store: RwLock<Vec<Profile>>
	     = RwLock::new(Vec::with_capacity(10000));

         static ref Profile_data_nitems: RwLock<i32> = RwLock::new(0);
    }
//}

fn main() {
    'l: loop{
        let line = {
	    let mut line = String::new();
	    std::io::stdin().read_line(&mut line).unwrap();
	    line.trim_end().to_owned()
	};
        parse_line(&line)
    }


}

fn parse_line(str: &String){
    let line = str.replace("\n", "\0").to_string();
    if line.starts_with("%") {
        let cmd = &line.chars().nth(1).unwrap();
	let mut param = String::new();
	if line.len() > 2 {
    	    param = line[3..].to_string();
	} 
        exec_command(*cmd, &param);
    } else {
	new_profile(&line);
    }
}

fn new_profile (csv: &String) {
    let ptr:Vec<&str> = csv.split(',').collect();
    if ptr.len() != 5 {
        println!("Invalid input.\n");
        return;
    }

    let mut profile_data_store = Profile_data_store.write().unwrap();
    let mut profile_data_nitems = Profile_data_nitems.write().unwrap();
    let ptr2:Vec<&str> = ptr[2].split('-').collect();
    if ptr2.len() != 3 {
        println!("Invalid input.\n");
        return;
    }
    if ptr2[0].len() >= 5 || ptr2[1].len() >= 3 || ptr2[2].len() >= 3 {
        println!("Invalid input.\n");
        return;
    }

    let day = Date {
        y : match ptr2[0].parse(){
	    Ok(num) => num,
	    Err(_) => {
	        println!("Invalid input.\n");
		return
		},
	    },
        m : match ptr2[1].parse(){
	    Ok(num) => num,
	    Err(_) => {
	        println!("Invalid input.\n");
		return
		},
	    },
        d : match ptr2[2].parse(){
	    Ok(num) => num,
	    Err(_) => {
	        println!("Invalid input.\n");
		return
		},
	    },
    };

    let p = Profile {
        id : match ptr[0].parse() {
	    Ok(num) => num,
	    Err(_) => {
	        println!("Invalid input.\n");
		return
		},
	    },
        name : ptr[1].to_string(),
        birthday : day,
        home : ptr[3].to_string(),
        comment : ptr[4].to_string(),
    };
    profile_data_store.push(p);
    *profile_data_nitems += 1;
}

fn exec_command(cmd:char, param:&String){
    if cmd == 'Q' {
        cmd_quit();
    } else if cmd == 'C' {
        cmd_check();
    } else if cmd == 'P' {
        cmd_print(param);
    } else if cmd == 'R' {
        cmd_read(param);
    } else if cmd == 'W' {
       cmd_write(param);
    } else if cmd == 'F' {
        cmd_find(&param);
    } else if cmd == 'S' {
        cmd_sort(param);
    } else {
	println!("Invalid command {}: ignored.\n", cmd);
    }
}

fn cmd_quit(){
    println!("Bye");
    std::process::exit(1);
}

fn cmd_check(){
    let profile_data_nitems = Profile_data_nitems.read().unwrap();
    println!("{} profile(s)\n", profile_data_nitems);
}

fn cmd_print(param: &String){
    let mut i:usize;
    let mut start = 0;
    let nitems = match param.parse(){
        Ok(num) => num,
	Err(_) => {
	    println!("Invalid argument\n");
	    return
	    },
	};
    let profile = Profile_data_nitems.read().unwrap();
    let mut end = *profile;

    if end == 0 {
        println!("No Data\n");
    };
    
    if nitems > 0 {
        end = min(nitems, *profile);
    }
    if nitems < 0 {
        let a = end + nitems;
        start = max(a, 0);
    }

    i = start as usize;

    while i < end as usize {
        print_profile(i);
	println!("\n");
	i += 1;
    }
}

fn min(a:i32, b:i32) -> i32{
    if a < b {
        return a;
    }
    b
}
fn max(a:i32, b: i32) -> i32{
    if a > b {
        return a;
    }
    b
}

fn print_profile(i: usize){
    let vec = Profile_data_store.read().unwrap();
    
    println!("Id   : {}", vec[i].id);
    println!("Name : {}", vec[i].name);
    println!("Birth: {}", date_to_string(&vec[i].birthday));
    println!("Addr : {}", vec[i].home);
    println!("Com. : {}", vec[i].comment);
}

fn date_to_string(p:&Date) -> String{

    let s = format!("{0: >04}-{1: >02}-{2: >02}", p.y, p.m, p.d);
    s
}

fn cmd_read(filename: &String){
    let f = match File::open(filename){
        Ok(t) => t,
	Err(_) => {
	    println!("Invalid argument\n");
	    return
	    },
	};
    let reader = BufReader::new(f);
    
    for (_index, line) in reader.lines().enumerate(){
        let line = line.unwrap();

        parse_line(&line);
    
    }
    println!("Read Success\n");
}

fn cmd_write(filename: &String){
    let f = match File::create(filename){
        Ok(t) => t,
	Err(_) => {
	    println!("Invalid argument\n");
	    return
	    },
	};
    let mut writer = BufWriter::new(f);
    let v = Profile_data_store.read().unwrap();
    
    for i in v.iter() {
        let contents = format!("{},{},{},{},{}\n",i.id, i.name, date_to_string(&i.birthday), i.home, i.comment);
        writer.write_all(contents.as_bytes()).unwrap();
    }
    println!("Write Success\n");
}

fn cmd_find(word: &str){
    let mut i:usize = 0;
    let profile = Profile_data_nitems.read().unwrap();
    let end = *profile as usize;
    let mut count = 0;

    while i < end {
        let vec = Profile_data_store.read().unwrap();

        if vec[i].id.to_string() == word ||
	vec[i].name == word ||
	date_to_string(&vec[i].birthday) == word ||
        vec[i].home == word ||
        vec[i].comment == word {
            print_profile(i);
     	    println!("\n");
	    count += 1;
	} 
	i += 1;
    }
    if count == 0 {
        println!("Not Found\n");
    }

}

fn cmd_sort(column:&String){
    let n:u8 = match column.parse(){
        Ok(num) => num,
	Err(_) => {
	    println!("Invalid argument\n");
	    return
	},
    };
    let mut v = Profile_data_store.write().unwrap();
    match n {
        1 => {
	    v.sort_by(|a,b| a.id.cmp(&b.id));
	    println!("Sort by ID\n")
	    },
	2 => {
	    v.sort_by(|a,b| a.name.cmp(&b.name));
	    println!("Sort by Name\n")
	    },
	3 => {
	    v.sort_by(|a,b| date_to_string(&a.birthday).cmp(&date_to_string(&b.birthday)));
	    println!("Sort by Birthday\n")
	    },
	4 => {
	    v.sort_by(|a,b| a.home.cmp(&b.home));
	    println!("Sort by Home\n")
	    },
	5 => {
	    v.sort_by(|a,b| a.comment.cmp(&b.comment));
	    println!("Sort by Comment\n")
	    },
	_ => {
	    println!("Invalid argument\n");
	    return
	}
    }
}

