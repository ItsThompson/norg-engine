use crate::category::{Date, ScheduleItem, TodoItem};
use serde::Serialize;

// agenda.rs
//     structs and impls relating to agenda

#[derive(Clone, Serialize)]
pub struct AgendaDay {
    pub date: Date,
    pub agenda_items: Option<Vec<AgendaItem>>,
}

#[derive(Debug, Clone, Serialize)]
pub struct AgendaItem {
    pub category_id: u32,
    pub agenda_type: AgendaType,
}

#[derive(Debug, Clone, Serialize)]
pub enum AgendaType {
    ScheduleItem(ScheduleItem),
    TodoItem(TodoItem),
}
