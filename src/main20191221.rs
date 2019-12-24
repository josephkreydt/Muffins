use std::fs;
use std::path::Path;

fn main() {

    let directory: Path = "C:\\Users\\joekr\\Programs\\joekreydt_web\\";

    //function to walk directory recursively
    fn get_files(dir: &Path) -> Vec<&Path> {
        let paths = fs::read_dir(dir).unwrap();
        let mut files = Vec::new();
        let mut scooby: Path = "";

        for e in paths {
            let path = e.unwrap();
            println!("Name: {:?}", path.path());
            if path.metadata().unwrap().is_dir() {
                //println!("It's a dir: {:?}", path.path());
                get_files(path.path().to_str().expect("Error with path conversion to string."));
            } else if path.metadata().unwrap().is_file() {
                scooby = path.path();
                println!("{:?}", files.push(scooby.to_str().expect("Error with path conversion to string.")));
            }
            return files;
        }
    }

    let file_vec = get_files(directory);

    

    //https://doc.rust-lang.org/std/fs/fn.read_dir.html
    //https://doc.rust-lang.org/nightly/std/fs/struct.Metadata.html

    // if file is a directory, then do nothing for now
    // if file is a file, then append it into a copy of index.html

}