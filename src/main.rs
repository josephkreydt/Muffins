/*************
 * libraries *
 *************/
use std::fs; // for reading and writing files
//use serde_json::{Result, Value}; // for handling JSON files
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

    /********************
    * function: load map.json *
    ********************/
    // load JSON file to string
    let json_str = fs::read_to_string("./map.json")
        .expect("Error loading file.");

    // load JSON string to Pages struct
    let pages_obj = serde_json::from_str::<Page>(&json_str)
        .expect("Error structuring JSON to Page.");

    /********************
    * function: load template file *
    ********************/
    // this gets the Page struct then pages vec, then index 0 of pages vec, then the title of the struct in index 0
    let mut template_str: String = "".to_string();

    // find template HTML file
    for each in 0..pages_obj.pages.len() {
        let page_details = &pages_obj.pages[each];
        let template = page_details.template;
        if template == true {
            
            // FIX: move variable declarations out of code, to beginning of for loop
            let template_path = &pages_obj.pages[each].content;

            // load template HTML file to string
            template_str = fs::read_to_string(template_path)
                .expect("Error loading file.");
            break
        }
    };
    println!("{:?}", template_str);

    /********************
    * function: generate menu items HTML *
    ********************/
    let mut menu_items = Vec::new();

    let mut menu_items_count = 0;
    for each in 0..pages_obj.pages.len() {
        let page_details = &pages_obj.pages[each];
        let menu_index = &page_details.index;

        if menu_index > &0 {
            menu_items_count = menu_items_count + 1;
        }
    }
    let mut menu_link_str: String = "".to_string();
    let mut i: i32 = 1; //starts w/1 instead of 0 because menu_index 0 marks do not add to menu
    loop {

        if menu_items.len() == menu_items_count {
            break;
        } else {
            for each in 0..pages_obj.pages.len() {
                let page_details = &pages_obj.pages[each];
                let menu_index = &page_details.index;
                let page_path = &page_details.path;
                let page_title = &page_details.title;
                let page_link_bool = &page_details.link;
                if menu_index == &i && page_link_bool == &false {
                    menu_items.push(page_title);
                    menu_link_str = menu_link_str + "<a href=" + r#"""# + page_path + r#"""# + r#" class="menu-link"> "# + page_title + " </a>\n";
                } else if menu_index == &i && page_link_bool == &true {
                    menu_items.push(page_title);
                    menu_link_str = menu_link_str + "<a href=" + r#"""# + page_path + r#"""# + r#" class="menu-link" target="_blank">"# + page_title + " </a>\n";
                }
            }
        }

        i = i + 1;
    }


    /********************
    * function: create HTML string that contain the template and the content *
    ********************/
    let mut pages_vec = Vec::new();

    for each in 0..pages_obj.pages.len() {
        let page_details = &pages_obj.pages[each];
        let content_path = &page_details.content;
        let template = page_details.template;

        if content_path != "" && template != true {
            let content_str = fs::read_to_string(&content_path)
                .expect("Error loading file.");

            let page_str = template_str.replace("{INSERT_HTML}", &content_str);

            pages_vec.push(page_str.to_string());
        } else {
            let page_str = "";
            pages_vec.push(page_str.to_string());
        }
    }

    /********************
    * function: write the menu items HTML to the page_str *
    ********************/
    let mut final_pages_vec = Vec::new();
    for each in 0..pages_vec.len() {
        let final_page_str = pages_vec[each].replace("{MENU}", &menu_link_str);
        final_pages_vec.push(final_page_str);
    }

    /********************
    * function: write new pages to file *
    ********************/
    for each in 0..pages_obj.pages.len() {
        let page_details = &pages_obj.pages[each];
        let content_path = &page_details.content;
        let template = page_details.template;
        let page_path = &page_details.path;
        let write_path = format!("./site/{}", page_path);
        let page_content = &final_pages_vec[each];

        if content_path != "" && template != true {
            match fs::write(write_path, page_content) {
                Ok(o) => o,
                Err(_e) => ()
            }
        }
    }
}