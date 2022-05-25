use std::convert::TryInto;

fn vec_to_array<T, const N: usize>(v: Vec<T>) -> [T; N] {
    v.try_into()
        .unwrap_or_else(|v: Vec<T>| panic!("Expected a Vec of length {} but it was {}", N, v.len()))
}

#[test]
fn test_vec_to_array() {
  let array =  vec_to_array<5>(vec![1,2,3,4,7]);
}
