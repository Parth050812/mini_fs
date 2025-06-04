mod disk;
mod fs;

use fs::FileSystem;

fn main() {
    let disk_path: &str = "disk.bin";
    if !std::path::Path::new(disk_path).exists(){
        println!("Creating the Disk space");
        disk::init_disk(disk_path,1024*1024);
    }
    let disk_data = disk::load_disk(disk_path);
    let mut fs = match fs::FileSystem::deserialize(&disk_data){
        Ok(system)=>system,
        Err(_)=>{
            println!("Could not load file system");
            fs::init_fs();
        }
    };
}
