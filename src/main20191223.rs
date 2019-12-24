use std::fs; // for reading and writing files
use serde_json::{Result, Value}; // for reading JSON files
use serde::{Deserialize, Serialize};

    // create structures of types to hold JSON data
    #[derive(Serialize, Deserialize, Debug)]
    struct Page {
        pages: Vec<PageDetails>,
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct PageDetails {
        title: String,
        link: bool,
        path: String,
        template: bool,
        index: i32,
    }

fn main() {

    // load JSON file to string
    let json_str = fs::read_to_string("C:\\Users\\joekr\\Programs\\soswg_site\\map.json")
        .expect("Error loading file.");

    // load JSON string to Pages struct
    let pages_obj = serde_json::from_str::<Page>(&json_str)
        .expect("Error structuring JSON to Page.");

        // println!("{:?}", pages_obj.pages);
        // this gets the Page struct then pages vec, then index 0 of pages vec, then the title of the struct in index 0
        let mut page_num: u32;
        // find template HTML file
        for each in 0..pages_obj.pages.len() {
            let page_details = &pages_obj.pages[each];
            let template = page_details.template;
            if template == true {
                // FIX: move variable declarations out of code, to beginning of for loop
                let template_path = &pages_obj.pages[each].path;
                //println!("{:?}", template_path);

                // load template HTML file to string
                let template_str = fs::read_to_string(template_path)
                    .expect("Error loading file."); // FIX: determine template file name from JSON
                
                return template_str;
            } else {
                return None;
            }
        };
        println!("{:?}", template_str);
        /* THIS NEEDS TO BE ANOTHER FOR LOOP
        let page_details = &pages_obj.pages[each];
        let page_path = &page_details.path;

        // create HTML file
        match fs::write(page_path, template_str) {
            // FIX: make the file create in specified project folder/where SOSWG is run
            // FIX: determine HTML file name from JSON
            Ok(o) => o.to_string(),
            Err(e) => e.to_string()
        }
        */

    // NEXT: parse template file (string) for variable
    // then insert the contents HTML into newly created HTML files for each page
}