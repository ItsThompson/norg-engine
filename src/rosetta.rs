use crate::category::{
    Category, Date, PropertiesItem, Recurrence, ScheduleItem, TodoItem, TodoParent,
};
use super::helper::read_file;
use yaml_rust::Yaml;

// rosetta.rs
//      "Translates" yml file and generates category struct

pub fn generate_category_struct(filename: &str) -> Category {
    let map = read_file(filename);
    // TODO: Expect Function
    assert!(map.is_ok());
    let data = map.unwrap();

    let mut s_name: String = String::new();
    let mut u_id: Option<u32> = None;
    let mut properties: Vec<Yaml> = Vec::new();
    let mut schedule: Vec<Yaml> = Vec::new();
    let mut todo: Vec<Yaml> = Vec::new();

    for (k, v) in data.iter() {
        let key = k.as_str().unwrap();
        match key {
            "name" => {
                let n = v.as_str().unwrap();
                s_name = String::from(n.to_owned());
            }
            "id" => {
                let id_i64 = v.as_i64().unwrap();
                u_id = Some(u32::try_from(id_i64).unwrap());
            }
            "Properties" => {
                let value = v.as_vec().unwrap();
                properties = value.to_owned();
            }
            "Schedule" => {
                let value = v.as_vec().unwrap();
                schedule = value.to_owned();
            }
            "To Do" => {
                let value = v.as_vec().unwrap();
                todo = value.to_owned();
            }
            _ => {
                println!("Extra Key");
            }
        }
    }

    // TODO: Expect Function
    assert!(s_name != "");

    // Generate Properties Struct
    // TODO: Expect Function
    assert!(properties.len() > 0);
    let properties_struct: Vec<PropertiesItem> = generate_properties_struct(properties);

    // Generate Schedule Struct
    // TODO: Expect Function
    assert!(schedule.len() > 0);
    let schedule_struct: Vec<ScheduleItem> = generate_schedule_struct(schedule);

    // Generate Todo Struct
    // TODO: Expect Function
    assert!(todo.len() > 0);
    let todo_struct: Vec<TodoItem> = generate_todo_struct(todo);

    let category = Category {
        name: s_name,
        id: u_id.unwrap(),
        properties: properties_struct,
        schedule: schedule_struct,
        todo: todo_struct,
    };

    category
}

pub fn generate_properties_struct(properties_vector: Vec<Yaml>) -> Vec<PropertiesItem> {
    let mut properties_vec: Vec<PropertiesItem> = Vec::new();

    for i in properties_vector.iter() {
        let items = i.as_hash().unwrap();
        for (k, v) in items.iter() {
            let key = k.as_str().unwrap();
            let value = v.as_str().unwrap();
            let key_string = String::from(key);
            let value_string = String::from(value);

            properties_vec.push(PropertiesItem {
                key: key_string,
                value: value_string,
            });
        }
    }

    properties_vec
}

pub fn generate_schedule_struct(schedule_vector: Vec<Yaml>) -> Vec<ScheduleItem> {
    let mut schedule_vec: Vec<ScheduleItem> = Vec::new();

    for i in schedule_vector.iter() {
        let items = i.as_hash().unwrap();
        let mut s_name: String = String::new();
        let mut o_date: Option<Date> = None;
        let mut o_recurring: Option<Recurrence> = None;

        for (k, v) in items.iter() {
            let key = k.as_str().unwrap();
            match key {
                "name" => {
                    let n = v.as_str().unwrap();
                    s_name = String::from(n.to_owned());
                }
                "date" => {
                    let d = v.as_str().unwrap();
                    o_date = Some(date_formatter(d.to_owned()));
                }

                "recurring" => {
                    let r = v.as_str().unwrap();
                    o_recurring = Some(recurrence_formatter(r.to_owned()));
                }
                _ => {
                    println!("Extra Key");
                }
            }
        }

        schedule_vec.push(ScheduleItem {
            name: s_name,
            date: o_date.unwrap(),
            recurring: o_recurring.unwrap(),
        });
    }

    schedule_vec
}

pub fn generate_todo_struct(todo_vector: Vec<Yaml>) -> Vec<TodoItem> {
    let mut todo_vec: Vec<TodoItem> = Vec::new();

    for i in todo_vector.iter() {
        let items = i.as_hash().unwrap();
        let mut i_todo_id: Option<u32> = None;
        let mut s_parent: Option<TodoParent> = None;
        let mut s_title: String = String::new();
        let mut o_date: Date = Date::new();

        for (k, v) in items.iter() {
            let key = k.as_str().unwrap();
            match key {
                "todo-id" => {
                    let n = v.as_i64().unwrap();
                    i_todo_id = Some(u32::try_from(n).unwrap());
                }
                "parent" => {
                    if v.as_str() == Some("none") {
                        s_parent = Some(TodoParent::None);
                    } else {
                        // TODO: Make sure is valid id before conversion
                        let p = v.as_i64().unwrap();
                        let x = u32::try_from(p).unwrap();
                        s_parent = Some(TodoParent::TodoID(x));
                    }
                }
                "title" => {
                    let n = v.as_str().unwrap();
                    s_title = String::from(n.to_owned());
                }
                "date" => {
                    let d = v.as_str().unwrap();
                    o_date = date_formatter(d.to_owned());
                }
                _ => {
                    println!("Extra Key");
                }
            }
        }

        // TODO: Expect Function
        assert!(s_title != "");

        todo_vec.push(TodoItem {
            todo_id: i_todo_id.unwrap(),
            parent: s_parent.unwrap(),
            title: s_title,
            date: o_date,
        });
    }

    todo_vec
}

pub fn date_formatter(date: String) -> Date {
    let date_str = date.as_str();

    let year: String = (&date_str[1..5].to_string()).to_owned();
    let month: String = (&date_str[6..8].to_string()).to_owned();
    let day: String = (&date_str[9..11].to_string()).to_owned();

    let mut start_hour: String = (&date_str[12..14].to_string()).to_owned();
    let start_min: &str = &date_str[15..17].to_string();
    start_hour.push_str(start_min);

    let mut end_hour: String = (&date_str[18..20].to_string()).to_owned();
    let end_min: &str = &date_str[21..23].to_string();
    end_hour.push_str(end_min);

    return Date {
        year: year.parse::<u16>().unwrap(),
        month: month.parse::<u8>().unwrap(),
        day: day.parse::<u8>().unwrap(),
        start_time: start_hour.parse::<u16>().unwrap(),
        end_time: end_hour.parse::<u16>().unwrap(),
    };
}

fn recurrence_formatter(recurrence: String) -> Recurrence {
    let mut r: Option<Recurrence> = None;
    match recurrence.as_str() {
        "none" => {
            r = Some(Recurrence::None);
        }
        "daily" => {
            r = Some(Recurrence::Daily);
        }
        "weekly" => {
            r = Some(Recurrence::Weekly);
        }
        "monthly" => {
            r = Some(Recurrence::Monthly);
        }
        _ => {
            println!("Invalid Recurrence")
        }
    }

    return r.unwrap();
}
