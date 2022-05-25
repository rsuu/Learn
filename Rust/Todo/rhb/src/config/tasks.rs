use crate::config::base::Base;

pub enum Type {
    Habits,
    Dailies,
    Todo,
}
pub struct Habits {
    pub base: Base,
    pub mode: DisplayType,
}

pub struct Dailies {
    pub base: Base,
    pub mode: DisplayType,
}

pub struct Todo {
    pub base: Base,
    pub mode: DisplayType,
}

pub enum DisplayType {
    Lv1,
    Lv2,
    Lv3,
    Lv4,
    Lv5,
    Due,
    Notdue,
}
