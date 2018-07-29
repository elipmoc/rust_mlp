use std::collections::HashMap;

struct Model {
    w: HashMap<(usize, usize, usize, usize), f64>,
    layer_nums: Vec<usize>,
}
impl Model {
    pub fn new(layer_nums: Vec<usize>) -> Model {
        let mut w: HashMap<(usize, usize, usize, usize), f64> = HashMap::new();
        if layer_nums.len() < 3 {
            panic!("3層以上必要だろう！");
        }
        for i in 1..layer_nums.len() {
            for ii in 0..layer_nums[i] {
                for iii in 0..layer_nums[i - 1] {
                    w.insert((i - 1, iii, i, ii), 1.0);
                }
            }
        }
        Model {
            w: w,
            layer_nums: layer_nums,
        }
    }
}

fn main() {
    println!("Hello, world!");
}
