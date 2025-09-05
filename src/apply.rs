use crate::structs::{AppData, Theme};
use crate::utils::{rgb_to_hex, rgb_u8_to_f32, rgb_f32_to_u8};
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
        rgb_to_hex(theme.color6),
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
    write_string_to_file("/home/.config/polybar/config", polybarconfig);

    //Reset Polybar
    let mut kill_polybar = Command::new("pkill");
    kill_polybar.arg("polybar");
    kill_polybar.status().expect("Kill failed");
    let mut walpolybar = Command::new("wal-polybar");
    let mut polybar_main = Command::new("polybar");
    let mut polybar_second = Command::new("polybar");
    polybar_main.arg("maindisplay");
    polybar_second.arg("seconddisplay");
    walpolybar.spawn().expect("Wal Polybar Failed");
    polybar_main.spawn().expect("Main Polybar Failed");
    polybar_second.spawn().expect("Second Polybar Failed");
    //Theme Setter Scripts

    let mut waltokitty = Command::new("sh");
    waltokitty.arg("/home/.config/kitty/waltokitty.sh");
    waltokitty.status().expect("WaltoKitty Failed");
    

    //Get u8 color string triplets

    let color0u8 = rgb_f32_to_u8(theme.color0);
    let color1u8 = rgb_f32_to_u8(theme.color1);
    let color2u8 = rgb_f32_to_u8(theme.color2);
    let color3u8 = rgb_f32_to_u8(theme.color3);
    let color4u8 = rgb_f32_to_u8(theme.color4);
    let color5u8 = rgb_f32_to_u8(theme.color5);
    let color6u8 = rgb_f32_to_u8(theme.color6);
    let color7u8 = rgb_f32_to_u8(theme.color7);
    let color8u8 = rgb_f32_to_u8(theme.color8);
    let color9u8 = rgb_f32_to_u8(theme.color9);
    let color10u8 = rgb_f32_to_u8(theme.color10);
    let color11u8 = rgb_f32_to_u8(theme.color11);
    let color12u8 = rgb_f32_to_u8(theme.color12);
    let color13u8 = rgb_f32_to_u8(theme.color13);
    let color14u8 = rgb_f32_to_u8(theme.color14);
    let color15u8 = rgb_f32_to_u8(theme.color15);
    let foregroundu8 = rgb_f32_to_u8(theme.foreground);

    let color0u8string  = format!("[{}, {}, {}]", color0u8[0], color0u8[1], color0u8[2]);
    let color1u8string  = format!("[{}, {}, {}]", color1u8[0], color1u8[1], color1u8[2]);
    let color2u8string  = format!("[{}, {}, {}]", color2u8[0], color2u8[1], color2u8[2]);
    let color3u8string  = format!("[{}, {}, {}]", color3u8[0], color3u8[1], color3u8[2]);
    let color4u8string  = format!("[{}, {}, {}]", color4u8[0], color4u8[1], color4u8[2]);
    let color5u8string  = format!("[{}, {}, {}]", color5u8[0], color5u8[1], color5u8[2]);
    let color6u8string  = format!("[{}, {}, {}]", color6u8[0], color6u8[1], color6u8[2]);
    let color7u8string  = format!("[{}, {}, {}]", color7u8[0], color7u8[1], color7u8[2]);
    let color8u8string  = format!("[{}, {}, {}]", color8u8[0], color8u8[1], color8u8[2]);
    let color9u8string  = format!("[{}, {}, {}]", color9u8[0], color9u8[1], color9u8[2]);
    let color10u8string  = format!("[{}, {}, {}]", color10u8[0], color10u8[1], color10u8[2]);
    let color11u8string  = format!("[{}, {}, {}]", color11u8[0], color11u8[1], color11u8[2]);
    let color12u8string  = format!("[{}, {}, {}]", color12u8[0], color12u8[1], color12u8[2]);
    let color13u8string  = format!("[{}, {}, {}]", color13u8[0], color13u8[1], color13u8[2]);
    let color14u8string  = format!("[{}, {}, {}]", color14u8[0], color14u8[1], color14u8[2]);
    let color15u8string  = format!("[{}, {}, {}]", color15u8[0], color15u8[1], color15u8[2]);
    let foregroundu8string  = format!("[{}, {}, {}]", foregroundu8[0], foregroundu8[1], foregroundu8[2]);


    let chrome_theme: String = format!(
        "{{
            \"manifest_version\": 3,
            \"name\": \"WalTheme\",
            \"version\": \"1.0\",
            \"theme\": {{
            \"colors\": {{
            \"background_tab\": {},
            \"bookmark_text\": {},
            \"button_background\": {},
            \"frame\": {},
            \"ntp_background\": {},
            \"ntp_header\": {},
            \"ntp_link\": {},
            \"ntp_text\": {},
            \"omnibox_background\": {},
            \"omnibox_text\": {},
            \"tab_background_text\": {},
            \"tab_text\": {},
            \"toolbar\": {},
            \"toolbar_text\": {}
        }}
      }}
    }}",
    color6u8string,    //Background_tab
    foregroundu8string,    //Bookmark_Text
    color0u8string,    //Button_Background 
    color7u8string,    //Frame
    color6u8string,    //Ntp_Background
    color6u8string,    //Ntp_Header
    color6u8string,    //Ntp_Link
    foregroundu8string,    //Ntp_Text
    color7u8string,    //Omnibox_Background
    color0u8string,    //Omnibox_text
    foregroundu8string,    //Tab_Background_Text
    foregroundu8string,    //Tab_Texts
    color6u8string,    //Toolbar
    foregroundu8string    //Toolbar_Text
    );

    write_string_to_file("/home/ChromeTheme/manifest.json", chrome_theme);

    //dmenu 
    let dmenuconfig: String = format!(
        "dmenu_run -nb \"{}\" -nf \"{}\" -sb \"{}\" -sf \"{}\" ",
        rgb_to_hex(theme.polybar_background),
        rgb_to_hex(theme.polybar_foreground),
        rgb_to_hex(theme.color6),
        rgb_to_hex(theme.polybar_foreground)
    );

    write_string_to_file("/home/.config/i3/dmenuscript.sh", dmenuconfig);

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

fn write_string_to_file(dest: &str, contents: String){
    let path = Path::new(dest);
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}: {}", display, why),
        Ok(file) => file,
    };

    match file.write_all(contents.as_bytes()) {
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

