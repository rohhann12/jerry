use std::path::Path;
use std::io;

pub fn create_folder()-> Result<(), io::Error>{
    let current_pwd= Path::new(".jerry");
    println!("pwd: {:?}", current_pwd);
    if current_pwd.is_dir(){
        println!("already exists");
        Ok(())
    }else{
        std::fs::create_dir(".jerry")?;
        println!("created directory");
        Ok(())
    }
}