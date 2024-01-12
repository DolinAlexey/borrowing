#[warn(dead_code)]
#[warn(clippy::needless_range_loop)]


fn main() {
    let mut tuple: (u8, u8) = (11, 48);
    let _first_element = get_mutable_reference_tuple(&mut tuple, false);
    let _second_element = get_mutable_reference_tuple(&mut tuple, true);

    let mut slice = [1, 2, 3, 4];
    let _third_element = get_mutable_reference_slice(&mut slice, 2);

    let slice = [11, 22, 33, 44];
    let _third_element_from_end = get_slice_and_number_n(&slice, 2);

    let slice = [11, 22, 33, 44, 55];
    let (_left, _right) = get_slice_and_number_n_return_two_slides(&slice, 3);

    let slice = [11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 200, 300];
    let _result = get_slice_and_return_four_equal_parts(&slice);
    let (_left, _right) = get_slice_and_number_n_return_two_slides(&slice, 2);
    //let (_left1, _left2) = get_slice_and_number_n_return_two_slides(&mut slice, 1); // Убедиться, что копилятор не позволит вернуть более одной мутабельной ссылки на один объект.
}

//Принимает мутабельную ссылку на кортеж и bool значение.
fn get_mutable_reference_tuple<T>(tuple: &mut (T, T), bool_value: bool) -> &mut T {
    if bool_value {
        &mut tuple.1
    } else {
        &mut tuple.0
    }
}

//Принимает мутабельную ссылку на слайс и число N. Возвращает мутабельную ссылку на N-ый элемент.
fn get_mutable_reference_slice<T>(slice: &mut [T], n: usize) -> &mut T {
    &mut slice[n]
}

//Принимает слайс и число N. Возвращает ссылку на N-ый элемент слайса с конца.
fn get_slice_and_number_n<T>(slice: &[T], n: usize) -> &T {
    &slice[slice.len() - 1 - n]
}

//Принимает слайс и число N. Возвращает два слайса с элементами:
// с нулевого по N-1;
// с N-го по последний;
fn get_slice_and_number_n_return_two_slides<T>(slice: &[T], n: usize) -> (&[T], &[T]) {
    let (from_zero_to_n_minus_1, fron_nth_to_last) = slice.split_at(n);
    (from_zero_to_n_minus_1, fron_nth_to_last)
}

//Принимает слайс и возвращает массив слайсов, содержащий четыре равные (насколько возможно) части исходного слайса.
// Протестировать функции.
// Убедиться, что копилятор не позволит вернуть более одной мутабельной ссылки на один объект.
fn get_slice_and_return_four_equal_parts<T>(slice: &[T]) -> [&[T]; 4] {
    let len = slice.len();
    let _size = (len + 3) / 4;
    let mut result = [&[][..]; 4];

    let mut start = 0;
    let fragment_size = slice.len() / result.len();
    let mut end = fragment_size;
    for fragment in result.iter_mut().skip(1) {
        let slice_fragment = &slice[start..end];
        *fragment = slice_fragment;
        start = end;
        end += fragment_size;
    }
    result
}
