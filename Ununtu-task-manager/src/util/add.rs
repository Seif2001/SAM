#[derive(PartialEq, Debug, Clone, Copy)]
pub enum Addby{
    CPU,
    Memory,
    Nice,
    state,
    ppid,
    Def
}

impl std::str::FromStr for Addby {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "cpu" => Ok(Addby::CPU),
            "mem" => Ok(Addby::Memory),
            "nice" => Ok(Addby::Nice),
            "state" => Ok(Addby::state),
            "ppid" => Ok(Addby::ppid),
            _ => Err(()),
        }
    }
}
