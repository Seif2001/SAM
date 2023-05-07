use tui::layout::{Constraint, Direction, Layout, Rect, Corner};
use tui::widgets::{Block, Borders, Widget, Sparkline, Gauge, Row, Table, List, Text, Paragraph};
use tui::style::{Color, Style};
use tui::backend::Backend;
use tui::terminal::Frame;
use pretty_bytes::converter::convert;
use regex;
use crate::app::App;
use crate::process::{Process, self};
use crate::util::*;


// Helper function to make creating layouts easier
pub fn define_layout (direction: Direction, constraints: &[Constraint], location: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(direction)
        .constraints(constraints)
        .split(location)
}

pub fn render_console_layout<B> (f: &mut Frame<B>, layout: Rect, app: &App)
        where
        B: Backend {
            let log_text = app.console.history.iter().map(Text::raw);
            List::new(log_text)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .title("Console")
                )
                .start_corner(Corner::BottomLeft)
                .render(f, layout);
}

pub fn render_sparklines_layout<B> (f: &mut Frame<B>, layout: &[Rect], app: &App)
    where
    B: Backend {
    Gauge::default()
        .block(
            Block::default()
                .title(&format!("Number of Cores: {}", app.system.cpu_num_cores))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
        )
        .style(Style::default().fg(Color::Yellow))
        .percent((*app.system.cpu_usage_history.as_slice())[app.system.cpu_usage_history.len()-1] as u16)
        .render(f, layout[0]);

    Gauge::default()
        .block(
            Block::default()
                .title(&format!("Memory Used: {} | Memory Free: {}", convert(app.system.mem_used as f64 * 1000.0), convert(app.system.mem_free as f64 * 1000.0)))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
        )
        .percent(((app.system.mem_used as f32/ app.system.mem_total as f32 ) as f32 *100.0) as u16) //changed to f32 and 100.0
        .style(Style::default().fg(Color::Blue))
        .render(f, layout[1]);
}

pub fn render_cpu_cores_layout<B> (f: &mut Frame<B>, layout: &[Rect], app: &App)
    where
    B: Backend {
    // Creates a guage for each cpu core
    for (i, core_usage) in app.system.cpu_core_usages.iter().enumerate() {
        Gauge::default()
            .block(
                Block::default()
                .title(&format!("Core {}", i + 1))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Green))
            )
            .style(Style::default().fg(Color::Green))
            .percent(*core_usage)
            .render(f, layout[i]);
    }
}

