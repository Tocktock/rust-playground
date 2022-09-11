use sysinfo::{ProcessExt, SystemExt};

fn main() {
    let mut system = sysinfo::System::new();
    system.refresh_all();
    for (pid,process) in system.processes() {
        println!("{}:{}", pid, process.name());
    }
}