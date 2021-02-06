// feed

// check for feed.json folder in main theme folder (same folder as map.json)

// read feed.json into struct
//// feed.html template file location
//// number posts per feed page
//// 

// get name of each file in blog folder, and add to blog_posts array

// loop through blog_posts array until index 9 (10th item) (or whatever number of posts per page is specified in feed.json)
//// for each blog post, if .md, then convert to html
//// check file for "read more" delimeter. if one, append file contents until delimeter into feed.html file (or whatever file name is specified in feed.json) array. else, append full file into feed.html file
//// at same time, copy html, or convert md, file into feed folder
//// add link from blog feed to actual blog post into the feed page file
//// if there are additional items in the blog_posts array, then add next page button to bottom of feed.html



// if > 10 files in blog folder, then create a separate page, add 10 files, then create next page, and so on until all files are accounted for