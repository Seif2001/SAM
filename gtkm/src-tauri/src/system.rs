//  calling sysinfo and procfs. 
use sysinfo::{SystemExt, ProcessorExt, ProcessExt, ComponentExt};
use procfs::process;
use crate::process::Process;
//use syscalls::x86::Sysno::setpriority;



#[link(name = "systemcall", kind = "static")]
// chPrio sets the priority of a process. 
// extern to be used out of the file. (globally).
// defined as an external C library with the name 'systemcall. 
// and takes 2 parameters (pid, priority_value).
extern "C"{
    fn chPrio(pid:i32, s:i64) -> i32;
}


pub struct System {
    pub sysinfo             : sysinfo::System, // object >> info about system hardware
    pub cpu_usage_history   : Vec<u64>, // vector >> cpu usage history
    pub cpu_current_usage   : u64, // current cpu usage
    pub cpu_num_cores       : usize, // number of cores. 
    pub mem_total           : u64, // total amount of memory on the system
    pub mem_free            : u64, // the amount of free memory on the system
    pub mem_used            : u64, // the amount of used memory on the system
    pub mem_usage_history   : Vec<u64>, // vector >> mem usage history
    pub cpu_core_usages     : Vec<u16>, // vector >> cpu usage for each core
    pub processes           : Vec<Process>, // vector of process objects >> represents the running processes
}


impl System {
    // starts a system object with some initialized data
    pub fn new() -> System {
        
        // new sysinfo::System object and initializes
        // the CPU and memory usage history vectors with 25 elements each.
        let sysinfo = sysinfo::System::new();

        
        // Overall CPU usage
        let cpu_usage_history = vec![0; 25]; // ?
        let cpu_num_cores: usize = sysinfo.get_processor_list().len() - 1;

        // Memory usage
        let mem_total = sysinfo.get_total_memory();
        let mem_usage_history = vec![0; 25 as usize];

        System {
            sysinfo,
            cpu_usage_history,
            cpu_current_usage: 0,
            cpu_num_cores,
            mem_total,
            mem_free: 0,
            mem_used: 0,
            mem_usage_history,
            cpu_core_usages: vec![],
            processes: vec![]
        }
    }


    // updates the initialized values of the new system object. 
    // 
    pub fn update(&mut self) -> System {
        self.sysinfo.refresh_all();

        // Overall CPU usage
        self.cpu_current_usage = (self.sysinfo.get_processor_list()[0].get_cpu_usage() * 100.0).round() as u64;
        self.cpu_usage_history.push(self.cpu_current_usage);
        self.cpu_usage_history.remove(0);

        // Memory usage
        self.mem_used = self.sysinfo.get_used_memory();
        self.mem_free = self.sysinfo.get_free_memory();
        self.mem_usage_history.push(self.mem_used);
        self.mem_usage_history.remove(0);

        // CPU core usage
        self.cpu_core_usages = self.sysinfo.get_processor_list()
            .iter()
            .skip(1)
            .map(|p| (p.get_cpu_usage() * 100.0).round() as u16)
            .collect();

        // Processes
        self.processes = self.sysinfo.get_process_list()
            .iter()
            .map(|(_, process)|
                Process::new(process)
            )
            .collect();

        System {
            sysinfo: sysinfo::System::new(),
            cpu_usage_history: self.cpu_usage_history.clone(),
            mem_usage_history: self.mem_usage_history.clone(),
            cpu_core_usages: self.cpu_core_usages.clone(),
            processes: self.processes.clone(),
            ..*self
        }
    }

    pub fn kill_process(&mut self, pid: i32) {
        if let Some(process) = self.sysinfo.get_process(pid) {
            process.kill(sysinfo::Signal::Kill);
        }
    }

    pub fn increase_priority(&mut self, pid:i32){
        let all_procs = process::all_processes().unwrap();
        for p in all_procs{
            let mut process = p.unwrap().stat().unwrap();
            if process.pid == pid{
                let res = {unsafe {chPrio(pid, process.nice - 1)}};
            }
        }
    }

    pub fn decrease_priority(&mut self, pid:i32){
        let all_procs = process::all_processes().unwrap();
        for p in all_procs{
            let mut process = p.unwrap().stat().unwrap();
            if process.pid == pid{
                let res = {unsafe {chPrio(pid, process.nice + 1)}};
            }
        }
    }
}
