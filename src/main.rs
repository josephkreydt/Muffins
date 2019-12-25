/*************
 * libraries *
 *************/
use std::fs; // for reading and writing files
use serde_json::{Result, Value}; // for handling JSON files
use serde::{Deserialize, Serialize}; // for handling JSON files

/************************************
 * create structs to hold JSON data *
 ************************************/
#[derive(Serialize, Deserialize, Debug)]
struct Page {
    pages: Vec<PageDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PageDetails {
    title: String,
    link: bool,
    path: String,
    content: String,
    template: bool,
    index: i32,
}

/********************
 * the main program *
 ********************/
fn main() {

    // load JSON file to string
    let json_str = fs::read_to_string("C:\\Users\\joekr\\Programs\\soswg_site\\map.json")
        .expect("Error loading file.");

    // load JSON string to Pages struct
    let pages_obj = serde_json::from_str::<Page>(&json_str)
        .expect("Error structuring JSON to Page.");

        // println!("{:?}", pages_obj.pages);
        // this gets the Page struct then pages vec, then index 0 of pages vec, then the title of the struct in index 0
        let mut template_str: String = "".to_string();//FIX: Need to init this with something different
        let mut page_paths = Vec::new();

        // find template HTML file
        for each in 0..pages_obj.pages.len() {
            let page_details = &pages_obj.pages[each];
            let template = page_details.template;
            if template == true {
                
                // FIX: move variable declarations out of code, to beginning of for loop
                let template_path = &pages_obj.pages[each].path;
                //println!("{:?}", template_path);

                // load template HTML file to string
                template_str = fs::read_to_string(template_path)
                    .expect("Error loading file.");
                break
            }
        };
        //println!("{:?}", template_str);

        for each in 0..pages_obj.pages.len() {
            let page_details = &pages_obj.pages[each];
            let page_path = &page_details.path;
            let template = page_details.template;

            if template == false {
                page_paths.push(page_path.to_string());
            }
        }

        //println!("{:?}", page_paths);

        for each in 0..page_paths.len() {
            let page_details = &pages_obj.pages[each];
            let page_path = &page_details.path;
            let link = page_details.link;

            if link == false {
                // create HTML file
                match fs::write(page_path, &template_str) {
                    // FIX: make the file create in specified project folder/where SOSWG is run
                    // FIX: determine HTML file name from JSON
                    Ok(o) => o,
                    Err(_e) => ()
                }
            }
        }


    // NEXT: parse template file (string) for variable
    // then insert the contents HTML into newly created HTML files for each page
    //let template_parts: Vec<&str> = template_str.split("{INSERT_HTML}").collect();

    for each in 0..pages_obj.pages.len() {
        let page_details = &pages_obj.pages[each];
        let page_path = &page_details.path;
        let content_path = &page_details.content;

        if content_path != "" {
            println!("{:?}", content_path);

            let content_str = fs::read_to_string(&content_path)
            .expect("Error loading file.");

            let page_str = template_str.replace("{INSERT_HTML}", &content_str);

            match fs::write(page_path, page_str) {
                Ok(o) => o,
                Err(_e) => ()
            }
        }
        // append template part or content to file
    }

    // THEN: need to parse template file (string) for menu variable
    // then loop through the map.json file and create a menu link for any pages with
    // an index of > 0
}