pub fn create_branch(branch_name:String){
    let cwd=std::Path::new(".jerry")
    if !cwd.is_dir(){
        println!("jerry not initialised");
        return;
    }
    // we search all the folder names and then see if the branch_name exists or not

    let branch_name:PathBuf=cwd.join(&branch_name);
    if branch_name.exists(){
        println!("branch name already exists");
        return
    }
    cwd.std::fs.create_dir(branch_name).expect("folder creation may fail");
}