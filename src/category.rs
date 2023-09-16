use chrono::prelude::*;
use serde::Serialize;

// category.rs
//      structs and impls relating to category

#[derive(Clone, Serialize)]
pub struct Category {
    pub name: String,
    pub id: u32,
    pub properties: Vec<PropertiesItem>,
    pub schedule: Vec<ScheduleItem>,
    pub todo: Vec<TodoItem>,
}

#[derive(Clone, Serialize)]
pub struct PropertiesItem {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScheduleItem {
    pub name: String,
    pub date: Date,
    pub recurring: Recurrence,
}

#[derive(Debug, Copy, Clone, Serialize)]
pub struct Date {
    pub year: u16,
    pub month: u8,
    pub day: u8,
    pub start_time: u16,
    pub end_time: u16,
}

impl Date {
    pub fn new() -> Date {
        Date {
            year: 0,
            month: 0,
            day: 0,
            start_time: 0,
            end_time: 0,
        }
    }

    pub fn new_date_without_time(year: u16, month: u8, day: u8) -> Date {
        Date {
            year,
            month,
            day,
            start_time: 0,
            end_time: 0,
        }
    }

    pub fn is_equal_to(&self, date: Date) -> bool {
        let date_one = &self.to_owned();
        let date_two = date.to_owned();

        if date_one.year == date_two.year
            && date_one.month == date_two.month
            && date_one.day == date_two.day
        {
            true
        } else {
            false
        }
    }

    pub fn is_valid_date(&self) -> bool {
        let u16_zero: &u16 = &0;
        let u8_zero: &u8 = &0;
        if &self.year == u16_zero
            && &self.month == u8_zero
            && &self.day == u8_zero
            && &self.start_time == u16_zero
            && &self.end_time == u16_zero
        {
            false
        } else {
            true
        }
    }

    pub fn get_day_of_the_week(&self) -> String {
        let date = &self.to_owned();

        let year = i32::try_from(date.year).unwrap();
        let month = u32::try_from(date.month).unwrap();
        let day = u32::try_from(date.day).unwrap();
        let date = Local.with_ymd_and_hms(year, month, day, 0, 0, 0).unwrap();

        let weekday: String = date.weekday().to_string();

        weekday
    }

    pub fn to_string(&self) -> String {
        let data = &self.to_owned();

        if !data.is_valid_date() {
            return String::from("Invalid Date");
        }

        fn format_month_or_day(number: u8) -> String {
            let mut return_string: String = String::from("0");
            let num_string: &str = &number.to_string();
            if num_string.len() == 1 {
                return_string.push_str(num_string);
            } else {
                return_string = String::from(num_string);
            }
            return_string
        }

        fn format_time(number: u16) -> String {
            let mut date_string: String = String::new();
            let num_string: &str = &number.to_string();
            match num_string.len() {
                1 => {
                    date_string = String::from("00:0");
                    date_string.push_str(num_string);
                }
                2 => {
                    date_string = String::from("00:");
                    date_string.push_str(num_string);
                }
                3 => {
                    date_string = String::from("0");

                    date_string.push_str(num_string);

                    let mut return_string: String = (&date_string[0..2].to_string()).to_owned();
                    return_string.push_str(":");
                    return_string.push_str(&date_string[2..].to_string());

                    date_string = return_string;
                }
                4 => {
                    let mut return_string: String = (&num_string[0..2].to_string()).to_owned();
                    return_string.push_str(":");
                    return_string.push_str(&num_string[2..].to_string());

                    date_string = return_string;
                }
                _ => {}
            }

            date_string
        }

        let year = data.year.to_string();
        let month = format_month_or_day(data.month);
        let day = format_month_or_day(data.day);
        let start = format_time(data.start_time);
        let end = format_time(data.end_time);

        let mut date_string: String = String::new();
        date_string.push_str(&String::from("<"));
        date_string.push_str(&year);
        date_string.push_str(&String::from("-"));
        date_string.push_str(&month);
        date_string.push_str(&String::from("-"));
        date_string.push_str(&day);
        date_string.push_str(&String::from(" "));
        date_string.push_str(&start);
        date_string.push_str(&String::from("-"));
        date_string.push_str(&end);
        date_string.push_str(&String::from(">"));

        date_string
    }
}

#[derive(Debug, Copy, Clone, Serialize)]
pub enum Recurrence {
    None,
    Daily,
    Weekly,
    Monthly,
}

#[derive(Debug, Clone, Serialize)]
pub struct TodoItem {
    pub todo_id: u32,
    pub parent: TodoParent,
    pub title: String,
    pub date: Date,
}

#[derive(Debug, Copy, Clone, Serialize)]
pub enum TodoParent {
    None,
    TodoID(u32),
}
