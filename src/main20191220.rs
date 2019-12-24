fn main() {
    use walkdir::WalkDir;

    let directory = "C:\\Users\\joekr\\Programs\\soswg_site\\";
    let mut files = Vec::new();
    let mut not_dirs = Vec::new();

    for entry in WalkDir::new(directory) {
        let entry = entry.unwrap();
        files.push(entry);
    }

    for i in 0..files.len() {
        if files[i].metadata().unwrap().is_file() {
            let not_dir = &files[i];
            println!("{:?} is a file", files[i]);
            not_dirs.push(not_dir);
        }
    }
    
    for e in 0..not_dirs.len() {
        println!("{:?}", not_dirs[e]);
    }


    /*
    println!("File name:{:?}", files[0].file_name());
    println!("File? {:?}", files[0].metadata().unwrap().is_file());
    println!("Directory? {:?}", files[0].metadata().unwrap().is_dir());
    println!("{:?}", files[1]);
    println!("{:?}", files[2]);
    println!("{:?}", files[3]);
*/

    // if file is a directory, then do nothing for now
    // if file is a file, then append it into a copy of index.html
}