extern crate time;

use std::env;
use std::f32;
use std::ops::{Add, Sub, Mul, Div};
use std::fs::File;
use std::io::Read;

use time::PreciseTime;

#[derive(Copy, Clone, Debug)]
struct Measure {
    x: f32,
    y: f32,
}

impl Measure {
	pub fn new(x: f32, y: f32) -> Measure {
		Measure { x: x, y: y }
	}

	pub fn dist(&self, to: &Self) -> f32 {
		let a = self.x - to.x;
		let b = self.y - to.y;
		f32::sqrt(a * a + b * b)
	}
}

impl Add for Measure {
	type Output = Measure;

	fn add(self, rhs: Measure) -> Measure {
		Measure::new(self.x + rhs.x, self.y + rhs.y)
	}
}

impl Sub for Measure {
	type Output = Measure;

	fn sub(self, rhs: Measure) -> Measure {
		Measure::new(self.x - rhs.x, self.y - rhs.y)
	}
}

impl Mul<f32> for Measure {
    type Output = Measure;

    fn mul(self, rhs: f32) -> Measure {
        Measure::new(self.x * rhs, self.y * rhs)
    }
}

impl Div<f32> for Measure {
    type Output = Measure;

    fn div(self, rhs: f32) -> Measure {
        Measure::new(self.x / rhs, self.y / rhs)
    }
}

#[test]
fn test_measure() {
    let a = Measure::new(1.0, 3.0);
    let b = Measure::new(4.0, 5.0);
    let add = a + b;
    assert!(add.x == 5.0 && add.y == 8.0);
    let dif = a - b;
    assert!(dif.x == -3.0 && dif.y == -2.0);
    let mlt = a * 3.0;
    assert!(mlt.x == 3.0 && mlt.y == 9.0);
    let div = b / 2.0;
    assert!(div.x == 2.0 && div.y == 2.5);
}

#[derive(Clone, Debug)]
struct Class {
    mean: Measure,
    n: usize,
}

impl Class {
	pub fn new(m: &Measure) -> Class {
		Class { mean: m.clone(), n: 1 }
	}

	pub fn append(&mut self, m: &Measure) {
		self.mean = ( self.mean * self.n as f32 + *m ) / ( self.n + 1 ) as f32;
		self.n += 1;
	}

	pub fn merge(&mut self, m: &Class) {
		self.mean = (self.mean * self.n as f32 + m.mean * m.n as f32) / (self.n + m.n) as f32;
        self.n += m.n;
	}
}

#[test]
fn test_class() {
    let mut cls = Class::new(&Measure::new(1.0, 2.0));
    assert!(cls.mean.x == 1.0 && cls.mean.y == 2.0);
    assert!(cls.n == 1);
    cls.append(&Measure::new(3.0, 4.0));
    assert!(cls.mean.x == 2.0 && cls.mean.y == 3.0);
    assert!(cls.n == 2);
}

#[derive(Debug)]
struct Classifer {
    list: Vec<Box<Class>>,
    ncls_dist: f32,
}

impl Classifer {
	pub fn new(mdist: f32) -> Classifer {
		Classifer { ncls_dist: mdist, list: Vec::new() }
	}

	pub fn classificate(&mut self, m: &Measure) {
		if self.list.len() > 0 {
			let mut min_dist = std::f32::MAX;
			let mut near_cls = self.list[0].clone();

			for i in self.list.iter() {
				let d = m.dist(&i.mean);
				if d < min_dist {
					min_dist = d;
					near_cls = i.clone();
				}
			}

			if min_dist < self.ncls_dist {
				near_cls.append(m);
			} else {
				self.list.push(Box::new(Class::new(m)));
			}
		} else {
			self.list.push(Box::new(Class::new(m)));
		}
	}

	pub fn merge_classes(&mut self) {
		let mut uniq: Vec<Box<Class>> = Vec::new();

		for cls in self.list.iter() {
			let mut is_uniq = true;

			for trg in uniq.iter_mut() {
				if cls.mean.dist(&trg.mean) < self.ncls_dist {
                    trg.merge(cls);
                    is_uniq = false;
                }

				if !is_uniq {
					break;
				}
			}

			if is_uniq {
				uniq.push(cls.clone());
			}
		}

		self.list = uniq;
	}
}

fn read_measures_from_file(path: &str) -> Vec<Measure> {
	let mut res = Vec::new();

	let mut file = File::open(path).unwrap();
	let mut text = String::new();
	assert!(file.read_to_string(&mut text).is_ok());

	for line in text.lines() {
		let values: Vec<&str> = line.split_whitespace().take(2).collect();
		let x = values[0].parse::<f32>().unwrap();
		let y = values[1].parse::<f32>().unwrap();
		res.push(Measure::new(x, y));
	}

	return res;
}

fn main() {
	let mut args = env::args();
    let measures = read_measures_from_file( &args.nth(1).expect("Need argument: `input file`") );

	let start = PreciseTime::now();

	let mut clsf = Classifer::new(3.0);

	for i in measures.iter() {
		clsf.classificate(i);
	}

	if cfg!(feature = "merge") {
		clsf.merge_classes();
	}

	let duration = start.to(PreciseTime::now());
	println!("work time: {}", duration);

	for i in clsf.list.iter() {
		println!("[{}, {}]: {}", i.mean.x, i.mean.y, i.n);
	}
}
