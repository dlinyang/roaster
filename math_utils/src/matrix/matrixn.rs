use std::ops::Add;

#[macro_export]
macro_rules! matrix_n {
    ($name:ident, $N: expr) => (
        pub struct $name<T> {
            pub data : [[T;$N];$N],
        }

        impl<T> $name<T> {
            pub fn new(data: [[T; $N];$N]) -> Self {
                $name {
                    data,
                }
            }
        }
    );
}


#[test]
fn matrix() {
    matrix_n!(Matrix2,2);
    Matrix2::<f32>::new([[1f32,1f32],[1f32,1f32]]);
}