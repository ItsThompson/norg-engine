use crate::agenda::{AgendaDay, AgendaItem, AgendaType};
use crate::category::{Category, Date};
use chrono::prelude::*;
use std::collections::HashMap;

// donna.rs
// generates vec<AgendaDay>

// Generate vector for next X(default 14) days from today
const AGENDA_DATES_IN_SCOPE: usize = 30;

pub fn generate_agenda(category_list: Vec<Category>) -> Vec<AgendaDay> {
    let all_agenda_items_list: Vec<AgendaItem> = generate_agenda_items(category_list.clone());

    let mut agenda_dates_arr: [Date; AGENDA_DATES_IN_SCOPE] = [Date::new(); AGENDA_DATES_IN_SCOPE];
    //let local: DateTime<Local> = Local::now();
    let local: DateTime<Local> = Local.with_ymd_and_hms(2023, 10, 2, 0, 0, 0).unwrap();
    let date: NaiveDate =
        NaiveDate::from_ymd_opt(local.year(), local.month(), local.day()).unwrap();
    let mut i = 0;
    for d in date.iter_days().take(AGENDA_DATES_IN_SCOPE) {
        agenda_dates_arr[i] = Date::new_date_without_time(
            u16::try_from(d.year()).unwrap(),
            u8::try_from(d.month()).unwrap(),
            u8::try_from(d.day()).unwrap(),
        );
        i += 1;
    }

    let agenda_date_map: HashMap<usize, Option<Vec<AgendaItem>>> =
        generated_agenda_hashmap(agenda_dates_arr, all_agenda_items_list);

    let agenda_data: Vec<AgendaDay> = generate_agenda_day(agenda_dates_arr, agenda_date_map);

    agenda_data
}

// Get list of AgendaItems
fn generate_agenda_items(category_list: Vec<Category>) -> Vec<AgendaItem> {
    let mut agenda_item_vec: Vec<AgendaItem> = Vec::new();
    for category in category_list.iter() {
        for schedule_item in category.schedule.iter() {
            agenda_item_vec.push(AgendaItem {
                category_id: category.id,
                agenda_type: AgendaType::ScheduleItem(schedule_item.to_owned()),
            });
        }
        for todo_item in category.todo.iter() {
            agenda_item_vec.push(AgendaItem {
                category_id: category.id,
                agenda_type: AgendaType::TodoItem(todo_item.to_owned()),
            });
        }
    }
    agenda_item_vec
}

// Categorize AgendaItems by Date
// Maps index of agenda_dates_arr to relevant AgendaItem for each date
fn generated_agenda_hashmap(
    agenda_dates_arr: [Date; AGENDA_DATES_IN_SCOPE],
    all_agenda_items_list: Vec<AgendaItem>,
) -> HashMap<usize, Option<Vec<AgendaItem>>> {
    let mut agenda_date_map: HashMap<usize, Option<Vec<AgendaItem>>> = HashMap::new();
    for i in 0..AGENDA_DATES_IN_SCOPE {
        let date = agenda_dates_arr[i];
        let mut agenda_item_vec: Vec<AgendaItem> = Vec::new();

        for agenda_item in all_agenda_items_list.iter() {
            let agenda_type: AgendaType = agenda_item.agenda_type.to_owned();
            let id = agenda_item.category_id;
            match agenda_type {
                AgendaType::ScheduleItem(schedule_item) => {
                    if schedule_item.date.is_equal_to(date) {
                        agenda_item_vec.push(AgendaItem {
                            category_id: id,
                            agenda_type: AgendaType::ScheduleItem(schedule_item),
                        })
                    }
                }
                AgendaType::TodoItem(todo_item) => {
                    if todo_item.date.is_equal_to(date) {
                        agenda_item_vec.push(AgendaItem {
                            category_id: id,
                            agenda_type: AgendaType::TodoItem(todo_item),
                        })
                    }
                }
            }
        }

        if agenda_item_vec.is_empty() {
            agenda_date_map.insert(i, None);
        } else {
            agenda_date_map.insert(i, Some(agenda_item_vec));
        }
    }
    agenda_date_map
}

fn generate_agenda_day(
    agenda_dates_arr: [Date; AGENDA_DATES_IN_SCOPE],
    agenda_date_map: HashMap<usize, Option<Vec<AgendaItem>>>,
) -> Vec<AgendaDay> {
    let mut agenda_data: Vec<AgendaDay> = Vec::new();

    for i in 0..AGENDA_DATES_IN_SCOPE {
        let date = agenda_dates_arr[i];
        let option = agenda_date_map.get(&i).unwrap();
        if option.is_none() {
            agenda_data.push(AgendaDay {
                date,
                agenda_items: None,
            });
        } else {
            let vec = option.to_owned().unwrap();
            let mut agenda_items: Vec<AgendaItem> = Vec::new();
            for agenda_item in vec {
                let id = agenda_item.category_id;
                let agenda_type: AgendaType = agenda_item.agenda_type.to_owned();
                match agenda_type {
                    AgendaType::ScheduleItem(schedule_item) => {
                        agenda_items.push(AgendaItem {
                            category_id: id,
                            agenda_type: AgendaType::ScheduleItem(schedule_item),
                        });
                    }
                    AgendaType::TodoItem(todo_item) => {
                        agenda_items.push(AgendaItem {
                            category_id: id,
                            agenda_type: AgendaType::TodoItem(todo_item),
                        });
                    }
                }
            }
            agenda_data.push(AgendaDay {
                date,
                agenda_items: Some(agenda_items),
            });
        }
    }
    agenda_data
}
