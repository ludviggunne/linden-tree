use std::mem::swap;

#[derive(Debug)]
pub enum Symbol {
    Push,
    Pop,
    Turn(f64),
    Translate(f64),
    Generic(u32),
}

pub type Processor = fn(&Symbol, &mut Vec<Symbol>) -> bool;
pub struct System {
    processor: Processor,
    symbols: Vec<Symbol>,
    symbols_swap: Vec<Symbol>,
}

#[repr(C)]
#[derive(Debug)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl System {
    pub fn new(processor: Processor) -> System {
        System {
            processor: processor,
            symbols: Vec::new(),
            symbols_swap: Vec::new(),
        }
    }

    pub fn init(&mut self, symbols: Vec<Symbol>) {
        self.symbols = symbols;
    }

    pub fn step(&mut self) {
        for symbol in &self.symbols {
            (self.processor)(&symbol, &mut self.symbols_swap);
        }

        swap(&mut self.symbols, &mut self.symbols_swap);
    }

    pub fn gen_vbuf(&self) -> Vec<Point> {
        let mut state: (f64, f64, f64) = (0.0, 0.0, -std::f64::consts::PI / 2.0);
        let mut stack = Vec::new();
        let mut output = Vec::new();

        for symbol in &self.symbols {
            use Symbol::*;

            match symbol {
                Push => stack.push(state),
                Pop => state = stack.pop().unwrap(),
                Turn(a) => state.2 += a,
                Translate(x) => {
                    output.push(Point {
                        x: state.0 as f32,
                        y: state.1 as f32,
                    });
                    state.0 += x * state.2.cos();
                    state.1 += x * state.2.sin();
                    output.push(Point {
                        x: state.0 as f32,
                        y: state.1 as f32,
                    });
                }
                Generic(_) => (),
            }
        }

        output
    }
}
