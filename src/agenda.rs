use crate::category::{Date, ScheduleItem, TodoItem};

// agenda.rs
//     structs and impls relating to agenda

#[derive(Clone)]
pub struct AgendaDay {
    pub date: Date,
    pub agenda_items: Option<Vec<AgendaItem>>,
}

#[derive(Debug, Clone)]
pub struct AgendaItem {
    pub category_id: u32,
    pub agenda_type: AgendaType,
}

#[derive(Debug, Clone)]
pub enum AgendaType {
    ScheduleItem(ScheduleItem),
    TodoItem(TodoItem),
}
