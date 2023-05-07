

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Filter {
    PID,
    Name,
    CPU,
    Memory,
    Ppid,
    Nice,
    def
}

impl std::str::FromStr for FilterBy {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "pid" => Ok(FilterBy::PID),
            "name" => Ok(FilterBy::Name),
            "cpu" => Ok(FilterBy::CPU),
            "mem" => Ok(FilterBy::Memory),
            "ppid"=> OK(SorFilterBytBy::Ppid),
            "nice"=> Ok(SorFilterBytBy::Nice),
            _ => Err(()),
        }
    }
}
