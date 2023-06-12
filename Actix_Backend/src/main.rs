
use actix_web::{middleware, web, App, HttpRequest, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use serde::Serializer;
use psutil::cpu::CpuTimesPercentCollector;
use actix_cors::Cors;

extern crate procfs;
extern crate sysinfo;
use std::fs::File;
use sysinfo::{NetworkExt, NetworksExt, ProcessExt, System, SystemExt};
use sysconf;
use std::io::{self, Write};
use std::time::Duration;
use std::sync::Arc;


use actix_web::{get};
use sysinfo::{CpuExt};


#[derive(Serialize, Deserialize)]
struct CpuUsage {
    core_id: usize,
    usage: f32,
}

// // #[get("/cpu")]
// async fn cpu_usage(system: web::Data<Arc<System>>) -> HttpResponse {
//     let mut sys = System::new_all();
//     sys.refresh_cpu();

//     let processors = sys.cpus();
//     let cpu_usage: Vec<CpuUsage> = processors
//         .iter()
//         .enumerate()
//         .map(|(i, cpu)| CpuUsage {
//             core_id: i,
//             usage: cpu.cpu_usage(),
//         })
//         .collect();
//     // sys.refresh_cpu();

//     HttpResponse::Ok().json(cpu_usage)
// }


async fn cpu_usage(system: web::Data<Arc<System>>) -> HttpResponse {
    let mut sys = System::new_all();
    sys.refresh_cpu();
    std::thread::sleep(std::time::Duration::from_millis(100));
    sys.refresh_cpu();

    let processors = sys.cpus();
    let cpu_usage: Vec<CpuUsage> = processors
        .iter()
        .enumerate()
        .map(|(i, cpu)| CpuUsage {
            core_id: i,
            usage: cpu.cpu_usage(),
        })
        .collect();

    HttpResponse::Ok().json(cpu_usage)
}
#[derive(Debug, Deserialize, Serialize)]
struct Process {
    name: String,
    pid: i32,
    state: String,
    parent_id: i32,
    priority: i64,
    niceness: i64,
    user_id: u32,
    memory: i64,
    cpu_time: String,
    opened_files: usize,
}



async fn get_processes(system: web::Data<Arc<System>>) -> HttpResponse {
    let mut processes = Vec::new();
    let sys = system.as_ref();

    for process in procfs::process::all_processes().unwrap() {
        let cpu_time = process.stat.utime + process.stat.stime;
        let cpu_time_secs = Duration::from_secs(cpu_time as u64 / sysconf::page::pagesize() as u64);
        let cputtime_str = format!("{:?}", cpu_time_secs);
        let open_files_count = match process.fd() {
            Ok(open_files) => open_files.len(),
            Err(_) => 0,
        };

        let process = Process {
            name: process.stat.comm,
            pid: process.pid,
            state: process.stat.state.to_string(),
            parent_id: process.stat.ppid,
            priority: process.stat.priority,
            niceness: process.stat.nice,
            user_id: process.owner,
            memory: process.stat.rss * (sysconf::page::pagesize() as i64) / 1024,
            cpu_time: cputtime_str,
            opened_files: open_files_count
        };
        processes.push(process)
    }
    HttpResponse::Ok().json(processes)
}



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let system = Arc::new(System::new_all());

    //print 

    HttpServer::new(move || {
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(
                // enable cors using cors middleware
                actix_cors::Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .app_data(web::Data::new(system.clone()))
            .route("/processes", web::get().to(get_processes))
            .route("/cpu", web::get().to(cpu_usage))
            
            // .service(cpu_usage)

    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}


