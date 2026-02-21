use std::path::Path;
use std::io;
use std::fs;
use std::path::PathBuf;

// how do you return this- what is this Ok()
// and no try catch wtf
pub fn create_folder()-> Result<(), io::Error>{
    let current_pwd= Path::new(".jerry");
    // dont know about this pathBuf
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