use actix_web::{get, web, App, HttpResponse, HttpServer};
use serde::{Deserialize, Serialize};
use sysinfo::{System, SystemExt};
use sysinfo::{CpuExt};


#[derive(Serialize, Deserialize)]
struct CpuUsage {
    core_id: usize,
    usage: f32,
}

#[get("/cpu")]
async fn cpu_usage() -> HttpResponse {
    let mut sys = System::new_all();
    loop {
        sys.refresh_cpu();
        if sys.cpus().len() > 0 {
            break;
        }
    }
    // sys.refresh_cpu();

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

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(cpu_usage))
        .bind("127.0.0.1:8080")?
        .run()
        .await
}
