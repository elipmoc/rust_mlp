trait PerceptronI {
    fn output(&self, input: f64) -> f64;
}

struct Perceptron<T: PerceptronI> {
    inputs: Vec<Input<T>>,
}

impl<T: PerceptronI> PerceptronI for Perceptron<T> {
    fn output(&self, input: f64) -> f64 {
        self.inputs
            .iter()
            .map(|x| x.get(input))
            .fold(0.0, |acc, x| acc + x)
    }
}

struct InputPerceptron {}

impl PerceptronI for InputPerceptron {
    fn output(&self, input: f64) -> f64 {
        input
    }
}

struct Input<T: PerceptronI> {
    w: f64,
    p: T,
}

impl<T: PerceptronI> Input<T> {
    fn get(&self, input: f64) -> f64 {
        self.p.output(input) * self.w
    }
}

fn main() {
    println!("Hello, world!");
}