pub fn render_processes_layout<B> (f: &mut Frame<B>, layout: Rect, app: &mut App)
    where
    B: Backend {

    let mut processes: Vec<Process> = app.system.processes.clone();

    match app.processes_sort_by {
        SortBy::PID => processes.sort_by(|a, b| a.pid.partial_cmp(&b.pid).unwrap()),
        SortBy::Name => processes.sort_by(|a, b| a.name.partial_cmp(&b.name).unwrap()),
        SortBy::CPU => processes.sort_by(|a, b| a.cpu.partial_cmp(&b.cpu).unwrap()),
        SortBy::Memory => processes.sort_by(|a, b| a.mem.partial_cmp(&b.mem).unwrap()),
        SortBy::Ppid => processes.sort_by(|a, b| a.ppid.partial_cmp(&b.ppid).unwrap()),
        SortBy::Nice => processes.sort_by(|a, b| a.nice.partial_cmp(&b.nice).unwrap())
    }
    
    if app.processes_sort_direction == SortDirection::DESC {
        processes.reverse();
    }
    if app.pid != -1 {
        processes = filterby_pid(app.pid, processes);
    }
    if app.name != String::from("") {
        processes = filterby_name(app.name.clone(), processes);
    }
    if app.ppid != -1 {
        processes = filterby_ppid(app.ppid, processes);
    }
    if app.state != 'x' {
        processes = filterby_state(app.state.clone(), processes);
    }
    if app.nice != -21 {
        processes = filterby_nice(app.nice, processes);
    }
    static mut headers: [&str; 5] = ["PID", "Name", "CPU", "Memory", "Nice"];
    static mut mem:usize = 3;
    static mut cpu:usize = 2;
    let mut fmt_processes: Vec<Vec<String>> = processes.iter().map(|process| process.format0()).collect();
    
    match app.processes_add_by{
        
        Addby::ppid => {
            unsafe{
            headers = add_ppid( headers);
        }
        
            app.processes_add_by = Addby::Memory;
        },
        Addby::CPU => {
            unsafe{
            headers = add_cpu(headers);
            
            app.processes_add_by = Addby::Memory;
            
        }
            
        },
        Addby::Memory => unsafe{
            headers = add_mem(headers);
            
            app.processes_add_by = Addby::Def;
            
        },
        Addby::state => unsafe{
            headers = add_state(headers);
            
            app.processes_add_by = Addby::Def;
            
        },
        Addby::Nice => unsafe{
            headers = add_Nice(headers);
            
            app.processes_add_by = Addby::Def;
        },  
        Addby::Def => unsafe{
            headers = headers;
        }
            
        
    }
    unsafe{
        if headers[2] == "CPU" && headers[3] == "Memory" && headers[4] == "Nice"{
            fmt_processes = processes.iter().map(|process| process.format0()).collect();
            cpu = 2;
            mem = 3;
        }

        if headers[2] == "CPU" && headers[3] == "Memory" && headers[4] == "state"{
            fmt_processes = processes.iter().map(|process| process.format1()).collect();
            cpu = 2;
            mem = 3;
        }

        if headers[2] == "CPU" && headers[3] == "Memory" && headers[4] == "ppid"{
            fmt_processes = processes.iter().map(|process| process.format2()).collect();
            cpu = 2;
            mem = 3;
        }

        if headers[2] == "CPU" && headers[3] == "Nice" && headers[4] == "Memory"{
            fmt_processes = processes.iter().map(|process| process.format3()).collect();
            cpu = 2;
            mem = 4;
        }

        if headers[2] == "CPU" && headers[3] == "Nice" && headers[4] == "state"{
            fmt_processes = processes.iter().map(|process| process.format4()).collect();
            cpu = 2;
            mem = 5;
        }

        if headers[2] == "CPU" && headers[3] == "Nice" && headers[4] == "ppid"{
            fmt_processes = processes.iter().map(|process| process.format5()).collect();
            cpu = 2;
            mem = 5;
        }

        if headers[2] == "CPU" && headers[3] == "state" && headers[4] == "Memory"{
            fmt_processes = processes.iter().map(|process| process.format6()).collect();
            cpu = 2;
            mem = 4;
        }

        if headers[2] == "CPU" && headers[3] == "state" && headers[4] == "Nice"{
            fmt_processes = processes.iter().map(|process| process.format7()).collect();
            cpu = 2;
            mem = 5;
        }

        if headers[2] == "CPU" && headers[3] == "state" && headers[4] == "ppid"{
            fmt_processes = processes.iter().map(|process| process.format8()).collect();
            cpu = 2;
            mem = 5;
        }

        if headers[2] == "CPU" && headers[3] == "ppid" && headers[4] == "Memory"{
            fmt_processes = processes.iter().map(|process| process.format9()).collect();
            cpu = 2;
            mem = 4;


        }

        if headers[2] == "CPU" && headers[3] == "ppid" && headers[4] == "Nice"{
            fmt_processes = processes.iter().map(|process| process.format10()).collect();
            cpu = 2;
            mem = 5;
        }

        if headers[2] == "CPU" && headers[3] == "ppid" && headers[4] == "state"{
            fmt_processes = processes.iter().map(|process| process.format11()).collect();
            cpu = 2;
            mem = 5;
        }

        if headers[2] == "Memory" && headers[3] == "CPU" && headers[4] == "Nice"{
            fmt_processes = processes.iter().map(|process| process.format12()).collect();
            cpu = 3;
            mem = 2;
        }

        if headers[2] == "Memory" && headers[3] == "CPU" && headers[4] == "state"{
            fmt_processes = processes.iter().map(|process| process.format13()).collect();
            cpu = 3;
            mem = 2;
        }
        
        if headers[2] == "Memory" && headers[3] == "CPU" && headers[4] == "ppid"{
            fmt_processes = processes.iter().map(|process| process.format14()).collect();
            cpu = 3;
            mem = 2;

        }

        if headers[2] == "Memory" && headers[3] == "Nice" && headers[4] == "CPU"{
            fmt_processes = processes.iter().map(|process| process.format15()).collect();
            cpu = 4;
            mem = 2;
        }

        if headers[2] == "Memory" && headers[3] == "Nice" && headers[4] == "state"{
            fmt_processes = processes.iter().map(|process| process.format16()).collect();
            cpu = 5;
            mem = 2;
        }

        if headers[2] == "Memory" && headers[3] == "Nice" && headers[4] == "ppid"{
            fmt_processes = processes.iter().map(|process| process.format17()).collect();
            cpu = 5;
            mem = 2;
        }
        
        if headers[2] == "Memory" && headers[3] == "state" && headers[4] == "CPU"{
            fmt_processes = processes.iter().map(|process| process.format18()).collect();
            cpu = 4;
            mem = 2;
        }
        
        if headers[2] == "Memory" && headers[3] == "state" && headers[4] == "Nice"{
            fmt_processes = processes.iter().map(|process| process.format19()).collect();
            cpu = 5;
            mem = 2;
        }

        if headers[2] == "Memory" && headers[3] == "state" && headers[4] == "ppid"{
            fmt_processes = processes.iter().map(|process| process.format20()).collect();
            cpu = 5;
            mem = 2;
        }

        if headers[2] == "Memory" && headers[3] == "ppid" && headers[4] == "CPU"{
            fmt_processes = processes.iter().map(|process| process.format21()).collect();
            cpu = 4;
            mem = 2;
        }


        if headers[2] == "Memory" && headers[3] == "ppid" && headers[4] == "Nice"{
            fmt_processes = processes.iter().map(|process| process.format22()).collect();
            cpu = 5;
            mem = 2;
        }

        if headers[2] == "Memory" && headers[3] == "ppid" && headers[4] == "state"{
            fmt_processes = processes.iter().map(|process| process.format23()).collect();
            cpu = 5;
            mem = 2;
        }

        if headers[2] == "Nice" && headers[3] == "CPU" && headers[4] == "Memory"{
            fmt_processes = processes.iter().map(|process| process.format24()).collect();
            cpu = 3;
            mem = 4;
        }

        if headers[2] == "Nice" && headers[3] == "CPU" && headers[4] == "state"{
            fmt_processes = processes.iter().map(|process| process.format25()).collect();
            cpu = 3;
            mem = 5;
        }

        if headers[2] == "Nice" && headers[3] == "CPU" && headers[4] == "ppid"{
            fmt_processes = processes.iter().map(|process| process.format26()).collect();
            cpu = 3;
            mem = 5;
        }

        if headers[2] == "Nice" && headers[3] == "Memory" && headers[4] == "CPU"{
            fmt_processes = processes.iter().map(|process| process.format27()).collect();
            cpu = 4;
            mem = 3;
        }
        if headers[2] == "Nice" && headers[3] == "Memory" && headers[4] == "state"{
            fmt_processes = processes.iter().map(|process| process.format28()).collect();
            cpu = 5;
            mem = 3;
        }

        if headers[2] == "Nice" && headers[3] == "CPU" && headers[4] == "ppid"{
            fmt_processes = processes.iter().map(|process| process.format29()).collect();
            cpu = 3;
            mem = 5;
        }

        if headers[2] == "Nice" && headers[3] == "state" && headers[4] == "CPU"{
            fmt_processes = processes.iter().map(|process| process.format30()).collect();
            cpu = 4;
            mem = 5;
        }

        if headers[2] == "Nice" && headers[3] == "state" && headers[4] == "Memory"{
            fmt_processes = processes.iter().map(|process| process.format31()).collect();
            cpu = 5;
            mem = 4;
        }

        if headers[2] == "Nice" && headers[3] == "state" && headers[4] == "ppid"{
            fmt_processes = processes.iter().map(|process| process.format32()).collect();
            cpu = 5;
            mem = 6;
        }

        if headers[2] == "Nice" && headers[3] == "ppid" && headers[4] == "CPU"{
            fmt_processes = processes.iter().map(|process| process.format33()).collect();
            cpu = 4;
            mem = 5;
        }

        if headers[2] == "Nice" && headers[3] == "ppid" && headers[4] == "Memory"{
            fmt_processes = processes.iter().map(|process| process.format34()).collect();
            cpu = 5;
            mem = 4;
        }

        if headers[2] == "Nice" && headers[3] == "ppid" && headers[4] == "state"{
            fmt_processes = processes.iter().map(|process| process.format35()).collect();
            cpu = 5;
            mem = 6;
        }
        
        if headers[2] == "state" && headers[3] == "CPU" && headers[4] == "Memory"{
            fmt_processes = processes.iter().map(|process| process.format36()).collect();
            cpu = 3;
            mem = 4;
        }

        if headers[2] == "state" && headers[3] == "CPU" && headers[4] == "Nice"{
            fmt_processes = processes.iter().map(|process| process.format37()).collect();
            cpu = 3;
            mem = 5;
        }
        
        if headers[2] == "state" && headers[3] == "CPU" && headers[4] == "ppid"{
            fmt_processes = processes.iter().map(|process| process.format38()).collect();
            cpu = 3;
            mem = 5;
        }

        if headers[2] == "state" && headers[3] == "Memory" && headers[4] == "CPU"{
            fmt_processes = processes.iter().map(|process| process.format39()).collect();
            cpu = 4;
            mem = 3;
        }

        if headers[2] == "state" && headers[3] == "Memory" && headers[4] == "Nice"{
            fmt_processes = processes.iter().map(|process| process.format40()).collect();
            cpu = 5;
            mem = 3;
        }

        if headers[2] == "state" && headers[3] == "Memory" && headers[4] == "ppid"{
            fmt_processes = processes.iter().map(|process| process.format41()).collect();
            cpu = 5;
            mem = 3;
        }

        if headers[2] == "state" && headers[3] == "Nice" && headers[4] == "CPU"{
            fmt_processes = processes.iter().map(|process| process.format42()).collect();
            cpu = 4;
            mem = 5;
        }

        if headers[2] == "state" && headers[3] == "Nice" && headers[4] == "Memory"{
            fmt_processes = processes.iter().map(|process| process.format43()).collect();
            cpu = 5;
            mem = 4;
        }

        if headers[2] == "state" && headers[3] == "Nice" && headers[4] == "ppid"{
            fmt_processes = processes.iter().map(|process| process.format44()).collect();
            cpu = 5;
            mem = 6;
        }

        if headers[2] == "state" && headers[3] == "ppid" && headers[4] == "CPU"{
            fmt_processes = processes.iter().map(|process| process.format45()).collect();
            cpu = 4;
            mem = 5;
        }

        if headers[2] == "state" && headers[3] == "ppid" && headers[4] == "Memory"{
            fmt_processes = processes.iter().map(|process| process.format46()).collect();
            cpu = 5;
            mem = 4;
        }

        if headers[2] == "state" && headers[3] == "ppid" && headers[4] == "Nice"{
            fmt_processes = processes.iter().map(|process| process.format47()).collect();
            cpu = 5;
            mem = 6;
        }

        if headers[2] == "ppid" && headers[3] == "CPU" && headers[4] == "Memory"{
            fmt_processes = processes.iter().map(|process| process.format48()).collect();
            cpu = 3;
            mem = 4;
        }

        if headers[2] == "ppid" && headers[3] == "CPU" && headers[4] == "Nice"{
            fmt_processes = processes.iter().map(|process| process.format49()).collect();
            cpu = 3;
            mem = 5;
        }

        if headers[2] == "ppid" && headers[3] == "CPU" && headers[4] == "state"{
            fmt_processes = processes.iter().map(|process| process.format50()).collect();
            cpu = 3;
            mem = 5;
        }

        if headers[2] == "ppid" && headers[3] == "Memory" && headers[4] == "CPU"{
            fmt_processes = processes.iter().map(|process| process.format51()).collect();
            cpu = 4;
            mem = 3;
        }

        if headers[2] == "ppid" && headers[3] == "Memory" && headers[4] == "Nice"{
            fmt_processes = processes.iter().map(|process| process.format52()).collect();
            cpu = 5;
            mem = 3;
        }


        if headers[2] == "ppid" && headers[3] == "Memory" && headers[4] == "state"{
            fmt_processes = processes.iter().map(|process| process.format53()).collect();
            cpu = 5;
            mem = 3;
        }

        if headers[2] == "ppid" && headers[3] == "Nice" && headers[4] == "CPU"{
            fmt_processes = processes.iter().map(|process| process.format54()).collect();
            cpu = 4;
            mem = 5;
        }

        if headers[2] == "ppid" && headers[3] == "Nice" && headers[4] == "Memory"{
            fmt_processes = processes.iter().map(|process| process.format55()).collect();
            cpu = 5;
            mem = 4;
        }

        if headers[2] == "ppid" && headers[3] == "Nice" && headers[4] == "state"{
            fmt_processes = processes.iter().map(|process| process.format56()).collect();
            cpu = 5;
            mem = 6;
        }

        if headers[2] == "ppid" && headers[3] == "state" && headers[4] == "CPU"{
            fmt_processes = processes.iter().map(|process| process.format57()).collect();
            cpu = 4;
            mem = 5;
        }

        if headers[2] == "ppid" && headers[3] == "state" && headers[4] == "Memory"{
            fmt_processes = processes.iter().map(|process| process.format58()).collect();
            cpu = 5;
            mem = 4;
        }

        if headers[2] == "ppid" && headers[3] == "state" && headers[4] == "Nice"{
            fmt_processes = processes.iter().map(|process| process.format59()).collect();
            cpu = 5;
            mem = 6;
        }
        let rows = fmt_processes.iter().map(|process| {
            let row = Row::StyledData(process.iter(), Style::default());
            let redrow =  Row::StyledData(process.iter(), Style::default().fg(Color::Red));
            let mut cpuu = 0;
            let mut memm = 0.0;
            if process[cpu].contains("%"){
                cpuu = process[cpu].replace("%", "").replace(".", "").trim().parse::<i32>().unwrap();
            }
            else{
                cpuu = 0;
            }
            if process[mem].contains("MB") || process[mem].contains("B"){
                memm = process[mem].replace("B", "").replace("M","").trim().parse::<f32>().unwrap()
            }
            else{
                memm = 0.0;
            }
            if ( cpuu> 150 || memm> 100.0 )
                {return redrow;}
            else
                {return row;}
        }
        );
        // TODO: Show visual indication of sort direction
    Table::new(unsafe{headers.iter()}, rows)
    .block(Block::default().borders(Borders::ALL).title("Processes"))
    .widths(&[6, 25, 9, 9, 9 ])
    .column_spacing(5)
    .render(f, layout);
    }


    
}


