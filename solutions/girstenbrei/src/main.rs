use core::f64;
use std::{collections::BTreeMap, fs::File, io::{BufRead, BufReader}};



struct Measurement {
    min: f64,
    mean: Vec<f64>,
    max: f64,
}

impl Measurement {
    fn add(&mut self, value: f64) {
        self.min = self.min.min(value);
        self.max = self.max.max(value);
        self.mean.push(value);
    }

    fn mean(&self) -> f64 {
        let total: f64 = self.mean.iter().sum();
        total / (self.mean.len() as f64)
    }
}

fn main() {

    let mut args = std::env::args();
    let _binname = args.next().expect("Failed reading command name");
    let filename = args.next().expect("Failed reading file name");

    let file = File::open(filename).expect("Failed opening input file");
    let buf_file = BufReader::new(file);


    let mut measurements: BTreeMap<String, Measurement> = BTreeMap::new();
    
    // INPUT
    for line in buf_file.lines() {
        let line = line.expect("Failed reading line");

        let mut split = line.split(';');
        let name = split.next().expect("Failed to find measurement name").to_string();
        let value = split.next().expect("Failed to find measurement value");

        let value_parsed: f64 = value.parse().expect("Failed parsing value");

        if let Some(val) = measurements.get_mut(&name) {
            val.add(value_parsed);
        } else {
            measurements.insert(name, Measurement{
                min: value_parsed,
                mean: vec![value_parsed],
                max: value_parsed,
            });
        }
    }

    // OUTPUT
    println!("{{");
    for (name, measurement) in measurements {
        println!("    {name}={:.01}/{:.01}/{:.01}", measurement.min, measurement.mean(), measurement.max);
    }
    println!("}}");
}
