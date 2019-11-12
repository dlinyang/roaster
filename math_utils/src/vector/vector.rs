
use std::ops::{Add,Sub,Mul,Div,Neg,Index,IndexMut};

//generate a Rⁿ vector type
#[macro_export]
macro_rules! vector {
    ($name: ident, $dim:expr) => {
        #[derive(Debug,Copy,Clone,PartialEq)]
        pub struct $name<T: Default + Copy> {
            pub data: [T; $dim],
        }

        impl<T: Default + Copy > $name<T> {
            pub fn new(data: [T;$dim]) -> Self {
                $name {
                    data,
                }
            }

            pub fn broadcast(a: T) -> Self {
                $name {
                    data: [a;$dim],
                }
            }
        }

        pub fn dot<T>(a: $name<T>, b: $name<T>) -> T
            where T: Mul<Output = T> + Add<Output = T>  + Copy + Default {
                let mut result = Default::default();
                for i in 0..$dim {
                    result = result + a.data[i] * a.data[i];
                }
                result
            }

        impl<T: Add<Output = T> + Default + Copy> Add for $name<T> {
            type Output = Self;
            fn add(self,other: Self) -> Self {
                let mut result = [Default::default();$dim];

                for i in 0..$dim {
                    result[i] = self.data[i] + other.data[i];
                }

                Self {
                    data: result,
                }
            }
        }

        impl<T: Sub<Output = T> + Default +Copy> Sub for $name<T> {
            type Output = Self;
            fn sub(self,other: Self) -> Self {
                let mut result = [Default::default();$dim];

                for i in 0..$dim {
                    result[i] = self.data[i] - other.data[i];
                }

                Self {
                    data: result,
                }
            }
        }

        impl<T: Mul<Output = T> + Default + Copy> Mul for $name<T> {
            type Output = Self;
            fn mul(self, other: Self) -> Self {
                let mut result = [Default::default();$dim];

                for i in 0..$dim {
                    result[i] = self.data[i] * other.data[i];
                }

                Self {
                    data: result,
                }
            }
        }
    };
}


#[test]
fn vector_f32() {
    vector!(Vector3,3);
    type Vector3f = Vector3<f32>;
    let a = Vector3f::new([1.0,1.0,1.0]);
    let b = Vector3f::new([1.0,1.0,1.0]);
    assert_eq!(a + b,Vector3f::new([2.0,2.0,2.0]));
    assert_eq!(a - b,Vector3f::broadcast(0.0));
    assert_eq!(dot(a, b),3.0);
}