fn add_cpu(headers: [&str; 5]) -> [&str; 5] {
        if (!(headers[2] == "CPU" || headers[3] == "CPU" || headers[4] == "CPU")){
            let mut headerss = [headers[0], headers[1], headers[3], headers[4], "CPU"];

            return headerss;
        }
        else{
            return headers;
        }
    
}

// TODO: Show visual indication of invalid command
pub fn render_input_layout<B> (f: &mut Frame<B>, layout: Rect, app: &App)
    where B: Backend {
    Paragraph::new([Text::raw(&app.console.input)].iter())
        .style(Style::default().fg(Color::White))
        .block(Block::default().borders(Borders::ALL).title("Input"))
        .render(f, layout);
}


pub fn add_ppid( headers: [&str; 5])-> [&str;5]{
    let headerss = [headers[0], headers[1], headers[3], headers[4], "ppid"];
    headerss
}

fn add_mem(headers: [&str; 5]) -> [&str; 5] {
    if (!(headers[2] == "Memory" || headers[3] == "Memory" || headers[4] == "Memory")){
        let mut headerss = [headers[0], headers[1], headers[3], headers[4], "Memory"];
        return headerss;
    }
    else{
        return headers;
    }
}
fn add_state(headers: [&str; 5]) -> [&str; 5] {
    if (!(headers[2] == "state" || headers[3] == "state" || headers[4] == "state")){
        let mut headerss = [headers[0], headers[1], headers[3], headers[4], "state"];
        return headerss;
    }
    else{
        return headers;
    }
}
fn add_Nice(headers: [&str; 5]) -> [&str; 5] {
    if (!(headers[2] == "Nice" || headers[3] == "Nice" || headers[4] == "Nice")){
        let mut headerss = [headers[0], headers[1], headers[3], headers[4], "Nice"];
        return headerss;
    }
    else{
        return headers;
    }
}

