
use std::path::Path;
use std::env; 
use ico;


fn main() {
    let mut args = env::args();
    match args.nth(1) {
        None => {
            println!("[ERROR]: please enter image file name as argument next time");
        },
        Some(image_name) => {
            match env::current_dir() {
                Err(_) => {
                    println!("[ERROR]: problem during reading actual directory path");
                },
                Ok(dir) => {
                    let path = Path::new(dir.parent().unwrap());
                    let img_location = path.join(&image_name);
                    //println!("[IMAGE]: {}", img_location.to_str().unwrap());
                    process_image(img_location.as_path());
                },
            }
        },
    }
}


fn process_image(image_path: &Path) {
    let mut icon_dir = ico::IconDir::new(ico::ResourceType::Icon);
    //println!("[IMAGE TO OPEN]: {}", image_path.to_str().unwrap());
    let file = std::fs::File::open(image_path.file_name().unwrap()).unwrap();
    let image = ico::IconImage::read_png(file).unwrap();
    icon_dir.add_entry(ico::IconDirEntry::encode(&image).unwrap());
    let image_name = image_path.file_stem().unwrap();
    let icon_str = format!("{}.ico", image_name.to_str().unwrap());
    //println!("[ICON]: {}", &icon_str);
    let icon_name = Path::new(&icon_str);
    let file = std::fs::File::create(icon_name).unwrap();
    match icon_dir.write(file) {
        Err(_) => {
            println!("[ERROR]: problem during saving processed icon");
        },
        Ok(_) => {
            println!("[SUCCESS]: new icon in ready to use");
        },
    }
}
