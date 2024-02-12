use std::{io::{self, Write}, cmp::{Ordering, min}, ops::Range};

use image::{io::Reader, GenericImageView, Rgb, ImageBuffer};
use rand::Rng;

const SLICE_RANGE: Range<usize> = 20..160;

fn main() {

	// reading filename
	let mut filename = String::new();
	print!("Enter filename: ");
	io::stdout().flush().unwrap();
	io::stdin().read_line(&mut filename).expect("Failed to read filename");
	filename = filename.trim().to_owned();

	// opening and decoding image
	let input_img = Reader::open(filename.clone())
		.expect("Couldn't open image")
		.decode()
		.expect("Couldn't decode image");

	// reading dimensions
	let (width, height) = input_img.dimensions();
	let (w, h) = (width as usize, height as usize);

	// reading pixels
	let mut pixels: Vec<[u8; 3]> = input_img
		.as_rgb8()
		.unwrap()
		.to_vec()
		.chunks(3)
		.map(|chunk| [chunk[0], chunk[1], chunk[2]])
		.collect();

	// sorting pixels horizontally
	for y in 0..h {
		let mut i: usize = 0;
		while i < w {
			let slice_len = min(rand::thread_rng().gen_range(SLICE_RANGE), w - i);
			{
				let range = (y * w + i)..(y * w + i + slice_len);
				let slice = &mut pixels[range];
				slice.sort_by(|p1, p2| comp_rgb(&p1, &p2))
			}
			i += slice_len;
		}
	}
	// flattening the pixels into a Vec<u8>
	let raw_output_pixels: Vec<u8> = pixels.into_iter().flatten().collect();
	
	// creating the new image
	let new_img: ImageBuffer<Rgb<u8>, Vec<u8>> = ImageBuffer::from_raw(width, height, raw_output_pixels).unwrap();
	// creating the output filename
	let output_path = match filename.split_once(".") {
		Some((part1, part2)) => format!("{part1}_sorted.{part2}"),
		None => format!("{filename}_sorted"),
	};
	// saving
	new_img.save(output_path).expect("Couldn't save image");

}

// comparing pixels with v = r + g + b
fn comp_rgb(a: &[u8; 3], b: &[u8; 3]) -> Ordering {
	let v1 = a[0] as usize + a[1] as usize + a[2] as usize;
	let v2 = b[0] as usize + b[1] as usize + b[2] as usize;
	v1.cmp(&v2)
}
