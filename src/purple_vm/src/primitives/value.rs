/*
  Copyright (C) 2018-2019 The Purple Core Developers.
  This file is part of the Purple Core Library.

  The Purple Core Library is free software: you can redistribute it and/or modify
  it under the terms of the GNU General Public License as published by
  the Free Software Foundation, either version 3 of the License, or
  (at your option) any later version.

  The Purple Core Library is distributed in the hope that it will be useful,
  but WITHOUT ANY WARRANTY; without even the implied warranty of
  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
  GNU General Public License for more details.

  You should have received a copy of the GNU General Public License
  along with the Purple Core Library. If not, see <http://www.gnu.org/licenses/>.
*/

use std::cmp::{Ordering, PartialOrd};
use std::fmt;
use std::ops::{Add, Div, Mul, Sub, SubAssign};

#[derive(Clone, Copy)]
pub enum VmValue {
    I32(i32),
    I64(i64),
    F32(f32),
    F64(f64),
    i32Array2([i32; 2]),
    i32Array4([i32; 4]),
    i32Array8([i32; 8]),
    i32Array16([i32; 16]),
    i32Array32([i32; 32]),
    i32Array64([i32; 64]),
    i32Array128([i32; 128]),
    i32Array256([i32; 256]),
    i64Array2([i64; 2]),
    i64Array4([i64; 4]),
    i64Array8([i64; 8]),
    i64Array16([i64; 16]),
    i64Array32([i64; 32]),
    i64Array64([i64; 64]),
    i64Array128([i64; 128]),
    i64Array256([i64; 256]),
    f32Array2([f32; 2]),
    f32Array4([f32; 4]),
    f32Array8([f32; 8]),
    f32Array16([f32; 16]),
    f32Array32([f32; 32]),
    f32Array64([f32; 64]),
    f32Array128([f32; 128]),
    f32Array256([f32; 256]),
    f64Array2([f64; 2]),
    f64Array4([f64; 4]),
    f64Array8([f64; 8]),
    f64Array16([f64; 16]),
    f64Array32([f64; 32]),
    f64Array64([f64; 64]),
    f64Array128([f64; 128]),
    f64Array256([f64; 256]),
}

impl VmValue {
    /// Returns the byte size of the inner value.
    pub fn byte_size(&self) -> usize {
        match *self {
            VmValue::I32(_) => 4,
            VmValue::I64(_) => 8,
            VmValue::F32(_) => 4,
            VmValue::F64(_) => 8,
            VmValue::i32Array2(_) => 8,
            VmValue::i32Array4(_) => 16,
            VmValue::i32Array8(_) => 32,
            VmValue::i64Array2(_) => 16,
            VmValue::i64Array4(_) => 32,
            VmValue::i64Array8(_) => 64,
            VmValue::f32Array2(_) => 8,
            VmValue::f32Array4(_) => 16,
            VmValue::f32Array8(_) => 32,
            VmValue::f64Array2(_) => 8,
            VmValue::f64Array4(_) => 16,
            VmValue::f64Array8(_) => 32,
            _ => panic!(),
        }
    }
}

