#[derive(Debug)]
pub struct Disk {
    pub total: u64,
    pub free: u64,
    pub used: u64,
}

fn create_disk(_total: u64, _free: u64, _used: u64) -> Disk {
    Disk {
        total: _total,
        free: _free,
        used: _used,
    }
}

pub fn disk() -> Vec<Disk> {
    let mut disks: Vec<Disk> = Vec::new();
    let mut total: u64 = 0;
    let mut free: u64 = 0;
    let mut used: u64 = 0;

    let output = std::process::Command::new("df")
        .arg("-k")
        .output()
        .expect("failed to execute process");

    let stdout = String::from_utf8_lossy(&output.stdout);
    let lines = stdout.split("\n");

    for line in lines {
        let fields: Vec<&str> = line.split_whitespace().collect();
        if fields.len() > 1 {
            if fields[0] == "Filesystem" {
                continue;
            }
            total = fields[1].parse::<u64>().unwrap();
            free = fields[3].parse::<u64>().unwrap();
            used = fields[2].parse::<u64>().unwrap();
            disks.push(create_disk(total, free, used));
        }
    }
    disks
}
