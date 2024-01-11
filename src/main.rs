#[warn(dead_code)]

fn main() {
    let mut tuple: (u8, u8) = (11, 48);
    let _first_element = get_mut_tuple_element(&mut tuple, false);
    let _second_element = get_mut_tuple_element(&mut tuple, true);

    let mut slice = [1, 2, 3, 4];
    let _third_element = get_mut_slice_element(&mut slice, 2);

    let slice = [11, 22, 33, 44];
    let _third_element_from_end = get_slice_element_from_end(&slice, 2);

    let slice = [11, 22, 33, 44, 55];
    let (_left, _right) = split_slice(&slice, 3);
}
fn get_mut_tuple_element<T>(tuple: &mut (T, T), second: bool) -> &mut T {
    if second {
        &mut tuple.1
    } else {
        &mut tuple.0
    }
}
fn get_mut_slice_element<T>(slice: &mut [T], index: usize) -> &mut T {
    &mut slice[index]
}

fn get_slice_element_from_end<T>(slice: &[T], index_from_end: usize) -> &T {
    &slice[slice.len() - 1 - index_from_end]
}
fn split_slice<T>(slice: &[T], n: usize) -> (&[T], &[T]) {
    let (left, right) = slice.split_at(n);
    (left, right)
}
