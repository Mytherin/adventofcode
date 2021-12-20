use std::fs::File;
use std::io::{self, BufRead};

const IMAGE_ALGORITHM_SIZE : usize = 512;
const BIT_COUNT : usize = 9;

struct Image {
    data : Vec<Vec<bool>>,
    infinite_pixel : bool,
}

impl Image {
    fn width(&self) -> usize {
        return self.data[0].len();
    }
    fn height(&self) -> usize {
        return self.data.len();
    }
    fn in_bounds(&self, x : usize, y : usize) -> bool {
        if x >= self.width() {
            return false;
        }
        if y >= self.height() {
            return false;
        }
        return true;
    }
    fn get_pixel(&self, x : usize, y : usize) -> bool {
        if !self.in_bounds(x, y) {
            return self.infinite_pixel;
        }
        return self.data[y][x];
    }
}

fn parse_line(line : &str, result : &mut Vec<bool>) {
    for character in line.chars() {
        if character == '#' {
            result.push(true);
        } else if character == '.' {
            result.push(false);
        } else {
            panic!("Unknown character");
        }
    }
}

fn print_image(image : &Image) {
    let width : usize = image.data[0].len();
    let height : usize = image.data.len();
    for y in 0..height {
        let mut line : String = String::new();
        for x in 0..width {
            if image.get_pixel(x ,y) {
                line.push('#');
            } else {
                line.push('.');
            }
        }
        println!("{}", line);
    }
}

fn add_padding(hpadding : usize, vpadding : usize, image : Image) -> Image {
    let width : usize = image.data[0].len();
    let height : usize = image.data.len();
    let new_width = width + hpadding * 2;
    let new_height = height + vpadding * 2;
    let mut result : Image = Image { data : Vec::new(), infinite_pixel: image.infinite_pixel };
    for y in 0..new_height {
        let mut line : Vec<bool> = Vec::new();
        for x in 0..new_width {
            if x >= hpadding && y >= vpadding {
                let original_x = x - hpadding;
                let original_y = y - vpadding;
                if original_x < width && original_y < height {
                    line.push(image.get_pixel(original_x, original_y));
                } else {
                    line.push(false);
                }
            } else {
                line.push(false);
            }
        }
        result.data.push(line);
    }
    return result;
}

fn count_pixels(image : &Image) -> usize {
    let count : usize = 0;
    if image.infinite_pixel {
        panic!("Infinite pixel is turned on... result is infinite?!");
    }
    let width : usize = image.width();
    let height : usize = image.height();
    let mut count : usize = 0;
    for y in 0..height {
        for x in 0..width {
            if image.get_pixel(x, y) {
                count += 1;
            }
        }
    }
    return count;
}

fn apply_algorithm_pixel(image_algorithm : &Vec<bool>, x : usize, y : usize, image : &Image) -> bool {
    let width : usize = image.data[0].len();
    let height : usize = image.data.len();
    let algorithm_index : usize = 0;
    // the bits are as follows
    // [x-1,y-1],[x,y-1],[x+1,y-1]
    // [x-1,y],[x,y],[x+1,y]
    // [x+1,y],[x+1,y],[x+1+1,y]
    let mut bits : Vec<bool> = Vec::new();
    for _i in 0..BIT_COUNT {
        bits.push(image.infinite_pixel);
    }
    let left_edge = x == 0;
    let right_edge = x + 1 == width;
    let top_edge = y == 0;
    let bottom_edge = y + 1 == height;
    if !top_edge {
        if !left_edge {
            bits[0] = image.data[y - 1][x - 1];
        }
        bits[1] = image.data[y - 1][x];
        if !right_edge {
            bits[2] = image.data[y - 1][x + 1];
        }
    }
    if !left_edge {
        bits[3] = image.data[y][x - 1];
    }
    bits[4] = image.data[y][x];
    if !right_edge {
        bits[5] = image.data[y][x + 1];
    }
    if !bottom_edge {
        if !left_edge {
            bits[6] = image.data[y + 1][x - 1];
        }
        bits[7] = image.data[y + 1][x];
        if !right_edge {
            bits[8] = image.data[y + 1][x + 1];
        }
    }
    // construct the number
    let mut result_index : usize = 0;
    for i in 0..BIT_COUNT {
        let bit : usize = (1 << (BIT_COUNT - i - 1));
        if bits[i] {
            result_index |= bit;
        } else {
            result_index &= !bit;
        }
    }
    return image_algorithm[result_index];
}

fn apply_algorithm(image_algorithm : &Vec<bool>, image : Image) -> Image {
    // figure out the infinite pixel
    // it is either the highest or the lowest value
    let mut new_infinite;
    if image.infinite_pixel {
        new_infinite = image_algorithm[IMAGE_ALGORITHM_SIZE - 1];
    } else {
        new_infinite = image_algorithm[0];
    }

    let mut result : Image = Image { data : Vec::new(), infinite_pixel: new_infinite };
    let width : usize = image.data[0].len();
    let height : usize = image.data.len();
    let mut count : usize = 0;
    for y in 0..height {
        let mut result_line : Vec<bool> = Vec::new();
        for x in 0..width {
            result_line.push(apply_algorithm_pixel(&image_algorithm, x, y, &image));
        }
        result.data.push(result_line);
    }
    return result;
}

fn main() {
    let file = File::open("input.txt").unwrap();
    let buf_reader = io::BufReader::new(file);
    let buf_lines = buf_reader.lines();
    let mut image : Image = Image { data : Vec::new(), infinite_pixel: false };
    let mut image_algorithm : Vec<bool> = Vec::new();
    let mut first_line = true;
    for line in buf_lines {
        let unwrapped_line = line.unwrap();
        if unwrapped_line.len() == 0 {
            continue;
        }
        if first_line {
            first_line = false;
            parse_line(&unwrapped_line, &mut image_algorithm);
        } else {
            let mut hline : Vec<bool> = Vec::new();
            parse_line(&unwrapped_line, &mut hline);
            image.data.push(hline);
        }
    }
    if image_algorithm.len() != IMAGE_ALGORITHM_SIZE {
        panic!("Image algo length {} wrong", image_algorithm.len());
    }
    print_image(&image);
    image = add_padding(10, 10, image);
    let width : usize = image.width();
    let height : usize = image.height();
    print_image(&image);
    image = apply_algorithm(&image_algorithm, image);
    print_image(&image);
    image = apply_algorithm(&image_algorithm, image);
    print_image(&image);
    println!("{}", count_pixels(&image));
}