fn filterby_pid(pid:i32, process:Vec<Process>)-> Vec<Process>{
    let mut procs:Vec<Process> = Vec::new();
    for p in process{
        if p.pid == pid{
            procs.push(p)
        }
    }
    procs

}

fn filterby_name(name:String, process:Vec<Process>)-> Vec<Process>{
    
     let mut procs:Vec<Process> = Vec::new();
    for p in process{
    let mut format_ff = format!(r"{name}");
    
    
    let re = regex::Regex::new(&format_ff).unwrap();
    if re.is_match(&p.name){
        procs.push(p);
    }
    }
    procs

}

fn filterby_ppid(ppid:i32, process:Vec<Process>)-> Vec<Process>{
    let mut procs:Vec<Process> = Vec::new();
    for p in process{
        if p.ppid == ppid{
            procs.push(p)
        }
    }
    procs

}

fn filterby_state(s:char, process:Vec<Process>)-> Vec<Process>{
    let mut procs:Vec<Process> = Vec::new();
    for p in process{
        if p.state == s.to_ascii_uppercase(){
            procs.push(p)
        }
    }
    procs

}

fn filterby_nice(nice:i64, process:Vec<Process>)-> Vec<Process>{
    let mut procs:Vec<Process> = Vec::new();
    for p in process{
        if p.nice == nice{
            procs.push(p)
        }
    }
    procs

}