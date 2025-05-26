use axum::{Json, response::IntoResponse};
use serde::Serialize;
use sysinfo::{CpuExt, System, SystemExt};

#[derive(Serialize)]
pub struct User {
    pub id: u64,
    pub nome: String,
}

pub async fn hello_world() -> &'static str {
    "Hello, World!"
}

pub async fn is_up() -> &'static str {
    "Server Up"
}

pub async fn get_user() -> impl IntoResponse {
    let user = User {
        id: 1,
        nome: "Jonathan Carvalho".to_string(),
    };
    Json(user)
}

#[derive(Serialize)]
pub struct HealthStatus {
    system: SystemStatus,
    status: String,
}

#[derive(Serialize)]
pub struct SystemStatus {
    memory: MemoryStatus,
    cpu: CpuStatus,
}

#[derive(Serialize)]
pub struct MemoryStatus {
    used_mb: u64,
    total_mb: u64,
    used_percent: u64,
    status: String,
}

#[derive(Serialize)]
pub struct CpuStatus {
    usage_percent: f32,
    status: String,
}

pub async fn health_check() -> impl IntoResponse {
    let mut sys = System::new_all();

    sys.refresh_all();

    let memory_status = {
        let used_mb = sys.used_memory() / 1024 / 1024;
        let total_mb = sys.total_memory() / 1024 / 1024;
        let used_percent = (used_mb * 100) / total_mb;

        MemoryStatus {
            used_mb,
            total_mb,
            used_percent,
            status: if used_percent < 90 {
                "UP".to_string()
            } else {
                "DOWN".to_string()
            },
        }
    };

    let cpu_status = {
        sys.refresh_cpu();
        // Espera um pouco para leitura mais precisa
        std::thread::sleep(std::time::Duration::from_millis(200));
        sys.refresh_cpu();

        let usage = sys.global_cpu_info().cpu_usage();

        CpuStatus {
            usage_percent: usage,
            status: if usage < 90.0 {
                "UP".to_string()
            } else {
                "DOWN".to_string()
            },
        }
    };

    let overall_status = if memory_status.status == "DOWN" && cpu_status.status == "DOWN" {
        "DOWN"
    } else {
        "UP"
    };

    Json(HealthStatus {
        system: SystemStatus {
            memory: memory_status,
            cpu: cpu_status,
        },
        status: overall_status.to_string(),
    })
}
