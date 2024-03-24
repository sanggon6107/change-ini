use walkdir::WalkDir;
use std::env;
use colored::Colorize;
use std::fs::{self, write};
use regex::Regex;


fn change_ini(file_name : &str, setting_name : &str, setting_value : &str) -> std::io::Result<()>{
    let contents = fs::read_to_string(&file_name).expect("Failed to read a file");

    let mut reg_target = String::from("\\r\\n");
    reg_target.push_str(&setting_name);
    reg_target.push_str(".*=.+\\r\\n");
    let re = Regex::new(&reg_target).unwrap();
    
    let mut result_target : String = String::from("\r\n");
    result_target.push_str(&setting_name);
    result_target.push_str("=");
    result_target.push_str(&setting_value);
    result_target.push_str("\r\n");

    let new_reg = re.replace_all(&contents, &result_target);

    //dbg!(&contents, &new, &new_reg);
    write(file_name, new_reg.as_bytes())?;

    Ok(())
}

fn main() -> std::io::Result<()>{
    println!("{}", "*** INITIALIZE INI CHANGE ***".green());
    println!("{} : {}", "Current Dir".yellow(), env::current_dir()?.display());

    loop {
        println!("{}", "Which setting do you want to change?".yellow());
        let mut buffer_in_setting_name = String::new();
        let mut buffer_in_setting_value = String::new();


        std::io::stdin().read_line(&mut buffer_in_setting_name).unwrap();
       if buffer_in_setting_name.contains("EXIT\n") {
            println!("{}", "*** PROGRAM TERMINATED ***".yellow());
            break;
        }
        buffer_in_setting_name.truncate(buffer_in_setting_name.len() - 1);
        
        println!("{} {}", (&buffer_in_setting_name).yellow(), "will be set to : ".yellow());
        std::io::stdin().read_line(&mut buffer_in_setting_value).unwrap();
        buffer_in_setting_value.truncate(buffer_in_setting_value.len() - 1);

        for entry in WalkDir::new(env::current_dir().unwrap().to_str().unwrap().to_string()).into_iter().filter_map(|e| e.ok()) {
            
            let file_name = entry.path().to_str().unwrap().to_string();
            if !file_name.ends_with(".ini") { continue; }

            println!("{}", entry.path().display());
            change_ini(&file_name, &buffer_in_setting_name, &buffer_in_setting_value)?;
        }

    }
   Ok(())
}
