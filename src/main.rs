#[warn(dead_code)]
#[warn(clippy::needless_range_loop)]

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

    let slice = [11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 200, 300];
    let _result = split_to_four(&slice);
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
fn split_to_four<T>(slice: &[T]) -> [&[T]; 4] {
    let len = slice.len();
    let _size = (len + 3) / 4; // округление вверх до ближайшего целого
    let mut result = [&[][..]; 4];

    for (i in result.iter_mut().enumerate() {
        let start = i * _size;
        let end = (i + 1) * _size;
        result[i] = &slice[start..end];
    }

    result
}