impl PartialEq for VmValue {
    fn eq(&self, other: &VmValue) -> bool {
        match (*self, *other) {
            (VmValue::I32(val1), VmValue::I32(val2)) => val1 == val2,
            (VmValue::I64(val1), VmValue::I64(val2)) => val1 == val2,
            (VmValue::F32(val1), VmValue::F32(val2)) => val1 == val2,
            (VmValue::F64(val1), VmValue::F64(val2)) => val1 == val2,
            (VmValue::i32Array2(val1), VmValue::i32Array2(val2)) => val1 == val2,
            (VmValue::i32Array4(val1), VmValue::i32Array4(val2)) => val1 == val2,
            (VmValue::i32Array8(val1), VmValue::i32Array8(val2)) => val1 == val2,
            (VmValue::i64Array2(val1), VmValue::i64Array2(val2)) => val1 == val2,
            (VmValue::i64Array4(val1), VmValue::i64Array4(val2)) => val1 == val2,
            (VmValue::i64Array8(val1), VmValue::i64Array8(val2)) => val1 == val2,
            (VmValue::f32Array2(val1), VmValue::f32Array2(val2)) => val1 == val2,
            (VmValue::f32Array4(val1), VmValue::f32Array4(val2)) => val1 == val2,
            (VmValue::f32Array8(val1), VmValue::f32Array8(val2)) => val1 == val2,
            (VmValue::f64Array2(val1), VmValue::f64Array2(val2)) => val1 == val2,
            (VmValue::f64Array4(val1), VmValue::f64Array4(val2)) => val1 == val2,
            (VmValue::f64Array8(val1), VmValue::f64Array8(val2)) => val1 == val2,
            (_, _) => panic!("Cannot perform equality between different variants!"),
        }
    }
}

