// 
use sysinfo::ProcessExt;
use procfs::process;            
#[derive(PartialEq, Clone)]


// process info needed
pub struct Process {
    pub pid: i32,
    pub name: String,
    pub cpu: f32,
    pub mem: u64,
    pub nice: i64,
    pub state: char,
    pub ppid:i32,
}

impl Process {
    pub fn new(process: &sysinfo::Process) -> Process {
        let all_procs = process::all_processes().unwrap();

        // initialize nice, state, ppid with initial values. 
        let mut nice: i64 = -1;
        let mut state: char = 'a';
        let mut ppid: i32 = 0;
        
        // each proc return from sysinfo::process
        for p in all_procs{
            let proc = p.unwrap().stat().unwrap();
            if process.pid() == proc.pid {
                nice = proc.nice;
                state = proc.state;
                ppid = proc.ppid;
            }
        }

        
        Process {
            pid     : process.pid(),
            name    : process.name().to_string(),
            cpu     : process.cpu_usage(),
            mem     : process.memory(),
            nice    : nice,
            state   : state,
            ppid    : ppid
        }
    }

     
    pub fn format(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            self.nice.to_string(),
            self.state.to_string(),
            self.ppid.to_string()
        ]
    }
}
    
    
 