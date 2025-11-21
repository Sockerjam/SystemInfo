pub mod models;
use std::{collections::VecDeque, sync::mpsc::Receiver};

use ratatui::widgets::ScrollbarState;
use sysinfo::{Components, Cpu, Disks, RefreshKind, System};
use crate::app::models::{CPU, AppEvents, Memory, OSInfo};
const BYTES_TO_GB: f32 = 1024.0 * 1024.0 * 1024.0;


trait SystemInfoProvider {
    fn refresh_all(&mut self);
    fn update(&mut self);
    fn get_cpus(&self) -> Vec<CPU>;
    fn get_memory(&self) -> Memory;
    fn get_system_info(&self) -> OSInfo;
    fn update_used_memory(&self) -> f32;
    fn update_used_swap(&self) -> f32;
    fn update_cpu_usage(&self, cpu_name: &String) -> u64;
}

pub struct SysInfoAdapter {
    system: System
}

impl SysInfoAdapter {
    fn new() -> Self {
        Self { system: System::new_all() }
    }
}

impl SystemInfoProvider for SysInfoAdapter {
    fn update(&mut self) {
        self.system.refresh_cpu_usage();
        self.system.refresh_memory();
    }
    
    fn refresh_all(&mut self) {
        self.system.refresh_all();
    }
    
    fn get_cpus(&self) -> Vec<CPU> {
        self.system.cpus().iter().map(|cpu| 
            CPU { 
                core: cpu.name().to_string(), 
                usage_history: VecDeque::from([cpu.cpu_usage() as u64]) }
            )
            .collect()
    }

    fn get_memory(&self) -> Memory {
        Memory { 
            total_memory: self.system.total_memory() as f32 / BYTES_TO_GB, 
            used_memory: self.system.used_memory() as f32 / BYTES_TO_GB, 
            total_swap: self.system.total_swap() as f32 / BYTES_TO_GB, 
            used_swap: self.system.used_swap() as f32 / BYTES_TO_GB}
    }
    
    fn get_system_info(&self) -> OSInfo {
        OSInfo { 
            cpu_arch: System::cpu_arch(), 
            os_version: System::os_version().unwrap_or_else(|| return "N/A".to_string() ), 
            system_name: System::name().unwrap_or_else(|| return "N/A".to_string() )}
    }
    
    fn update_used_memory(&self) -> f32 {
        self.system.used_memory() as f32 / BYTES_TO_GB
    }
    
    fn update_used_swap(&self) -> f32 {
        self.system.used_swap() as f32 / BYTES_TO_GB
    }

    fn update_cpu_usage(&self, cpu_name: &String) -> u64 {
        self.system.cpus()
            .iter()
            .find(|cpu| cpu.name() == cpu_name)
            .map(|cpu| cpu.cpu_usage() as u64)
            .unwrap_or(0)
    }
}
pub struct App {
    memory: Memory,
    cpus: Vec<CPU>,
    os_info: OSInfo,
    system_provider: Box<dyn SystemInfoProvider + Send>,
    rx: Receiver<AppEvents>,
    pub cpu_scroll_state: ScrollbarState,
    pub cpu_scroll_position: usize
}

impl App {
    pub fn new(rx: Receiver<AppEvents>) -> Self {
        let mut system_provider = Box::new(SysInfoAdapter::new());
        system_provider.refresh_all();
        let cpus = system_provider.get_cpus();
        let cpus_content_height = cpus.len() * 5;
        Self {
           memory: system_provider.get_memory(),
           cpus: cpus,
           os_info: system_provider.get_system_info(),
           rx: rx,
           cpu_scroll_state: ScrollbarState::new(cpus_content_height),
           cpu_scroll_position: 0,
           system_provider: system_provider,
        }
    } 

    pub fn update(&mut self) {
        self.system_provider.update();
        self.update_memory();
        self.update_cpu_usage();
    }
    
    fn update_memory(&mut self) {
        self.memory.used_memory = self.system_provider.update_used_memory();
        self.memory.used_swap = self.system_provider.update_used_swap();
    }
    
    fn update_cpu_usage(&mut self) {
        for cpu in &mut self.cpus {
            if cpu.usage_history.len() >= 75 { cpu.usage_history.pop_front(); }
            cpu.usage_history.push_back(self.system_provider.update_cpu_usage(&cpu.core));
        }
    }

    pub fn get_memory(&self) -> &Memory {
        &self.memory
    }
    
    pub fn get_cpus(&self) -> &Vec<CPU> {
        &self.cpus
    }

    pub fn get_os_info(&self) -> &OSInfo {
        &self.os_info
    }
    
    pub fn handle_rx(&mut self) -> bool {
        // Drain all queued events to avoid input lag
        while let Ok(received_value) = self.rx.try_recv() {
            match received_value {
                AppEvents::DOWN => {
                    self.cpu_scroll_position = self.cpu_scroll_position.saturating_add(5);
                    self.cpu_scroll_state = self.cpu_scroll_state.position(self.cpu_scroll_position);
                },
                AppEvents::UP => {
                    self.cpu_scroll_position = self.cpu_scroll_position.saturating_sub(5);
                    self.cpu_scroll_state = self.cpu_scroll_state.position(self.cpu_scroll_position);
                },
                AppEvents::UPDATE => {
                    self.update();
                }
                AppEvents::QUIT => {
                    return true;
                }
            }
        }
        false
    }
}