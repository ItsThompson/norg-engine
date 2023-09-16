//use std::collections::HashMap;
//
//use category::Category;
//use donna::generate_agenda;
//use helper::generate_category_list;
//use rosetta::generate_category_struct;

pub mod agenda;
pub mod category;
pub mod donna;
pub mod helper;
pub mod rosetta;

//fn main() {
//    let file_list: Vec<String> = generate_category_list();
//    let mut category_vec: Vec<Category> = Vec::new();
//    let mut id_map: HashMap<u32, usize> = HashMap::new();
//
//    for file in file_list {
//        category_vec.push(generate_category_struct(file.as_str()));
//    }
//
//    for i in 0..category_vec.len() {
//        id_map.insert(category_vec[i].id, i);
//    }
//    // category_vec[id_map.get(&(u32::try_from(6).unwrap())).unwrap().to_owned()].clone(); 
//    // would get category with id=6 which is in index 5 of the vector.
//
//    generate_agenda(category_vec);
//}
