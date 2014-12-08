extern crate hyper;
extern crate stb_image;

use hyper::client::Request;
use hyper::Url;
use stb_image::image::LoadResult::{ImageU8,ImageF32,Error};
use stb_image::image;

fn main() {
    let url = Url::parse("http://public.reverbnation.com/Album/12377/album_image/cropped/image.jpg").unwrap();
    // let url = Url::parse("http://4.bp.blogspot.com/-IJXe-eEJTmA/U-G6WnOSe_I/AAAAAAAAAtQ/v_V9BcQu9Ok/s33-c/336914de2a.png").unwrap();

    let req = match Request::get(url) {
        Ok(e) => e,
        _ => return
    };

    let mut response = req.start().unwrap().send().unwrap();
    let buf = match response.read_to_end() {
        Err(_) => return,
        Ok(a) => a
    };

    println!("File size was {}", buf.len());


    let image = match image::load_from_memory(buf.as_slice()) {
        ImageU8(z) => z,
        ImageF32(_) => {
            println!("F32!");
            return;
        },
        Error(s) => { println!("Bailed because {}", s); return }
    };

    let image_type = image::get_image_format(buf.as_slice());
    println!("Image type is {}", image_type);

    println!("File is {} long", buf.len());
    println!("Image is {}x{}", image.width, image.height);
}
