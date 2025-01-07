use clap::Parser;
use figlet_rs::FIGfont;
use sysinfo::{Disks, Networks, System};
use termion::{color, style};

#[derive(Parser)]
#[command(
    version,
    about = "A simple CLI tool to show system information",
    arg_required_else_help = true
)]
struct Cli {
    #[arg(short, long, help = "Show all information")]
    all: bool,
    #[arg(short, long, help = "Show CPU information")]
    cpu: bool,
    #[arg(short, long, help = "Show memory information")]
    memory: bool,
    #[arg(short, long, help = "Show disk information")]
    disk: bool,
    #[arg(short, long, help = "Show network information")]
    network: bool,
}

fn main() {
    let font = FIGfont::from_file("resources/larry3d.flf").unwrap();
    let figure = font.convert("SysInfo CLI").unwrap();
    // Print the title with styling
    println!("{}", color::Fg(color::Yellow));
    println!("{}", style::Bold);
    println!("{}", figure);
    println!("{}", style::Reset);  // Reset after title

    let args = Cli::parse();
    let mut sys = System::new_all();
    sys.refresh_all();

    if args.all {
        print_sys_info();
        print_cpu_info(&mut sys);
        print_memory_info(&mut sys);
        print_disk_info();
        print_network_info();
    } else {
        match args {
            Cli { cpu: true, .. } => print_cpu_info(&mut sys),
            Cli { memory: true, .. } => print_memory_info(&mut sys),
            Cli { disk: true, .. } => print_disk_info(),
            Cli { network: true, .. } => print_network_info(),
            _ => print_sys_info(),
        }
    }
}

fn print_sys_info() {
    println!("\n");
    println!("{}", color::Fg(color::Cyan));
    println!("==========================");
    println!("    System Information    ");
    println!("==========================");
    println!("{}", style::Reset); // Reset after header

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
    println!("\n");
    println!("{}", color::Fg(color::Green));
    println!("==========================");
    println!("     CPU Information      ");
    println!("==========================");
    println!("{}", style::Reset); // Reset after header

    let cpu_name = sys.cpus().first().unwrap().brand();
    println!("CPU Model: {}", cpu_name);
    
    let cores = sys.physical_core_count().unwrap();
    println!("Cores: {}", cores);
    
    std::thread::sleep(sysinfo::MINIMUM_CPU_UPDATE_INTERVAL);
    sys.refresh_cpu_usage();
    let usage = sys.global_cpu_usage();
    println!("CPU Usage: {}{}", usage, "%");
}

fn print_memory_info(sys: &mut System) {
    println!("\n");
    println!("{}", color::Fg(color::Magenta));
    println!("==========================");
    println!("    Memory Information    ");
    println!("==========================");
    println!("{}", style::Reset); // Reset after header

    let total_memory: f64 = sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let used_memory: f64 = sys.used_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    let free_memory: f64 = sys.free_memory() as f64 / 1024.0 / 1024.0 / 1024.0;
    
    println!("Total Memory: {:.1} GB", total_memory);
    println!("Used Memory: {:.1} GB", used_memory);
    println!("Free Memory: {:.1} GB", free_memory);
}

fn print_disk_info() {
    println!("\n");
    println!("{}", color::Fg(color::Blue));
    println!("==========================");
    println!("     Disk Information     ");
    println!("==========================");
    println!("{}", style::Reset); // Reset after header

    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        println!("");
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

fn print_network_info() {
    println!("\n");
    println!("{}", color::Fg(color::Red));
    println!("==========================");
    println!("   Network Information    ");
    println!("==========================");
    println!("{}", style::Reset); // Reset after header

    let networks = Networks::new_with_refreshed_list();
    for (interface_name, data) in &networks {
        println!("");
        println!("Interface: {}", interface_name);
        let ip_networks = data.ip_networks();
        for ip_network in ip_networks {
            println!("IP Network: {}", ip_network);
        }
        println!("MAC Address: {}", data.mac_address());
    }
}
