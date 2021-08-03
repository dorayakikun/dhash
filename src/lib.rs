use image::imageops::FilterType::Triangle;

pub fn calculate(path: &str) -> String {
    // TODO error の実装
    let img = image::open(path).expect("open an image file");

    let smaller_img = img.resize_exact(9, 8, Triangle);
    let grayscale_img = smaller_img.grayscale();
    let pixels = grayscale_img.into_bytes();

    let mut diff: Vec<bool> = vec![];
    for row in 0..8 {
        let row_start_index = row * 9;
        for colum in 0..8 {
            let left_pixel_index = row_start_index + colum;
            diff.push(pixels[left_pixel_index] > pixels[left_pixel_index + 1]);
        }
    }

    let mut decimal_value = 0;
    let mut hash_string = "".to_owned();
    for (i, value) in diff.iter().enumerate() {
        if *value {
            decimal_value += 2 ^ (i % 8);
        }
        if i % 8 == 7 {
            hash_string = format!("{}{:0>2x}", hash_string, decimal_value);
            decimal_value = 0;
        }
    }

    hash_string
}

pub fn hamming_distance(a: &str, b: &str) -> u32 {
    // TODO: add error handling
    let i = i64::from_str_radix(a, 16).unwrap() ^ i64::from_str_radix(b, 16).unwrap();
    i.count_ones()
}
