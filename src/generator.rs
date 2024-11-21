use std::{vec, f64};
use std::collections::HashMap;
use colored::Colorize;

use image::{ImageBuffer, Pixel, Rgb, RgbImage, open, GenericImage, GenericImageView, DynamicImage, imageops::FilterType};
use crate::structs::{AppData, Theme};
pub fn generate_colors(data: &mut AppData){
    let imagepath: String = data.themes[data.selected_index].imagepath.clone();
    let imgbuf = downsize_image(imagepath);

    let mut pixels: Vec<Rgb<u8>> = Vec::new();
    for (x,y,pixel) in imgbuf.enumerate_pixels(){
        let Rgb([red, green, blue]) = *pixel;
        pixels.push(Rgb([red, green, blue]));
    }
    let limit: f64 = 200.0;
    let black: Rgb<u8> = find_color(pixels.clone(), Rgb([0.0, 0.0, 0.0]), limit);
    let dark_red: Rgb<u8> = find_color(pixels.clone(), Rgb([128.0, 0.0, 0.0]), limit);
    let dark_green: Rgb<u8> = find_color(pixels.clone(), Rgb([0.0, 128.0, 0.0]), limit);
    let dark_yellow: Rgb<u8> = find_color(pixels.clone(), Rgb([64.0, 64.0, 0.0]), limit);
    let dark_blue: Rgb<u8> = find_color(pixels.clone(), Rgb([0.0, 0.0, 128.0]), limit);
    let dark_purple: Rgb<u8> = find_color(pixels.clone(), Rgb([64.0, 0.0, 64.0]), limit);
    let dark_cyan: Rgb<u8> = find_color(pixels.clone(), Rgb([0.0, 64.0, 64.0]), limit);
    let dark_grey: Rgb<u8> = find_color(pixels.clone(), Rgb([64.0, 64.0, 64.0]), limit);
    let white: Rgb<u8> = find_color(pixels.clone(), Rgb([192.0, 192.0, 192.0]), limit);
    let red: Rgb<u8> = find_color(pixels.clone(), Rgb([192.0, 0.0, 0.0]), limit);
    let green: Rgb<u8> = find_color(pixels.clone(), Rgb([0.0, 192.0, 0.0]), limit);
    let yellow: Rgb<u8> = find_color(pixels.clone(), Rgb([192.0, 192.0, 0.0]), limit);
    let blue: Rgb<u8> = find_color(pixels.clone(), Rgb([0.0, 0.0, 192.0]), limit);
    let purple: Rgb<u8> = find_color(pixels.clone(), Rgb([192.0, 0.0, 192.0]), limit);
    let cyan: Rgb<u8> = find_color(pixels.clone(), Rgb([0.0, 192.0, 192.0]), limit);
    let grey: Rgb<u8> = find_color(pixels.clone(), Rgb([192.0, 192.0, 192.0]), limit);
    
    data.themes[data.selected_index].color0 = [black[0] as f32 / 255.0, black[1] as f32 / 255.0, black[2] as f32 / 255.0];
    data.themes[data.selected_index].color1 = [dark_red[0] as f32 / 255.0, dark_red[1] as f32 / 255.0, dark_red[2] as f32 / 255.0];
    data.themes[data.selected_index].color2 = [dark_green[0] as f32 / 255.0, dark_green[1] as f32 / 255.0, dark_green[2] as f32 / 255.0];
    data.themes[data.selected_index].color3 = [dark_yellow[0] as f32 / 255.0, dark_yellow[1] as f32 / 255.0, dark_yellow[2] as f32 / 255.0];
    data.themes[data.selected_index].color4 = [dark_blue[0] as f32 / 255.0, dark_blue[1] as f32 / 255.0, dark_blue[2] as f32 / 255.0];
    data.themes[data.selected_index].color5 = [dark_purple[0] as f32 / 255.0, dark_purple[1] as f32 / 255.0, dark_purple[2] as f32 / 255.0];
    data.themes[data.selected_index].color6 = [dark_cyan[0] as f32 / 255.0, dark_cyan[1] as f32 / 255.0, dark_cyan[2] as f32 / 255.0];
    data.themes[data.selected_index].color7 = [dark_grey[0] as f32 / 255.0, dark_grey[1] as f32 / 255.0, dark_grey[2] as f32 / 255.0];
    data.themes[data.selected_index].color8 = [white[0] as f32 / 255.0, white[1] as f32 / 255.0, white[2] as f32 / 255.0];
    data.themes[data.selected_index].color9 = [red[0] as f32 / 255.0, red[1] as f32 / 255.0, red[2] as f32 / 255.0];
    data.themes[data.selected_index].color10 = [green[0] as f32 / 255.0, green[1] as f32 / 255.0, green[2] as f32 / 255.0];
    data.themes[data.selected_index].color11 = [yellow[0] as f32 / 255.0, yellow[1] as f32 / 255.0, yellow[2] as f32 / 255.0];
    data.themes[data.selected_index].color12 = [blue[0] as f32 / 255.0, blue[1] as f32 / 255.0, blue[2] as f32 / 255.0];
    data.themes[data.selected_index].color13 = [purple[0] as f32 / 255.0, purple[1] as f32 / 255.0, purple[2] as f32 / 255.0];
    data.themes[data.selected_index].color14 = [cyan[0] as f32 / 255.0, cyan[1] as f32 / 255.0, cyan[2] as f32 / 255.0];
    data.themes[data.selected_index].color15 = [grey[0] as f32 / 255.0, grey[1] as f32 / 255.0, grey[2] as f32 / 255.0];

    data.themes[data.selected_index].foreground = [white[0] as f32 / 255.0, white[1] as f32 / 255.0, white[2] as f32 / 255.0];
    data.themes[data.selected_index].background = [black[0] as f32 / 255.0, black[1] as f32 / 255.0, black[2] as f32 / 255.0];
    data.themes[data.selected_index].polybar_background = [white[0] as f32 / 255.0, white[1] as f32 / 255.0, white[2] as f32 / 255.0];
    data.themes[data.selected_index].polybar_foreground = [black[0] as f32 / 255.0, black[1] as f32 / 255.0, black[2] as f32 / 255.0];


}

