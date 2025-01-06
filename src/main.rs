use sysinfo::{Disks, System};

fn main() {
    let mut sys = System::new_all();
    sys.refresh_all();
    print_sys_info();
    print_cpu_info(&mut sys);
    print_memory_info(&mut sys);
    print_disk_info();
}

fn print_sys_info() {
    println!("==========================");
    println!("    System Information    ");
    println!("==========================");
    let os_name = System::name().unwrap_or_else(|| "Unknown".to_string());
    let os_version = System::os_version().unwrap_or_else(|| "Unknown".to_string());
    println!("Operating System: {} {}", os_name, os_version);
    let kernel_version = System::kernel_version().unwrap_or_else(|| "Unknown".to_string());
    println!("Kernel Version: {}", kernel_version);
    println!("Architecture: {}", System::cpu_arch());
    let uptime = System::uptime();
    let uptime_hours = uptime / 3600;
    let uptime_minutes = (uptime % 3600) / 60;
    let uptime_seconds = uptime % 60;
    println!(
        "Uptime: {} hours, {} minutes, {} seconds",
        uptime_hours, uptime_minutes, uptime_seconds
    );
}

fn print_cpu_info(sys: &mut System) {
    println!("==========================");
    println!("CPU Information");
    println!("==========================");
    let cpu_name = sys.cpus().first().unwrap().brand();
    println!("CPU Model: {}", cpu_name);
    let cores = sys.physical_core_count().unwrap();
    println!("Cores: {}", cores);
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_usage();
    let usage = sys.global_cpu_usage();
    println!("CPU Usage: {:.0}%", usage);
}

fn print_memory_info(sys: &mut System) {
    println!("==========================");
    println!("Memory Information");
    println!("==========================");
    let total_memory: f64 = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_memory: f64 = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let free_memory: f64 = sys.free_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    println!("Total Memory: {:.1} GB", total_memory);
    println!("Used Memory: {:.1} GB", used_memory);
    println!("Free Memory: {:.1} GB", free_memory);
}

fn print_disk_info() {
    println!("==========================");
    println!("Disk Information");
    println!("==========================");
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        let name = disk.mount_point();
        let total_space: f64 = disk.total_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        let free_space: f64 = disk.available_space() as f64 / 1024.0 / 1024.0 / 1024.0;
        println!(
            "{}: {:.1} GB total, {:.1} GB free",
            name.display(),
            total_space,
            free_space
        );
    }
}
