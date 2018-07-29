use std::collections::HashMap;
type Output = HashMap<(usize, usize), f64>;

struct Model {
    //重み係数
    w: HashMap<(usize, usize, usize, usize), f64>,
    //層のパーセプトロン個数
    layer_nums: Vec<usize>,
    //学習係数
    k: f64,
}
impl Model {
    pub fn new(layer_nums: Vec<usize>, k: f64) -> Model {
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
            k: k,
        }
    }
    pub fn input(&self, inputs: Vec<f64>) -> Output {
        let mut a: HashMap<(usize, usize), f64> = HashMap::new();
        if self.layer_nums[0] != inputs.len() {
            panic!("入力個数が不正だよ！");
        }
        for (i, item) in inputs.iter().enumerate() {
            a.insert((0, i), *item);
        }
        for i in 1..self.layer_nums.len() {
            for ii in 0..self.layer_nums[i] {
                let tmp = (0..self.layer_nums[i - 1]).fold(0.0, |acc: f64, x| {
                    acc + a.get(&(i - 1, x)).unwrap() * self.w.get(&(i - 1, x, i, ii)).unwrap()
                });
                a.insert((i, ii), 1.0 / (1.0 + (-tmp).exp()));
            }
        }
        a
    }
    //重みの修正
    pub fn fix(&mut self, output: Output, t: Vec<f64>) {
        let len = self.layer_nums.len();
        if self.layer_nums[len - 1] != t.len() {
            panic!("教師の数が不正です");
        }
        for i in 1..len {
            for ii in 0..self.layer_nums[i] {
                for iii in 0..self.layer_nums[i - 1] {
                    let tmp = (0..self.layer_nums[len - 1]).fold(0.0, |acc, x| {
                        acc + output.get(&(len - 1, x)).unwrap() - t[x]
                    });
                    let tmp2 = output.get(&(i, ii)).unwrap();
                    let tmp2 = (1.0 - tmp2) * tmp2 * output.get(&(i - 1, iii)).unwrap();
                    let fix = (i + 1..len).fold(1.0, |acc, x| {
                        (0..self.layer_nums[x]).fold(0.0, |acc, xx| {
                            let z = output.get(&(x, xx)).unwrap();
                            acc + (1.0 - z) * z * self.w.get(&(i, ii, x, xx)).unwrap()
                        }) * acc
                    }) * tmp * tmp2;
                    let w = *self.w.get(&(i - 1, iii, i, ii)).unwrap();
                    self.w.insert((i - 1, iii, i, ii), w - fix * self.k);
                }
            }
        }
    }
}

fn main() {
    let mut model = Model::new(vec![3, 3, 1], 0.3);
    let data = vec![
        vec![1.0, 4.0, 2.0],
        vec![1.0, 1.0, 1.0],
        vec![1.0, 5.0, 6.0],
        vec![1.0, 10.0, 0.0],
    ];
    for _ in 0..100000 {
        for x in data.iter() {
            let input = model.input(x.clone());
            model.fix(input, vec![(1.0 / (x[0] + x[1])) as f64]);
        }
    }
    for x in data.iter() {
        let input = model.input(x.clone());
        println!(
            "1/({}+{})={} true: {}",
            x[0],
            x[1],
            input.get(&(2, 0)).unwrap(),
            1.0 / (x[0] + x[1])
        );
    }
}