fn find_color(pixels: Vec<Rgb<u8>>, color: Rgb<f64>, limit: f64) -> Rgb<u8>{ 
    let mut mindif: f64 = 10000.0;
    let mut best_color: Rgb<u8> = Rgb([0,0,0]);
    for (i, pixel) in pixels.iter().enumerate(){
        let r_diff: f64 = (color[0] - (pixel[0] as f64)).powf(2.0);
        let g_diff: f64 = (color[1] - (pixel[1] as f64)).powf(2.0);
        let b_diff: f64 = (color[2] - (pixel[2] as f64)).powf(2.0);

        let c_diff: f64 = (r_diff + g_diff + b_diff).sqrt();
        if c_diff < mindif {
            mindif = c_diff;
            best_color = Rgb([pixel[0], pixel[1], pixel[2]]);
        }
    }
    /*
    println!("{}", mindif);
    println!("K: {}, {}, {}, {}", best_color[0], best_color[1], best_color[2], "Color".truecolor(best_color[0], best_color[1], best_color[2]));
    */
    if mindif < limit{
        return best_color;
    }
    else{
        return Rgb([color[0] as u8, color[1] as u8, color[2] as u8])
    }
}

fn downsize_image(imagepath: String) -> ImageBuffer<Rgb<u8>, Vec<u8>>{
    let image = image::open(imagepath).expect("Failed to Open Image");
    let down_image = image.resize(160, 100, FilterType::Nearest);
    //down_image.save("downscaled_image.jpg").expect("Failed to save image");
    let imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = down_image.to_rgb8();
    return imgbuf;
}


//GraveYard

/*
    let mut counts = HashMap::new();
    for (i, pixel) in pixels.iter().enumerate(){
        *counts.entry(pixel).or_insert(0) += 1;
    }
    /*
    for (item, count) in counts.iter().enumerate(){
        if *count.1 >=1 {
            println!("K: {}, {}, {}, V: {}, {}",count.0[0], count.0[1], count.0[2], count.1, "Color".truecolor(count.0[0], count.0[1], count.0[2]));
        }
    }
    */
*/