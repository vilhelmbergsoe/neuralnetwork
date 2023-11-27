use ndarray::{arr1, concatenate, Array1, Axis};

mod nn;
use nn::{NN, Linear, relu, sigmoid, tanh};

struct XORNet {
    fl1: Linear,
    fl2: Linear,

    bl1: Linear,
    bl2: Linear,

    hl1: Linear,
}

impl XORNet {}

impl NN for XORNet {
    fn new() -> Self {
        Self {
            fl1: Linear::new(2, 1),
            fl2: Linear::new(1, 2),

            bl1: Linear::new(2, 1),
            bl2: Linear::new(1, 2),

            hl1: Linear::new(4, 2),
        }
    }

    // TODO: Implement pipeline:
    // nn.Sequential(
    //     nn.Sequential(
    //         nn.Linear(2, 1, activation_func="relu"),
    //         nn.Linear(1, 2, activation_func="relu")
    //     ),
    //     nn.Sequential(
    //         nn.Linear(2, 1, activation_func="sigmoid"),
    //         nn.Linear(1,2, activation_func="sigmoid")
    //     ),
    //     nn.Linear(4, 1, activation_func="tanh")
    // )
    fn forward(&self, x: Array1<f32>) -> Array1<f32> {
        let flx = relu(self.fl1.calc(&x));
        let blx = relu(self.bl1.calc(&x));
        let flx = sigmoid(self.fl2.calc(&flx));
        let blx = sigmoid(self.bl2.calc(&blx));

        let merged_arr = concatenate(Axis(0), &[flx.view(), blx.view()]).unwrap();

        let x = tanh(self.hl1.calc(&merged_arr));

        return x;
    }
}

fn main() {
    let nn = XORNet::new();

    let x = nn.forward(arr1(&[1., 0.]));

    println!("{}", x);
}
