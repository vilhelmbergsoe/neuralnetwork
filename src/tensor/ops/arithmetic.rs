use crate::tensor::backward::{
    AddBackward,
    BinaryBackwardFn,
    MulBackward,
    PowBackward, // , MulBackward, PowBackward
    UnaryBackwardFn,
};
use crate::tensor::node::Node;
use crate::tensor::tensor::TensorRef;
use crate::tensor::Tensor;
use num_traits::Float;
use std::cell::RefCell;
use std::fmt;
use std::ops::{Add, Div, Mul, Sub};
use std::rc::Rc;

impl<T: Float + fmt::Debug> Mul for &TensorRef<T> {
    type Output = TensorRef<T>;

    fn mul(self, other: &TensorRef<T>) -> TensorRef<T> {
        let result_data = &self.borrow().data * &other.borrow().data;
        let mut result = Tensor::new(result_data);

        result.requires_grad = self.borrow().requires_grad || other.borrow().requires_grad;
        result.grad = None;

        if result.requires_grad {
            result.grad_fn = Some(Box::new(Node::Binary {
                tensors: (self.clone(), other.clone()),
                backward_fn: BinaryBackwardFn::Mul(MulBackward),
            }));
            result.is_leaf = false;
        }

        TensorRef::new(result)
    }
}

impl<T: Float + fmt::Debug> Add for &TensorRef<T> {
    type Output = TensorRef<T>;

    fn add(self, other: &TensorRef<T>) -> TensorRef<T> {
        let result_data = &self.borrow().data + &other.borrow().data;
        let mut result = Tensor::new(result_data);

        result.requires_grad = self.borrow().requires_grad || other.borrow().requires_grad;
        result.grad = None;

        if result.requires_grad {
            result.grad_fn = Some(Box::new(Node::Binary {
                tensors: (self.clone(), other.clone()),
                backward_fn: BinaryBackwardFn::Add(AddBackward),
            }));
            result.is_leaf = false;
        }

        TensorRef::new(result)
    }
}

impl<T: Float + fmt::Debug> TensorRef<T> {
    /// Element-wise power operation
    pub fn pow(&self, exponent: T) -> TensorRef<T> {
        let result_data = self.borrow().data.mapv(|val| val.powf(exponent));
        let mut result = Tensor::new(result_data);

        // Set requires_grad and grad_fn for the result tensor
        result.requires_grad = self.borrow().requires_grad;
        result.grad_fn = Some(Box::new(Node::Unary {
            tensor: self.clone(),
            backward_fn: UnaryBackwardFn::Pow(PowBackward(exponent)),
        }));
        result.is_leaf = false;

        TensorRef::new(result)
    }

    // /// Calculate the mean of all Tensor elements
    // pub fn mean(&self) -> Option<Tensor<T>> {
    //     if let Some(mean_data) = self.data.mean() {
    //         let mut result = Tensor::from(mean_data);

    //         result.requires_grad = self.requires_grad;

    //         if result.requires_grad {
    //             // Set grad_fn for result tensor
    //             result.grad_fn = Some(Box::new(Node {
    //                 saved_tensors: vec![Box::new(self)],
    //                 backward_fn: BackwardFn::Mean(mean),
    //             }));
    //             result.is_leaf = false;
    //         }

    //         Some(result)
    //     } else {
    //         None
    //     }
    // }

    // /// Calculate the sum of all Tensor elements
    // pub fn sum(&self, axis: usize) -> Tensor<T> {
    //     let sum_data = self.data.sum();
    //     let mut result = Tensor::from(sum_data);

    //     result.requires_grad = self.requires_grad;

    //     if result.requires_grad {
    //         // Set grad_fn for result tensor
    //         result.grad_fn = Some(Box::new(Node {
    //             saved_tensors: vec![Box::new(self)],
    //             backward_fn: BackwardFn::Sum(sum),
    //         }));
    //         result.is_leaf = false;
    //     }

    //     result
    // }
}
