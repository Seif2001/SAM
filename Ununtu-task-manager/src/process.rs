use sysinfo::ProcessExt;
use procfs::process;
#[derive(PartialEq, Clone)]
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
        let mut nice:i64 = -1;
        let mut state:char = 'a';
        let mut ppid: i32 = 0;
        

        for p in all_procs{
            let proc = p.unwrap().stat().unwrap();
            if process.pid() == proc.pid {
                nice = proc.nice;
                state = proc.state;
                ppid = proc.ppid;
            }
        }

        Process {
            pid: process.pid(),
            name: process.name().to_string(),
            cpu: process.cpu_usage(),
            mem: process.memory(),
            nice: nice,
            state:state,
            ppid:ppid
        }
    }

    pub fn format0(&self) -> Vec<String> {
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
    
    
    pub fn format1(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            
            self.state.to_string(),
            
        ]
    }
    pub fn format2(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            
            self.ppid.to_string()
        ]
    }
    pub fn format3(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            format!("{:.2}%", self.cpu),
            self.nice.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
        ]
    }
    pub fn format4(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            format!("{:.2}%", self.cpu),
            self.nice.to_string(),
            self.state.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

        ]
    }
    pub fn format5(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            format!("{:.2}%", self.cpu),
            self.nice.to_string(),
            self.ppid.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

        ]
    }
    pub fn format6(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            format!("{:.2}%", self.cpu),
            self.state.to_string(),

            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
        ]
    }
    pub fn format7(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            format!("{:.2}%", self.cpu),
            self.state.to_string(),

            self.nice.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

        ]
    }
    pub fn format8(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            format!("{:.2}%", self.cpu),
            self.state.to_string(),

            self.ppid.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

        ]
    }
    pub fn format9(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            format!("{:.2}%", self.cpu),
            self.ppid.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

        ]
    }
    pub fn format10(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            format!("{:.2}%", self.cpu),
            self.ppid.to_string(),
            self.nice.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),


        ]
    }
    pub fn format11(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            format!("{:.2}%", self.cpu),
            self.ppid.to_string(),
            
            self.state.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

            
        ]
    }
    pub fn format12(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            format!("{:.2}%", self.cpu),
            self.nice.to_string(),
        ]
    }
    pub fn format13(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            format!("{:.2}%", self.cpu),
            self.state.to_string(),
        ]
    }
    pub fn format14(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            format!("{:.2}%", self.cpu),
            self.ppid.to_string(),

        ]
    }
    pub fn format15(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            self.nice.to_string(),
            format!("{:.2}%", self.cpu),
            
        ]
    }
    pub fn format16(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            self.nice.to_string(),
            
            self.state.to_string(),
            format!("{:.2}%", self.cpu),

        ]
    }
    pub fn format17(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            self.nice.to_string(),
            
            self.ppid.to_string(),
            format!("{:.2}%", self.cpu),

        ]
    }
    pub fn format18(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            self.state.to_string(),
            format!("{:.2}%", self.cpu),
            
        ]
    }
    pub fn format19(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            self.state.to_string(),
            self.nice.to_string(),
            format!("{:.2}%", self.cpu),

            
        ]
    }
    pub fn format20(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            self.state.to_string(),
            self.ppid.to_string(),
            format!("{:.2}%", self.cpu),

        ]
    }
    pub fn format21(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            self.ppid.to_string(),
            format!("{:.2}%", self.cpu),
           
        ]
    }
    pub fn format22(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            self.ppid.to_string(),
            
            self.nice.to_string(),
            format!("{:.2}%", self.cpu),

            
        ]
    }
    pub fn format23(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            self.ppid.to_string(),
            self.state.to_string(),
            format!("{:.2}%", self.cpu),

        ]
    }
    pub fn format24(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.nice.to_string(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            
        ]
    }
    pub fn format25(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.nice.to_string(),
            format!("{:.2}%", self.cpu),
            self.state.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

            
        ]
    }
    pub fn format26(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.nice.to_string(),
            format!("{:.2}%", self.cpu),

            self.ppid.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

        ]
    }
    pub fn format27(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.nice.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            format!("{:.2}%", self.cpu),
           
        ]
    }
    pub fn format28(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.nice.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            self.state.to_string(),
            format!("{:.2}%", self.cpu),

            
        ]
    }
    pub fn format29(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            
            self.nice.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
           
            
            self.ppid.to_string(),
            format!("{:.2}%", self.cpu),

        ]
    }
    pub fn format30(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.nice.to_string(),
            self.state.to_string(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

            
        ]
    }
    pub fn format31(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.nice.to_string(),
            self.state.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            format!("{:.2}%", self.cpu),
            
        ]
    }
    pub fn format32(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.nice.to_string(),
            self.state.to_string(),
            self.ppid.to_string(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

        ]
    }
    pub fn format33(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.nice.to_string(),
            self.ppid.to_string(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

        ]
    }
    pub fn format34(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.nice.to_string(),
            
            self.ppid.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            format!("{:.2}%", self.cpu),

            
        ]
    }
    pub fn format35(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.nice.to_string(),
            
            self.ppid.to_string(),
            self.state.to_string(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
        ]
    }
    pub fn format36(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.state.to_string(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            
        ]
    }
    pub fn format37(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.state.to_string(),
            format!("{:.2}%", self.cpu),
            
            self.nice.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

            
        ]
    }
    pub fn format38(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            
            self.state.to_string(),
            format!("{:.2}%", self.cpu),
            self.ppid.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

        ]
    }
    pub fn format39(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.state.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            format!("{:.2}%", self.cpu),
        ]
    }
    pub fn format40(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.state.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            self.nice.to_string(),
            format!("{:.2}%", self.cpu),

        ]
    }
    pub fn format41(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),

            self.state.to_string(),

            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

            self.ppid.to_string(),
            format!("{:.2}%", self.cpu),

        ]
    }
    pub fn format42(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.state.to_string(),
            self.nice.to_string(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

        ]
    }
    pub fn format43(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            
            self.state.to_string(),
            self.nice.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            format!("{:.2}%", self.cpu),

            
        ]
    }
    pub fn format44(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.state.to_string(),
            self.nice.to_string(),
            self.ppid.to_string(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
        ]
    }
    pub fn format45(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.state.to_string(),
            self.ppid.to_string(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

            
            
        ]
    }
    pub fn format46(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            
            self.state.to_string(),
            self.ppid.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            format!("{:.2}%", self.cpu),

            
            
        ]
    }
    pub fn format47(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            
            self.state.to_string(),
            self.ppid.to_string(),
            self.nice.to_string(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),



        ]
    }
    pub fn format48(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.ppid.to_string(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            
            
        ]
    }
    pub fn format49(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.ppid.to_string(),
            format!("{:.2}%", self.cpu),
            self.nice.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

            
        ]
    }
    pub fn format50(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.ppid.to_string(),
            format!("{:.2}%", self.cpu),
            self.state.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

            
        ]
    }
    pub fn format51(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.ppid.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            format!("{:.2}%", self.cpu),
           
        ]
    }
    pub fn format52(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.ppid.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            self.nice.to_string(),
            format!("{:.2}%", self.cpu),


            
        ]
    }
    pub fn format53(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.ppid.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            self.state.to_string(),
            format!("{:.2}%", self.cpu),

        ]
    }
    pub fn format54(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),

            self.ppid.to_string(),
            self.nice.to_string(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

            
        ]
    }
    pub fn format55(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),

            self.ppid.to_string(),
            self.nice.to_string(),
            
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            format!("{:.2}%", self.cpu),

            
        ]
    }
    pub fn format56(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),

            self.ppid.to_string(),
            self.nice.to_string(),
            self.state.to_string(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),


            
        ]
    }
    pub fn format57(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.ppid.to_string(),
            self.state.to_string(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),

        ]
    }
    pub fn format58(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),
            self.ppid.to_string(),
            self.state.to_string(),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),
            format!("{:.2}%", self.cpu),

        ]
    }
    pub fn format59(&self) -> Vec<String> {
        vec![
            self.pid.to_string(),
            self.name.clone(),

            self.ppid.to_string(),
            self.state.to_string(),
            self.nice.to_string(),
            format!("{:.2}%", self.cpu),
            pretty_bytes::converter::convert((self.mem as f64) * 1000.0),


            
            
            
            
        ]
    }
    
}
