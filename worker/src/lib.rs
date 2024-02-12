use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use image::{Rgb, RgbImage};

pub fn validate_worker_argument(arguments: Vec<String>) -> Result<SocketAddr, Box<dyn std::error::Error>> {
    match arguments.len() {
        0 | 1 => panic!("Needs one argument: IP address of the server"),
        2 => (),
        _ => panic!("Too many arguments, only one is needed: IP address of the server")
    }
    let Ok(server_address) = arguments[1].parse::<SocketAddr>()
        else {
            panic!("Invalid IP address: {}\nWrite it using the following format 127.0.0.1:8000", arguments[1]);
        };
    Ok(server_address)
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Complex {
    re: f64,
    im: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Range {
    min: Point,
    max: Point,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Resolution {
    nx: u16,
    ny: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct U8Data {
    offset: u32,
    count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct JuliaDescriptor {
    c: Complex,
    divergence_threshold_square: f64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
enum FractalDescriptor {
    Julia(JuliaDescriptor),
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct FragmentTask {
    id: U8Data,
    fractal: FractalDescriptor,
    max_iteration: u32,
    resolution: Resolution,
    range: Range,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct PixelData {
    offset: u32,
    count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct FragmentResult {
    id: U8Data,
    resolution: Resolution,
    range: Range,
    pixels: PixelData,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct FragmentRequest {
    worker_name: String,
    maximal_work_load: u32,
}
impl FragmentRequest {
    fn process_request(&self) -> FragmentTask {
        let fractal_descriptor = FractalDescriptor::Julia(JuliaDescriptor {
            c: Complex { re: -0.9, im: 0.27015 },
            divergence_threshold_square: 5.0,
        });

        let fragment_task = FragmentTask {
            id: U8Data { offset: 0, count: 8 },
            fractal: fractal_descriptor,
            max_iteration: 1000,
            resolution: Resolution { nx: 1600, ny: 1200 },
            range: Range {
                min: Point { x: -1.5, y: -1.0 },
                max: Point { x: 1.5, y: 1.0 },
            },
        };

        fragment_task
    }
}

impl FractalDescriptor {
    fn get_c(&self) -> Complex {
        match self {
            FractalDescriptor::Julia(julia_desc) => julia_desc.c.clone(),
        }
    }

    fn get_divergence_threshold_square(&self) -> f64 {
        match self {
            FractalDescriptor::Julia(julia_desc) => julia_desc.divergence_threshold_square,
        }
    }
}

impl FragmentResult {
    fn save_as_image(&self, fragment_task: &FragmentTask) {
        let mut image = RgbImage::new(self.resolution.nx.into(), self.resolution.ny.into());

        for y in 0..self.resolution.ny {
            for x in 0..self.resolution.nx {
                // Calculate Julia set color value for each pixel
                let real = self.range.min.x + (x as f64 / (self.resolution.nx - 1) as f64)
                    * (self.range.max.x - self.range.min.x);
                let imag = self.range.min.y + (y as f64 / (self.resolution.ny - 1) as f64)
                    * (self.range.max.y - self.range.min.y);

                let mut z = Complex { re: real, im: imag };
                let c = fragment_task.fractal.get_c(); // Use fragment_task to get the fractal

                let mut iteration = 0;
                while iteration < fragment_task.max_iteration
                    && z.re * z.re + z.im * z.im < fragment_task.fractal.get_divergence_threshold_square()
                {
                    let new_re = z.re * z.re - z.im * z.im + c.re;
                    let new_im = 2.0 * z.re * z.im + c.im;

                    z.re = new_re;
                    z.im = new_im;

                    iteration += 1;
                }

                // temp coloration
                let color = Rgb([
                    (iteration % 256) as u8,
                    ((iteration / 256) % 256) as u8,
                    ((iteration / 256 / 256) % 256) as u8,
                ]);

                image.put_pixel(x as u32, y as u32, color);
            }
        }

        image.save("fragment.png").unwrap();
    }

    fn process_result(self, fragment_task: &FragmentTask) {
        println!("Fragment Result ID: {:?}", self.id);
        println!("Resolution: {:?}", self.resolution);
        println!("Range: {:?}", self.range);
        println!("Pixel Data: {:?}", self.pixels);

        self.save_as_image(fragment_task);
    }
}

// How I use this code
/*fn main() {
    // request simulation
    let request = FragmentRequest {
        worker_name: String::from("Worker1"),
        maximal_work_load: 1000,
    };

    let fragment_task = request.process_request();

    let fragment_result = FragmentResult {
        id: fragment_task.id.clone(),
        resolution: fragment_task.resolution.clone(),
        range: fragment_task.range.clone(),
        pixels: PixelData { offset: 0, count: 0 },
    };

    fragment_result.process_result(&fragment_task);
}*/
