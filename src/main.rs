extern crate hyper;
extern crate stb_image;

use hyper::client::Request;
use hyper::Url;
use stb_image::image::LoadResult::{ImageU8,ImageF32,Error};
use stb_image::image;
use stb_image::image::{ImageFormat};
use std::sync::{Arc, Mutex, Barrier};
use std::vec::Vec;

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
    let list:Arc<Mutex<Vec<FileStats>>> = Arc::new(Mutex::new(Vec::new()));
    let barrier = Arc::new(Barrier::new(11));

    for _ in range(0u, 10u) {
        let barrier_clone = barrier.clone();
        let list_clone = list.clone();
        spawn(proc() {
            let url = "http://public.reverbnation.com/Album/12377/album_image/cropped/image.jpg".as_slice();
            match get_file_stats(url) {
                Ok(fs) => {
                    list_clone.lock().push(fs);
                },
                Err(s) => {
                    println!("Stupid url {} died because {}", url, s);
                    return
                }
            };
            barrier_clone.wait();
        })
    }

    barrier.wait();

    for thing in list.lock().iter() {
        println!("List: {}", thing);
    }
}
