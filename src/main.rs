struct Row {
    row_size: u16,
    chars: Vec<char>,
}

impl Row {
    fn from_content(&mut self, chars: &mut Vec<char>) {
        while self.chars.len() < self.row_size as usize {
            self.chars.push(chars.remove(0))
        }
    }

    fn count_chars(&self, value: char) -> u16 {
        self.chars.iter().filter(|c| **c == value).count() as u16
    }
}

struct Layer {
    row_size: u16,
    rows_in_layer: u16,
    rows: Vec<Row>,
}

impl Layer {
    fn from_content(&mut self, chars: &mut Vec<char>) {
        while self.rows.len() < self.rows_in_layer as usize {
            let mut row = Row {
                row_size: self.row_size,
                chars: Vec::new(),
            };
            row.from_content(chars);
            self.rows.push(row);
        }
    }

    fn count_chars(&self, value: char) -> u16 {
        self.rows.iter().map(|r| r.count_chars(value)).sum()
    }
}

struct Image {
    row_size: u16,
    rows_in_layer: u16,
    layers: Vec<Layer>,
}

impl Image {
    fn from_content(&mut self, chars: &mut Vec<char>) {
        while chars.len() > 0 {
            let mut layer = Layer {
                rows_in_layer: self.rows_in_layer,
                row_size: self.row_size,
                rows: Vec::new(),
            };
            layer.from_content(chars);
            self.layers.push(layer);
        }
    }
}

fn main() {
    let row_size = 25;
    let rows_in_layer = 6;

    let content = std::fs::read_to_string("inputs.txt").unwrap();

    let mut image = Image {
        row_size,
        rows_in_layer,
        layers: Vec::new(),
    };

    image.from_content(&mut content.chars().collect());

    println!("{} layers\n", image.layers.len());

    let mut fewest_zeroes = image.row_size * image.rows_in_layer;

    for layer in image.layers.iter() {
        let zeroes = layer.count_chars('0');
        if zeroes < fewest_zeroes {
            fewest_zeroes = zeroes;
            let ones = layer.count_chars('1');
            let twos = layer.count_chars('2');
            println!(
                "{} zeroes -> {} 1s x {} 2s = {}",
                zeroes,
                ones,
                twos,
                ones * twos
            );
        }
    }

    let mut output_layer = Layer {
        row_size,
        rows_in_layer,
        rows: Vec::new(),
    };

    for _yy in 0..rows_in_layer {
        let mut row = Row {
            row_size,
            chars: Vec::new(),
        };
        for _xx in 0..row_size {
            row.chars.push('2');
        }
        output_layer.rows.push(row);
    }

    for layer in image.layers.iter() {
        for yy in 0..rows_in_layer {
            for xx in 0..row_size {
                if output_layer.rows[yy as usize].chars[xx as usize] == '2' {
                    output_layer.rows[yy as usize].chars[xx as usize] =
                        layer.rows[yy as usize].chars[xx as usize];
                }
            }
        }
    }

    println!("");
    for row in output_layer.rows.iter() {
        println!(
            "{}",
            row.chars
                .iter()
                .map(|c| {
                    if *c == '1' {
                        '1'
                    } else {
                        ' '
                    }
                })
                .collect::<String>()
        );
    }
}
