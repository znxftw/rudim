use std::mem::size_of;

use crate::eval::nnue::{ACC_SIZE, INPUT_SIZE};

#[repr(C, align(64))]
#[derive(Clone, Debug)]
pub struct Network {
    pub transformer_weights: [i16; INPUT_SIZE * ACC_SIZE],
    pub transformer_biases: [i16; ACC_SIZE],
    pub output_weights: [i16; ACC_SIZE * 2],
    pub output_bias: i16,
}

static EMBEDDED_NETWORK: Network =
    unsafe { std::mem::transmute(*include_bytes!("../../../resources/nnue.bin")) };

impl Network {
    pub fn get_embedded() -> &'static Self {
        &EMBEDDED_NETWORK
    }

    // TODO: for tests, refactor
    pub fn new_boxed() -> Box<Self> {
        let bytes = vec![0u8; size_of::<Self>()];
        unsafe {
            let raw = Box::into_raw(bytes.into_boxed_slice()) as *mut Self;
            Box::from_raw(raw)
        }
    }
}
