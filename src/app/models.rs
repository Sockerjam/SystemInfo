use std::collections::VecDeque;

pub struct CPU {
    pub core: String,
    pub usage_history: VecDeque<u64>
}

pub struct Memory {
    pub total_memory: f32,
    pub used_memory: f32,
    pub total_swap: f32,
    pub used_swap: f32
}

pub struct OSInfo {
    pub cpu_arch: String,
    pub os_version: String,
    pub system_name: String
}

pub enum AppEvents {
    UP,
    DOWN,
    QUIT,
    UPDATE
}