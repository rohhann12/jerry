use std::path::Path;
use std::io;
use std::fs;
use std::path::PathBuf;

pub fn create_folder()-> Result<(), io::Error>{
    let current_pwd= Path::new(".jerry");
    let srcdir = PathBuf::from("./");
    println!("{:?}", fs::canonicalize(&srcdir));
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