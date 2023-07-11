use sysinfo::{DiskExt, System, SystemExt};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ExternalDisk {
    pub name: String,
    pub mount_point: String,
    pub total_space: u64,
    pub available_space: u64,
    pub is_removable: bool,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct MainDisk {
    pub name: String,
    pub total_space: u64,
    pub available_space: u64,
}

pub fn return_external_disks() -> Vec<ExternalDisk> {
    create_disk_info()
}

pub fn return_main_disk() -> MainDisk {
    let mut s = System::new();
    let mut total_space: u64 = 0;
    let mut available_space: u64 = 0;
    let main_disk: MainDisk;
    s.refresh_disks_list();
    for disk in s.disks() {
        let name = disk.name().to_string_lossy();
        if &name[0..8] == "/dev/sda" { // Home
            total_space = total_space + disk.total_space();
            available_space = available_space + disk.available_space();
        }
    }
    main_disk = MainDisk {
        name: "Main Disk".to_string(),
        total_space: convert_to_gb(total_space),
        available_space: convert_to_gb(available_space),
    };
    main_disk
}

fn create_disk_info() -> Vec<ExternalDisk> {
    let mut s = System::new();
    let mut disks: Vec<ExternalDisk> = Vec::new();
    s.refresh_disks_list();
    for disk in s.disks() {
        let name = disk.name().to_string_lossy();
        if &name[0..8] != "/dev/sda" { // Home
            disks.push(ExternalDisk {
                name: name.to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                total_space: convert_to_gb(disk.total_space()),
                available_space: convert_to_gb(disk.available_space()),
                is_removable: disk.is_removable(),
            });
        }
    }
    disks
}

fn convert_to_gb(bytes: u64) -> u64 {
    let bytes = bytes as u64;
    let gb = bytes / 1000000000;
    gb
}
