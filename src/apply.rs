use crate::structs::{AppData, Theme};
use std::process::Command;
use std::fs::File;
use std::path::Path;
use std::io::{BufReader, ErrorKind, Read, Write};

pub fn apply_theme(data: &AppData){
    //Background
    let mut background = Command::new("feh");
    background.args(["--bg-scale", data.themes.clone()[data.selected_index].imagepath.clone().as_str()]);
    background.status().expect("process failed to execute");

    //Set Wal Theme
    let theme = data.themes[data.selected_index].clone();
    let colorscheme = format!(
    "{{
        \"wallpaper\": \"{}\",
        \"alpha\": \"100\",
        \"special\": {{
            \"background\": \"{}\",
            \"foreground\": \"{}\",
            \"cursor\": \"{}\",
            \"polybar_background\": \"{}\",
            \"polybar_foreground\": \"{}\"
        }},
        \"colors\": {{
            \"color0\": \"{}\",
            \"color1\": \"{}\",
            \"color2\": \"{}\",
            \"color3\": \"{}\",
            \"color4\": \"{}\",
            \"color5\": \"{}\",
            \"color6\": \"{}\",
            \"color7\": \"{}\",
            \"color8\": \"{}\",
            \"color9\": \"{}\",
            \"color10\": \"{}\",
            \"color11\": \"{}\",
            \"color12\": \"{}\",
            \"color13\": \"{}\",
            \"color14\": \"{}\",
            \"color15\": \"{}\"
        }}
    }}", 
    theme.imagepath,
    rgb_to_hex(theme.background),
    rgb_to_hex(theme.foreground),
    rgb_to_hex(theme.foreground),
    rgb_to_hex(theme.polybar_background),
    rgb_to_hex(theme.polybar_foreground),
    rgb_to_hex(theme.color0),
    rgb_to_hex(theme.color1),
    rgb_to_hex(theme.color2),
    rgb_to_hex(theme.color3),
    rgb_to_hex(theme.color4),
    rgb_to_hex(theme.color5),
    rgb_to_hex(theme.color6),
    rgb_to_hex(theme.color7),
    rgb_to_hex(theme.color8),
    rgb_to_hex(theme.color9),
    rgb_to_hex(theme.color10),
    rgb_to_hex(theme.color11),
    rgb_to_hex(theme.color12),
    rgb_to_hex(theme.color13),
    rgb_to_hex(theme.color14),
    rgb_to_hex(theme.color15),
    
    );

    create_colorscheme(colorscheme);
    let mut wal = Command::new("wal");
    wal.args(["-f", "/home/.config/themer/colorscheme.json"]);
    wal.spawn().expect("Failed to Wal");


    let polybarscheme = format!(
"[colors]
background = {}
background-alt = {}
foreground = {}
foreground-alt = {}
primary = {}
secondary = {}
alert = {}
disabled = {}", 
        rgb_to_hex(theme.polybar_background),
        rgb_to_hex(theme.foreground),
        rgb_to_hex(theme.polybar_foreground),
        rgb_to_hex(theme.background),
        rgb_to_hex(theme.color7),
        rgb_to_hex(theme.color15),
        rgb_to_hex(theme.color1),
        rgb_to_hex(theme.color3),
        );

    let polybartemplate = get_polybar_template();
    let polybarconfig = format!("{}\n{}",polybarscheme, polybartemplate);
    println!("{}",polybarconfig);
    create_polybar_theme(polybarconfig);

    //Reset Polybar
    let mut kill_polybar = Command::new("pkill");
    kill_polybar.arg("polybar");
    kill_polybar.status().expect("Kill failed");
    let mut polybar_main = Command::new("polybar");
    let mut polybar_second = Command::new("polybar");
    polybar_main.arg("maindisplay");
    polybar_second.arg("seconddisplay");
    polybar_main.spawn().expect("Main Polybar Failed");
    polybar_second.spawn().expect("Second Polybar Failed");
    //Theme Setter Scripts

    let mut waltokitty = Command::new("sh");
    waltokitty.arg("/home/.config/kitty/waltokitty.sh");
    waltokitty.status().expect("WaltoKitty Failed");
    let mut waltochrome = Command::new("sh");
    waltochrome.arg("/home/WaltoChrome/WaltoChrome.sh");
    waltochrome.status().expect("WaltoChrome Failed");
}

pub fn rgb_to_hex(colorf: [f32; 3]) -> String{
    let color = rgb_f32_to_u8(colorf);
    //return format!("#{:02X}{:02X}{:02X}", color[0], color[1], color[2]);
    return format!("#{}", hex::encode(color));
}
fn rgb_u8_to_f32(rgb: [u8; 3]) -> [f32; 3] {
    println!("{},{},{}", rgb[0], rgb[1], rgb[2]);
    [
        (rgb[0] as f32 / 255.0).powf(2.2),
        (rgb[1] as f32 / 255.0).powf(2.2),
        (rgb[2] as f32 / 255.0).powf(2.2),
    ]
}


fn rgb_f32_to_u8(rgb: [f32; 3]) -> [u8; 3] {
    [
        (rgb[0].powf(1.0/2.2) * 255.0).round() as u8,
        (rgb[1].powf(1.0/2.2) * 255.0).round() as u8,
        (rgb[2].powf(1.0/2.2) * 255.0).round() as u8,
    ]
}

fn create_colorscheme(scheme: String){
    let path = Path::new("/home/.config/themer/colorscheme.json");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(scheme.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
    print!("Saved");
}

fn create_polybar_theme(scheme: String){
    let path = Path::new("/home/.config/polybar/config");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(scheme.as_bytes()) {
        Err(why) => panic!("couldn't write to {}: {}", display, why),
        Ok(_) => println!("successfully wrote to {}", display),
    }
    print!("Saved");
}

fn get_polybar_template() -> String{
    let file1 = File::open("/home/.config/polybar/config.template");
    let file2 = match file1 {
        Ok(file) => file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("/home/.config/polybar/config.template"){
                Ok(fc) => fc,
                Err(e) => panic!("Error Opening AppData.txt")
            },
            other_error => {
                panic!("Error Opening polybar template")
            }
        },
    };
    let mut reader = BufReader::new(file2);
    let mut content: String = "".to_string();
    reader.read_to_string(&mut content).unwrap();
    
    
    return content;
}

