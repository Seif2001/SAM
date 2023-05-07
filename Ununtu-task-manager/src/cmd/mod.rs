mod error;

pub use error::CmdError;

use crate::util::{ SortBy, SortDirection, Addby };
// TODO: Add quit and help commands
#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Action {
    Sort,
    Kill,
    IncreaseP,
    DecreaseP,
    Add,
    pid,
    name,
    state,
    nice,
    ppid,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Cmd<'a> {
    pub cmd: Action,
    pub args: Vec<nom::types::CompleteStr<'a>>
}

impl<'a> Cmd<'a> {
    pub fn exec(&self, app: &mut crate::app::App) -> Result<(), CmdError> {
        match self.cmd {
            Action::Sort => {
                // Make sure only 1 argument is supplied
                if self.args.len() != 1 {
                    return Err(CmdError::IncorrectArgNum(1, self.args.len() as u32));
                }
                // Parse the first (and only) argument into a SortBy enum
                match self.args[0].0.parse::<SortBy>() {
                    Ok(sort_by) => {
                        if app.processes_sort_by == sort_by {
                            app.processes_sort_direction = if app.processes_sort_direction == SortDirection::ASC {
                                SortDirection::ASC
                            } else {
                                SortDirection::DESC
                            };
                        } else {
                            app.processes_sort_direction = SortDirection::DESC;
                        }
                        app.processes_sort_by = sort_by;
                    }
                    // If parsing failed, an invalid argument was supplied
                    Err(()) => return Err(CmdError::InvalidArg(self.args[0].0))
                }
            }
            Action::Kill => {
                // Make sure only 1 argument is supplied
                if self.args.len() != 1 {
                    return Err(CmdError::IncorrectArgNum(1, self.args.len() as u32));
                }
                match self.args[0].0.parse::<i32>() {
                    Ok(pid) => app.system.kill_process(pid),
                    Err(_) => return Err(CmdError::Err("Invalid PID"))
                }
            }
            Action::IncreaseP => {
                // Make sure only 1 argument is supplied
                if self.args.len() != 1 {
                    return Err(CmdError::IncorrectArgNum(1, self.args.len() as u32));
                }
                match self.args[0].0.parse::<i32>() {
                    Ok(pid) => app.system.increase_priority(pid),
                    Err(_) => return Err(CmdError::Err("Invalid PID"))
                }
            }
            Action::DecreaseP => {
                // Make sure only 1 argument is supplied
                if self.args.len() != 1 {
                    return Err(CmdError::IncorrectArgNum(1, self.args.len() as u32));
                }
                match self.args[0].0.parse::<i32>() {
                    Ok(pid) => app.system.decrease_priority(pid),
                    Err(_) => return Err(CmdError::Err("Invalid PID"))
                }
            }
            Action::Add =>{
                // Make sure only 1 argument is supplied
                if self.args.len() != 1 {
                    return Err(CmdError::IncorrectArgNum(1, self.args.len() as u32));
                }
                // Parse the first (and only) argument into a addby enum
                match self.args[0].0.parse::<Addby>() {
                    Ok(add_by) => {
                        app.processes_add_by = add_by;
                    }
                    // If parsing failed, an invalid argument was supplied
                    Err(()) => return Err(CmdError::InvalidArg(self.args[0].0))
                }
            }
            Action::pid =>{
                // Make sure only 1 argument is supplied
                if self.args.len() != 1 {
                    return Err(CmdError::IncorrectArgNum(1, self.args.len() as u32));
                }
                // Parse the first (and only) argument into a addby enum
                match self.args[0].0.parse::<i32>() {
                    Ok(pid) => {
                        app.pid = pid;
                    },
                    // If parsing failed, an invalid argument was supplied
                    Err(_) => return Err(CmdError::Err("Invalid PID"))
                }
            }
            Action::name =>{
                // Make sure only 1 argument is supplied
                if self.args.len() != 1 {
                    return Err(CmdError::IncorrectArgNum(1, self.args.len() as u32));
                }
                // Parse the first (and only) argument into a addby enum
                match self.args[0].0.parse::<String>() {
                    Ok(name) =>{
                        app.name = name.clone()
                    },
                    // If parsing failed, an invalid argument was supplied
                    Err(_) => return Err(CmdError::Err("Invalid name"))
                }
            }
            Action::nice =>{
                // Make sure only 1 argument is supplied
                if self.args.len() != 1 {
                    return Err(CmdError::IncorrectArgNum(1, self.args.len() as u32));
                }
                // Parse the first (and only) argument into a addby enum
                match self.args[0].0.parse::<i64>() {
                    Ok(nice) => {
                        app.nice = nice;
                    }
                    // If parsing failed, an invalid argument was supplied
                    Err(_) => return Err(CmdError::Err("Invalid nice"))
                }
            }
            Action::state =>{
                // Make sure only 1 argument is supplied
                if self.args.len() != 1 {
                    return Err(CmdError::IncorrectArgNum(1, self.args.len() as u32));
                }
                // Parse the first (and only) argument into a addby enum
                match self.args[0].0.parse::<char>() {
                    Ok(state) => {
                        app.state = state;
                    }
                    // If parsing failed, an invalid argument was supplied
                    Err(_) => return Err(CmdError::Err("Invalid state"))
                }
            }
            Action::ppid =>{
                // Make sure only 1 argument is supplied
                if self.args.len() != 1 {
                    return Err(CmdError::IncorrectArgNum(1, self.args.len() as u32));
                }
                // Parse the first (and only) argument into a addby enum
                match self.args[0].0.parse::<i32>() {
                    Ok(ppid) => {
                        app.ppid = ppid;
                    }
                    // If parsing failed, an invalid argument was supplied
                    Err(_) => return Err(CmdError::Err("Invalid PID"))
                }
            }


        }

        Ok(())
    }
}
