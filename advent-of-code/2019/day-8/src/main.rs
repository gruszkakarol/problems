use std::fs::read_to_string;
use std::slice::Chunks;

const IMAGE_WIDTH: usize = 25;
const IMAGE_HEIGHT: usize = 6;
const IMAGE_SIZE: usize = IMAGE_HEIGHT * IMAGE_WIDTH;

type Pixels<'a> = &'a [u32];

#[derive(Debug)]
struct Layer<'a> {
    pixels: Pixels<'a>,
}

impl<'a> Layer<'a> {
    pub fn new(pixels: Pixels<'a>) -> Self {
        Self { pixels }
    }

    pub fn count(&self, number: u32) -> usize {
        self.pixels
            .iter()
            .filter(|p| p == &&number)
            .count()
    }
}

fn get_input() -> Vec<u32> {
    read_to_string("input.txt")
        .expect("You need to provide some input")
        .chars()
        .map(|c| c.to_digit(10).expect("Bad character in your input!"))
        .collect()
}

fn print_chunk(pixels_chunk: &[&u32]) {
    for pixel in pixels_chunk {
        let color = match pixel {
            0 => " ",
            1 => "#",
            _ => unreachable!()
        };
        print!("{}", color);
    }
    println!();
}

fn main() {
    let input = get_input();
    let layers: Vec<Layer> = input.chunks(IMAGE_SIZE).map(Layer::new).collect();

    println!("{:#?}", layers[0].count(0));
    let layer = &layers
        .iter()
        .fold(None, |first: Option<&Layer>, second| {
            return if let Some(first) = first {
                if first.count(0) < second.count(0) {
                    Some(first)
                } else {
                    Some(second)
                }
            } else {
                Some(second)
            };
        })
        .expect("There should be a layer with fewest number of  zeros!");
    println!("First part: {}", layer.count(1) * layer.count(2));

    let pixels: Vec<&u32> = (0..IMAGE_SIZE)
        .map(|i| layers.iter()
            .map(|l| l.pixels.get(i).unwrap())
            .find(|pixel| pixel != &&2).expect("Layer can't be fully transparent")).collect();

    pixels.chunks(IMAGE_WIDTH).for_each(print_chunk);
}

// TODO: Test it

