extern crate procfs;
extern crate sysinfo;
use std::fs::File;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};

extern crate prettytable;
use sysconf;
// extern crate libc;
use prettytable::row;
use prettytable::Table;
use std::io::{self, Write};
use std::time::Duration;

fn main() -> io::Result<()> {
    // let mut count = 0;

    // while count < 5 {
    // Get a list of all processes from procfs
    let mut processes = procfs::process::all_processes().unwrap();
    
    // Please note that we use "new_all" to ensure that all list of
    // components, network interfaces, disks and users are already
    // filled!
    let mut sys = System::new_all();

    // Display system information:
    println!("System name:             {:?}", sys.name());
    println!("System kernel version:   {:?}", sys.kernel_version());
    println!("System OS version:       {:?}", sys.os_version());
    println!("System host name:        {:?}", sys.host_name());

    // Ask the user what field they want to sort by
    print!("Enter the field you want to sort by (name/pid/state/parent_id/priority/user_id/memory/cpu_time/open_files): ");
    io::stdout().flush().unwrap();
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let sort_field = input.trim();

    // Sort the processes based on the user's input
    match sort_field {
        "name" => processes.sort_by(|a, b| a.stat.comm.cmp(&b.stat.comm)),
        "pid" => processes.sort_by(|a, b| a.stat.pid.cmp(&b.stat.pid)),
        "state" => processes.sort_by(|a, b| a.stat.state.cmp(&b.stat.state)),
        "parent_id" => processes.sort_by(|a, b| a.stat.ppid.cmp(&b.stat.ppid)),
        "priority" => processes.sort_by(|a, b| a.stat.priority.cmp(&b.stat.priority)),
        "user_id" => processes.sort_by(|a, b| a.owner.cmp(&b.owner)),
        "memory" => processes.sort_by(|a, b| {
            (a.stat.rss * (sysconf::page::pagesize() as i64))
                .cmp(&(b.stat.rss * (sysconf::page::pagesize() as i64)))
        }),
        "cpu_time" => processes
            .sort_by(|a, b| (b.stat.utime + b.stat.stime).cmp(&(a.stat.utime + a.stat.stime))),
        "open_files" => processes.sort_by_key(|process| match process.fd() {
            Ok(open_files) => open_files.len(),
            Err(_) => 0,
        }),
        _ => println!("Invalid input!"),
    }

    let mut table = Table::new();

    table.add_row(row![
        "NAME",
        "PID",
        "STATE",
        "PARENT ID",
        "PRIORITY",
        "NICENESS",
        "USER ID",
        "MEMORY (KB)",
        "CPU TIME",
        "Network Bandwidth",
        "# OF Opened Files"
    ]); //, "PRIORITY", "CPU TIME"]);
    for process in processes {
        for (interface_name, data) in sys.networks() {
            // for (pid, process) in sys.processes() {

            let cpu_time = process.stat.utime + process.stat.stime;
            let cpu_time_secs =
                Duration::from_secs(cpu_time as u64 / sysconf::page::pagesize() as u64);
            let cputtime_str = format!("{:?}", cpu_time_secs);

            // let clock_ticks = procfs::ticks_per_second().unwrap();
            // let uptime = procfs::boot_time_secs().unwrap();

            // let cpu_time = process.stat.utime + process.stat.stime;
            // let seconds = uptime as f64 - (process.stat.starttime as f64 / clock_ticks as f64);
            // let cpu_usage = 100.0 * ((cpu_time as f64) / clock_ticks as f64) / seconds;

            let open_files_count = match process.fd() {
                Ok(open_files) => open_files.len(),
                Err(_) => 0,
            };

            // let open_files_names: Vec<String> = match process.fd() {
            //     Ok(open_files) => open_files.into_iter().filter_map(|file| match file.target {
            //         FDTarget::Path(path) => Some(format!("{:?}", path)),
            //         _ => None,
            //     }).collect(),
            //     Err(_) => Vec::new(),
            // };

            table.add_row(row![
                process.stat.comm,
                process.stat.pid,
                process.stat.state,
                process.stat.ppid,
                process.stat.priority,
                process.stat.nice,
                process.owner,
                process.stat.rss * (sysconf::page::pagesize() as i64) / 1024,
                cputtime_str,
                // process.disk_usage(),
                data.transmitted(),
                // format!("{:.2}%", cpu_usage),
                open_files_count // open_files_names.join(", ")
            ]);
            // }
        }
    }
    table.printstd();

    let mut file = File::create("Processes_Dump.csv")?;
    for row in table.row_iter() {
        let mut row_str = String::new();
        for cell in row.iter() {
            row_str.push_str(&cell.to_string());
            row_str.push(',');
        }
        row_str.pop(); // remove trailing comma
        writeln!(file, "{}", row_str)?;
    }
    // Flush output to make sure it appears immediately
    io::stdout().flush()?;
    //     let _ = Command::new("clear").status();
    //     // print!("{}", Cursor::Up(10));
    //     println!("COUNTER: {}", count);
    //     println!("Number of processes: {}", length);
    //     table.printstd();
    //     // count += 1;

    //     stdout().flush().unwrap();
    //     thread::sleep(Duration::from_millis(500));
    // }

    Ok(())
}
