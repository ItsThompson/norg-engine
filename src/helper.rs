use linked_hash_map::LinkedHashMap;
use std::{fs::{self, File}, io::Read};
use yaml_rust::{Yaml, YamlLoader};

const DIR:&str = "/Users/thompsontong/Documents/projects/2023/norg/sample-data";

pub fn read_file(filename: &str) -> Result<LinkedHashMap<Yaml, Yaml>, std::io::Error> {
    let mut file = File::open(filename).expect("Unable to open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read file");
    let load = YamlLoader::load_from_str(&contents).unwrap();
    let data = &load[0];

    let map = data.as_hash().unwrap();


    Ok(map.to_owned())
}

pub fn generate_category_list() -> Vec<String> {
    let mut file_list: Vec<String> = Vec::new();
    let paths =
        fs::read_dir(DIR).unwrap();

    for path in paths {
        let temp = path.unwrap().path().to_owned();
        let file_path_and_name = temp.to_str().unwrap();
        //let re = Regex::new(r"([^/]+$)").unwrap();
        //let mut file_name: Option<&str> = None;

        //match re.captures(file_path_and_name) {
        //    Some(caps) => file_name = Some(caps.get(0).unwrap().as_str()),
        //    None => {
        //        println!("error");
        //    }
        //}
        let s_file_name = String::from(file_path_and_name);
        if s_file_name.len() > 4 {
            if s_file_name[s_file_name.len() - 4..].to_string() == String::from(".yml") {
                file_list.push(s_file_name);
            } else if s_file_name[s_file_name.len() - 5..].to_string() == String::from(".yaml") {
                file_list.push(s_file_name);
            }
        }
    }

    file_list
}

//pub fn sort_category(category_data: Vec<Category>) -> Vec<Category> {
//    let mut data: Vec<Category> = category_data;
//
//    //bubble sort
//    let mut i = 0;
//    let mut sorted: bool = true;
//    let max = data.len() - 1;
//    loop {
//        //println!("{i}");
//        if i < max {
//            let first = usize::try_from(data[i].id).unwrap();
//            let second = usize::try_from(data[i + 1].id).unwrap();
//
//            // print!("first {}, ", first);
//            // println!("second {}", second);
//
//            if first > second {
//                data.swap(i, i + 1);
//                sorted = false;
//            }
//
//            i += 1;
//        } else {
//            // for d in data.iter() {
//            //     print!("{} ", d.id);
//            // }
//            // println!("");
//            if sorted == true {
//                break;
//            }
//            i = 0;
//            sorted = true;
//        }
//    }
//    data
//}