impl PartialOrd for VmValue {
    fn partial_cmp(&self, other: &VmValue) -> Option<Ordering> {
        match (self, other) {
            (VmValue::I32(val1), VmValue::I32(val2)) => {
                if val1 < val2 {
                    Some(Ordering::Less)
                } else if val1 > val2 {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (VmValue::I64(val1), VmValue::I64(val2)) => {
                if val1 < val2 {
                    Some(Ordering::Less)
                } else if val1 > val2 {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (VmValue::F32(val1), VmValue::F32(val2)) => {
                if val1 < val2 {
                    Some(Ordering::Less)
                } else if val1 > val2 {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (VmValue::F64(val1), VmValue::F64(val2)) => {
                if val1 < val2 {
                    Some(Ordering::Less)
                } else if val1 > val2 {
                    Some(Ordering::Greater)
                } else {
                    Some(Ordering::Equal)
                }
            }
            (VmValue::i32Array2(val1), VmValue::i32Array2(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::i32Array4(val1), VmValue::i32Array4(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::i32Array8(val1), VmValue::i32Array8(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::i64Array2(val1), VmValue::i64Array2(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::i64Array4(val1), VmValue::i64Array4(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::i64Array8(val1), VmValue::i64Array8(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::f32Array2(val1), VmValue::f32Array2(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::f32Array4(val1), VmValue::f32Array4(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::f32Array8(val1), VmValue::f32Array8(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::f64Array2(val1), VmValue::f64Array2(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::f64Array4(val1), VmValue::f64Array4(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (VmValue::f64Array8(val1), VmValue::f64Array8(val2)) => {
                panic!("Cannot perform comparison between arrays!")
            }
            (_, _) => panic!("Cannot perform compare between different variants!"),
        }
    }
}

// impl SubAssign for VmValue {
//     fn sub_assign(&mut self, other: VmValue) {
//         *self = match (self, other) {
//             (VmValue::I32(val1), VmValue::I32(val2)) => VmValue::I32(*val1 - val2),
//             (VmValue::I64(val1), VmValue::I64(val2)) => VmValue::I64(*val1 - val2),
//             (VmValue::F32(val1), VmValue::F32(val2)) => VmValue::F32(*val1 - val2),
//             (VmValue::F64(val1), VmValue::F64(val2)) => VmValue::F64(*val1 - val2),
//             (VmValue::i32Array2(val1), VmValue::i32Array2(val2)) => {
//                 let mut result: [i32; 2] = [0; 2];
//                 let src = val1.iter().zip(&val2).map(|(a, b)| a - b);

//                 for (r, v) in result.iter_mut().zip(src) {
//                     *r = v;
//                 }

//                 VmValue::i32Array2(result)
//             }
//             (VmValue::i32Array4(val1), VmValue::i32Array4(val2)) => {
//                 let mut result: [i32; 4] = [0; 4];
//                 let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

//                 for (r, v) in result.iter_mut().zip(src) {
//                     *r = v;
//                 }

//                 VmValue::i32Array4(result)
//             }
//             (VmValue::i32Array8(val1), VmValue::i32Array8(val2)) => {
//                 let mut result: [i32; 8] = [0; 8];
//                 let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

//                 for (r, v) in result.iter_mut().zip(src) {
//                     *r = v;
//                 }

//                 VmValue::i32Array8(result)
//             }
//             (VmValue::i64Array2(val1), VmValue::i64Array2(val2)) => {
//                 let mut result: [i64; 2] = [0; 2];
//                 let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

//                 for (r, v) in result.iter_mut().zip(src) {
//                     *r = v;
//                 }

//                 VmValue::i64Array2(result)
//             }
//             (VmValue::i64Array4(val1), VmValue::i64Array4(val2)) => {
//                 let mut result: [i64; 4] = [0; 4];
//                 let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

//                 for (r, v) in result.iter_mut().zip(src) {
//                     *r = v;
//                 }

//                 VmValue::i64Array4(result)
//             }
//             (VmValue::i64Array8(val1), VmValue::i64Array8(val2)) => {
//                 let mut result: [i64; 8] = [0; 8];
//                 let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

//                 for (r, v) in result.iter_mut().zip(src) {
//                     *r = v;
//                 }

//                 VmValue::i64Array8(result)
//             }
//             (VmValue::f32Array2(val1), VmValue::f32Array2(val2)) => {
//                 let mut result: [f32; 2] = [0.0; 2];
//                 let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

//                 for (r, v) in result.iter_mut().zip(src) {
//                     *r = v;
//                 }

//                 VmValue::f32Array2(result)
//             }
//             (VmValue::f32Array4(val1), VmValue::f32Array4(val2)) => {
//                 let mut result: [f32; 4] = [0.0; 4];
//                 let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

//                 for (r, v) in result.iter_mut().zip(src) {
//                     *r = v;
//                 }

//                 VmValue::f32Array4(result)
//             }
//             (VmValue::f32Array8(val1), VmValue::f32Array8(val2)) => {
//                 let mut result: [f32; 8] = [0.0; 8];
//                 let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

//                 for (r, v) in result.iter_mut().zip(src) {
//                     *r = v;
//                 }

//                 VmValue::f32Array8(result)
//             }
//             (VmValue::f64Array2(val1), VmValue::f64Array2(val2)) => {
//                 let mut result: [f64; 2] = [0.0; 2];
//                 let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

//                 for (r, v) in result.iter_mut().zip(src) {
//                     *r = v;
//                 }

//                 VmValue::f64Array2(result)
//             }
//             (VmValue::f64Array4(val1), VmValue::f64Array4(val2)) => {
//                 let mut result: [f64; 4] = [0.0; 4];
//                 let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

//                 for (r, v) in result.iter_mut().zip(src) {
//                     *r = v;
//                 }

//                 VmValue::f64Array4(result)
//             }
//             (VmValue::f64Array8(val1), VmValue::f64Array8(val2)) => {
//                 let mut result: [f64; 8] = [0.0; 8];
//                 let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

//                 for (r, v) in result.iter_mut().zip(src) {
//                     *r = v;
//                 }

//                 VmValue::f64Array8(result)
//             }
//             (_, _) => panic!("Cannot perform subassign between different variants!"),
//         }
//     }
// }

impl Add for VmValue {
    type Output = VmValue;

    // TODO: Possibly use native SIMD for arrays, but benchmark first
    fn add(self, other: VmValue) -> VmValue {
        match (self, other) {
            (VmValue::I32(val1), VmValue::I32(val2)) => VmValue::I32(val1 + val2),
            (VmValue::I64(val1), VmValue::I64(val2)) => VmValue::I64(val1 + val2),
            (VmValue::F32(val1), VmValue::F32(val2)) => VmValue::F32(val1 + val2),
            (VmValue::F64(val1), VmValue::F64(val2)) => VmValue::F64(val1 + val2),
            (VmValue::i32Array2(val1), VmValue::i32Array2(val2)) => {
                let mut result: [i32; 2] = [0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i32Array2(result)
            }
            (VmValue::i32Array4(val1), VmValue::i32Array4(val2)) => {
                let mut result: [i32; 4] = [0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i32Array4(result)
            }
            (VmValue::i32Array8(val1), VmValue::i32Array8(val2)) => {
                let mut result: [i32; 8] = [0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i32Array8(result)
            }
            (VmValue::i64Array2(val1), VmValue::i64Array2(val2)) => {
                let mut result: [i64; 2] = [0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i64Array2(result)
            }
            (VmValue::i64Array4(val1), VmValue::i64Array4(val2)) => {
                let mut result: [i64; 4] = [0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i64Array4(result)
            }
            (VmValue::i64Array8(val1), VmValue::i64Array8(val2)) => {
                let mut result: [i64; 8] = [0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i64Array8(result)
            }
            (VmValue::f32Array2(val1), VmValue::f32Array2(val2)) => {
                let mut result: [f32; 2] = [0.0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f32Array2(result)
            }
            (VmValue::f32Array4(val1), VmValue::f32Array4(val2)) => {
                let mut result: [f32; 4] = [0.0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f32Array4(result)
            }
            (VmValue::f32Array8(val1), VmValue::f32Array8(val2)) => {
                let mut result: [f32; 8] = [0.0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f32Array8(result)
            }
            (VmValue::f64Array2(val1), VmValue::f64Array2(val2)) => {
                let mut result: [f64; 2] = [0.0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f64Array2(result)
            }
            (VmValue::f64Array4(val1), VmValue::f64Array4(val2)) => {
                let mut result: [f64; 4] = [0.0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f64Array4(result)
            }
            (VmValue::f64Array8(val1), VmValue::f64Array8(val2)) => {
                let mut result: [f64; 8] = [0.0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| a + b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f64Array8(result)
            }
            (_, _) => panic!("Cannot perform addition between different variants!"),
        }
    }
}

impl Sub for VmValue {
    type Output = VmValue;

    fn sub(self, other: VmValue) -> VmValue {
        match (self, other) {
            (VmValue::I32(val1), VmValue::I32(val2)) => VmValue::I32(val1 - val2),
            (VmValue::I64(val1), VmValue::I64(val2)) => VmValue::I64(val1 - val2),
            (VmValue::F32(val1), VmValue::F32(val2)) => VmValue::F32(val1 - val2),
            (VmValue::F64(val1), VmValue::F64(val2)) => VmValue::F64(val1 - val2),
            (VmValue::i32Array2(val1), VmValue::i32Array2(val2)) => {
                let mut result: [i32; 2] = [0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| a - b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i32Array2(result)
            }
            (VmValue::i32Array4(val1), VmValue::i32Array4(val2)) => {
                let mut result: [i32; 4] = [0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| a - b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i32Array4(result)
            }
            (VmValue::i32Array8(val1), VmValue::i32Array8(val2)) => {
                let mut result: [i32; 8] = [0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| a - b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i32Array8(result)
            }
            (VmValue::i64Array2(val1), VmValue::i64Array2(val2)) => {
                let mut result: [i64; 2] = [0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| a - b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i64Array2(result)
            }
            (VmValue::i64Array4(val1), VmValue::i64Array4(val2)) => {
                let mut result: [i64; 4] = [0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| a - b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i64Array4(result)
            }
            (VmValue::i64Array8(val1), VmValue::i64Array8(val2)) => {
                let mut result: [i64; 8] = [0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| a - b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i64Array8(result)
            }
            (VmValue::f32Array2(val1), VmValue::f32Array2(val2)) => {
                let mut result: [f32; 2] = [0.0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| a - b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f32Array2(result)
            }
            (VmValue::f32Array4(val1), VmValue::f32Array4(val2)) => {
                let mut result: [f32; 4] = [0.0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| a - b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f32Array4(result)
            }
            (VmValue::f32Array8(val1), VmValue::f32Array8(val2)) => {
                let mut result: [f32; 8] = [0.0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| a - b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f32Array8(result)
            }
            (VmValue::f64Array2(val1), VmValue::f64Array2(val2)) => {
                let mut result: [f64; 2] = [0.0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| a - b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f64Array2(result)
            }
            (VmValue::f64Array4(val1), VmValue::f64Array4(val2)) => {
                let mut result: [f64; 4] = [0.0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| a - b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f64Array4(result)
            }
            (VmValue::f64Array8(val1), VmValue::f64Array8(val2)) => {
                let mut result: [f64; 8] = [0.0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| a - b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f64Array8(result)
            }
            (_, _) => panic!("Cannot perform substraction between different variants!"),
        }
    }
}

impl Mul for VmValue {
    type Output = VmValue;

    fn mul(self, other: VmValue) -> VmValue {
        match (self, other) {
            (VmValue::I32(val1), VmValue::I32(val2)) => VmValue::I32(val1 * val2),
            (VmValue::I64(val1), VmValue::I64(val2)) => VmValue::I64(val1 * val2),
            (VmValue::F32(val1), VmValue::F32(val2)) => VmValue::F32(val1 * val2),
            (VmValue::F64(val1), VmValue::F64(val2)) => VmValue::F64(val1 * val2),
            (VmValue::i32Array2(val1), VmValue::i32Array2(val2)) => {
                let mut result: [i32; 2] = [0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| a * b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i32Array2(result)
            }
            (VmValue::i32Array4(val1), VmValue::i32Array4(val2)) => {
                let mut result: [i32; 4] = [0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| a * b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i32Array4(result)
            }
            (VmValue::i32Array8(val1), VmValue::i32Array8(val2)) => {
                let mut result: [i32; 8] = [0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| a * b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i32Array8(result)
            }
            (VmValue::i64Array2(val1), VmValue::i64Array2(val2)) => {
                let mut result: [i64; 2] = [0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| a * b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i64Array2(result)
            }
            (VmValue::i64Array4(val1), VmValue::i64Array4(val2)) => {
                let mut result: [i64; 4] = [0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| a * b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i64Array4(result)
            }
            (VmValue::i64Array8(val1), VmValue::i64Array8(val2)) => {
                let mut result: [i64; 8] = [0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| a * b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i64Array8(result)
            }
            (VmValue::f32Array2(val1), VmValue::f32Array2(val2)) => {
                let mut result: [f32; 2] = [0.0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| a * b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f32Array2(result)
            }
            (VmValue::f32Array4(val1), VmValue::f32Array4(val2)) => {
                let mut result: [f32; 4] = [0.0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| a * b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f32Array4(result)
            }
            (VmValue::f32Array8(val1), VmValue::f32Array8(val2)) => {
                let mut result: [f32; 8] = [0.0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| a * b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f32Array8(result)
            }
            (VmValue::f64Array2(val1), VmValue::f64Array2(val2)) => {
                let mut result: [f64; 2] = [0.0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| a * b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f64Array2(result)
            }
            (VmValue::f64Array4(val1), VmValue::f64Array4(val2)) => {
                let mut result: [f64; 4] = [0.0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| a * b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f64Array4(result)
            }
            (VmValue::f64Array8(val1), VmValue::f64Array8(val2)) => {
                let mut result: [f64; 8] = [0.0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| a * b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f64Array8(result)
            }
            (_, _) => panic!("Cannot perform multiplication between different variants!"),
        }
    }
}

impl Div for VmValue {
    type Output = VmValue;

    fn div(self, other: VmValue) -> VmValue {
        match (self, other) {
            (VmValue::I32(val1), VmValue::I32(val2)) => VmValue::I32(val1 / val2),
            (VmValue::I64(val1), VmValue::I64(val2)) => VmValue::I64(val1 / val2),
            (VmValue::F32(val1), VmValue::F32(val2)) => VmValue::F32(val1 / val2),
            (VmValue::F64(val1), VmValue::F64(val2)) => VmValue::F64(val1 / val2),
            (VmValue::i32Array2(val1), VmValue::i32Array2(val2)) => {
                let mut result: [i32; 2] = [0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| a / b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i32Array2(result)
            }
            (VmValue::i32Array4(val1), VmValue::i32Array4(val2)) => {
                let mut result: [i32; 4] = [0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| a / b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i32Array4(result)
            }
            (VmValue::i32Array8(val1), VmValue::i32Array8(val2)) => {
                let mut result: [i32; 8] = [0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| a / b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i32Array8(result)
            }
            (VmValue::i64Array2(val1), VmValue::i64Array2(val2)) => {
                let mut result: [i64; 2] = [0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| a / b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i64Array2(result)
            }
            (VmValue::i64Array4(val1), VmValue::i64Array4(val2)) => {
                let mut result: [i64; 4] = [0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| a / b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i64Array4(result)
            }
            (VmValue::i64Array8(val1), VmValue::i64Array8(val2)) => {
                let mut result: [i64; 8] = [0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| a / b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::i64Array8(result)
            }
            (VmValue::f32Array2(val1), VmValue::f32Array2(val2)) => {
                let mut result: [f32; 2] = [0.0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| a / b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f32Array2(result)
            }
            (VmValue::f32Array4(val1), VmValue::f32Array4(val2)) => {
                let mut result: [f32; 4] = [0.0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| a / b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f32Array4(result)
            }
            (VmValue::f32Array8(val1), VmValue::f32Array8(val2)) => {
                let mut result: [f32; 8] = [0.0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| a / b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f32Array8(result)
            }
            (VmValue::f64Array2(val1), VmValue::f64Array2(val2)) => {
                let mut result: [f64; 2] = [0.0; 2];
                let src = val1.iter().zip(&val2).map(|(a, b)| a / b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f64Array2(result)
            }
            (VmValue::f64Array4(val1), VmValue::f64Array4(val2)) => {
                let mut result: [f64; 4] = [0.0; 4];
                let src = val1.iter().zip(&val2).map(|(a, b)| a / b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f64Array4(result)
            }
            (VmValue::f64Array8(val1), VmValue::f64Array8(val2)) => {
                let mut result: [f64; 8] = [0.0; 8];
                let src = val1.iter().zip(&val2).map(|(a, b)| a / b);

                for (r, v) in result.iter_mut().zip(src) {
                    *r = v;
                }

                VmValue::f64Array8(result)
            }
            (_, _) => panic!("Cannot perform division between different variants!"),
        }
    }
}

impl fmt::Debug for VmValue {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            VmValue::I32(val) => write!(f, "{}", val),
            VmValue::I64(val) => write!(f, "{}", val),
            VmValue::F32(val) => write!(f, "{}", val),
            VmValue::F64(val) => write!(f, "{}", val),
            VmValue::i32Array2(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i32Array4(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i32Array8(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i32Array16(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i32Array32(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i32Array64(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i32Array128(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i32Array256(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i64Array2(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i64Array4(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i64Array8(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i64Array16(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i64Array32(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i64Array64(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i64Array128(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::i64Array256(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f32Array2(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f32Array4(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f32Array8(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f32Array16(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f32Array32(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f32Array64(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f32Array128(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f32Array256(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f64Array2(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f64Array4(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f64Array8(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f64Array16(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f64Array32(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f64Array64(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f64Array128(val) => write!(f, "{:?}", val.to_vec()),
            VmValue::f64Array256(val) => write!(f, "{:?}", val.to_vec()),
        }
    }
}
