extern crate hyper;
extern crate stb_image;

use hyper::client::Request;
use hyper::Url;
use stb_image::image::LoadResult::{ImageU8,ImageF32,Error};
use stb_image::image;
use stb_image::image::{ImageFormat};

#[deriving(Show)]
struct FileStats {
    width: uint,
    height: uint,
    size: uint,
    format: ImageFormat
}

fn get_file_stats(url: &str) -> Result<FileStats, String> {
    let url = Url::parse(url).unwrap();

    let req = match Request::get(url) {
        Ok(e) => e,
        _ => return Err("Could not parse url".to_string())
    };

    let mut response = req.start().unwrap().send().unwrap();
    let buf = match response.read_to_end() {
        Err(_) => return Err("Could not read buffer".to_string()),
        Ok(a) => a
    };

    let image = match image::load_from_memory(buf.as_slice()) {
        ImageU8(z) => z,
        ImageF32(_) => {
            return Err("Weird image so we're bailing".to_string());
        },
        Error(s) => { return Err(s) }
    };

    let image_type = image::get_image_format(buf.as_slice());

    Ok(FileStats {
        size: buf.len(),
        width: image.width,
        height: image.height,
        format: image_type,
    })
}

fn main() {
    let url = "http://public.reverbnation.com/Album/12377/album_image/cropped/image.jpg".as_slice();
    let stats = match get_file_stats(url) {
        Ok(fs) => fs,
        Err(s) => {
            println!("Stupid url {} died because {}", url, s);
            return
        }
    };

    println!("File is {}", stats);
}
