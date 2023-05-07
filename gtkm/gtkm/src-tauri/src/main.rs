// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use sysinfo::{ProcessExt, SystemExt, ComponentExt};
use std::thread;
use std::time::Duration;


// converts the struct to a jason format. 
use serde::{Serialize};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
 mod system;
 mod process;

 // importing two rust modules defined in several files. 
 use crate::system::System;
 use crate::process::Process;

 
#[derive(Serialize)] // is applied on stringproc to serialize into various formats "JSON"
struct StringProc{
    pub pid: String,
    
    pub name: String,
    pub cpu: String,
    pub mem: String,
    pub nice: String,
    pub state: String,
    pub ppid:String
}

impl StringProc{
    pub fn new(pid: String, name: String, cpu: String, mem: String, nice: String, state: String,ppid:String) -> StringProc{
        StringProc{
             pid: pid,
             name: name,
             cpu: cpu,
             mem: mem,
             nice: nice,
             state: state,
             ppid:ppid
        }
    }
}



#[derive(Serialize)] // is applied on stringproc to serialize into various formats "JSON"
struct StringSys{
    pub cpu_usage_history: Vec<u64>,
    pub cpu_current_usage: u64,
    pub cpu_num_cores: usize,
    pub mem_total: u64,
    pub mem_free: u64,
    pub mem_used: u64,
    pub mem_usage_history: Vec<u64>,
    pub cpu_core_usages: Vec<u16>,
    pub temp: Vec<(String, f32)>,
}

impl StringSys{
    pub fn new( cpu_usage_history: Vec<u64>, 
                cpu_current_usage: u64, 
                cpu_num_cores: usize, 
                mem_total: u64, 
                mem_free: u64, 
                mem_used: u64,
                mem_usage_history:Vec<u64>, 
                cpu_core_usages:Vec<u16>,
                temp: Vec<(String, f32)>
                ) -> StringSys{

        StringSys{
         cpu_usage_history: cpu_usage_history,
         cpu_current_usage: cpu_current_usage,
         cpu_num_cores: cpu_num_cores,
         mem_total: mem_total,
         mem_free: mem_free,
         mem_used: mem_used,
         mem_usage_history: mem_usage_history,
         cpu_core_usages: cpu_core_usages,
         temp: temp,
        }
    }
}


// #[tauri::command]
// fn system() {
//     let mut system = System::new();
//     let system_update = system.update();
//     let mut systemStruct: StringSys = StringSys::new(system_update.cpu_usage_history, system_update.cpu_current_usage, system_update.cpu_num_cores, system_update.cpu_current_usage, system_update.cpu_current_usage, system_update.cpu_current_usage, system_update.cpu_current_usage, system_update.cpu_current_usage, )
// }


#[tauri::command]
fn pTable()-> String {
    let mut system = System::new();
    let system_update = system.update();
    let mut processes: Vec<Process> = system_update.processes.clone();
    //let mut headers: [&str; 5] = ["PID", "Name", "CPU", "Memory", "Nice"];
    let mut fmt_processes: Vec<Vec<String>> = processes.iter().map(|process| process.format()).collect();
    let mut ProcStrings: Vec<StringProc> = Vec::new();
    println!("{}", fmt_processes[0][0]);
    


    for row in fmt_processes{
        let proc: StringProc = StringProc::new(row[0].clone(), row[1].clone(), row[2].clone(), row[3].clone(), row[4].clone(), row[5].clone(), row[6].clone());
        ProcStrings.push(proc);
    }

    let json = serde_json::to_string(&ProcStrings);
    // unwrap gets the result from the tuple. 
    json.unwrap()

}

#[tauri::command]
fn sysTable()-> String {
    let mut system = System::new();
    let mut s = sysinfo::System::new();
    let system_update = system.update();
    let mut temp: Vec<(String, f32)>  = Vec::new();
    let mut label;
    let mut tempp;
    for component in s.get_components_list() {
       label = component.get_label();
       tempp= component.get_temperature();
        temp.push((label.to_string().clone(),tempp));
    }

    // let mut fmt_ststem: Vec<String> = system.map(|system| system.format()).collect();
    // let proc: StringProc = StringProc::new(row[0].clone(), row[1].clone(), row[2].clone(), row[3].clone(), row[4].clone(), row[5].clone(), row[6].clone());
    let sys: StringSys = StringSys::new(
                                        system_update.cpu_usage_history, 
                                        system_update.cpu_current_usage, 
                                        system_update.cpu_num_cores, 
                                        system_update.mem_total, 
                                        system_update.mem_free, 
                                        system_update.mem_used, 
                                        system_update.mem_usage_history,
                                        system_update.cpu_core_usages,
                                        temp
                                        
                                    );

                                        // system_update.processes);
    
    let json = serde_json::to_string(&sys);
    println!("{}", serde_json::to_string_pretty(&sys).unwrap());
    // println!("json");                                    
    
    json.unwrap()

// old code for taking all values of system.
    // let mut cpu_usage_history = system_update.cpu_usage_history.clone();
    // let mut cpu_current_usage = system_update.cpu_current_usage;
    // let mut cpu_num_cores     = system_update.cpu_num_cores;
    // let mut mem_total         = system_update.mem_total;
    // let mut mem_free          = system_update.mem_free;
    // let mut mem_used          = system_update.mem_used;
    // let mut mem_usage_history = system_update.mem_usage_history.clone();
    // let mut cpu_core_usages   = system_update.cpu_core_usages.clone();
    // // let mut processes         = system_update.processes.clone();


    // let mut fmt_cpu_usage_history: Vec<Vec<String>> = cpu_usage_history.iter().map(|system| system.format()).collect();
    // let mut fmt_mem_usage_history: Vec<Vec<String>> = mem_usage_history.iter().map(|system| system.format()).collect();
    // let mut fmt_cpu_core_usages: Vec<Vec<String>> = cpu_core_usages.iter().map(|system| system.format()).collect();

    // let mut SystemStrings: Vec<StringProc> = Vec::new();
    // // let mut ProcStrings: Vec<StringProc> = Vec::new();
    // // let mut ProcStrings: Vec<StringProc> = Vec::new();


// old process code 
    // let mut processes: Vec<Process> = system_update.processes.clone();
    // // let mut headers: [&str; 5] = ["PID", "Name", "CPU", "Memory", "Nice"];
    // let mut fmt_processes: Vec<Vec<String>> = processes.iter().map(|process| process.format()).collect();
    // let mut ProcStrings: Vec<StringProc> = Vec::new();
    // println!("{}", fmt_processes[0][0]);
    


    // for row in fmt_processes{
    //     let proc: StringProc = StringProc::new(row[0].clone(), row[1].clone(), row[2].clone(), row[3].clone(), row[4].clone(), row[5].clone(), row[6].clone());
    //     ProcStrings.push(proc);
    // }

    // let json = serde_json::to_string(&ProcStrings);
    // // unwrap gets the result from the tuple. 
    // json.unwrap()
}




fn main() {
    loop{
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![ pTable,sysTable])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

        thread::sleep(Duration::from_secs(1));
    }
}
