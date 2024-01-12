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

    // Убедиться, что копилятор не позволит вернуть более одной мутабельной ссылки на один объект.
    let (_left, _right) = get_slice_and_number_n_return_two_slides(&slice, 2);
    //let (_left1, _left2) = get_slice_and_number_n_return_two_slides(&mut slice, 1);
}

//Принимает мутабельную ссылку на кортеж и bool значение.
//Если false, возвращает мутабельную ссылку на первый элемент кортежа.
// Если true, возвращает мутабельную ссылку на второй элемент кортежа.
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
    let (from_zero_to_n_minus_1, from_nth_to_last) = slice.split_at(n);
    (from_zero_to_n_minus_1, from_nth_to_last)
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
    for fragment in result.iter_mut().skip(0) {
        let slice_fragment = &slice[start..end];
        *fragment = slice_fragment;
        start = end;
        end += fragment_size;
    }
    result
}


#[test]
fn test_get_mutable_reference_tuple() {
    let mut tuple = (11, 48);
    let element = get_mutable_reference_tuple(&mut tuple, false);
    assert_eq!(*element, 11);
    let element = get_mutable_reference_tuple(&mut tuple, true);
    assert_eq!(*element, 48);
}

#[test]
fn test_get_mutable_reference_slice() {
    let mut slice = [908, 807, 706, 605];
    let element = get_mutable_reference_slice(&mut slice, 3);
    assert_eq!(*element, 605);
}

#[test]
fn test_get_slice_and_number_n () {
    let slice = [99, 77, 66, 55, 33, 22];
    let element = get_slice_and_number_n(&slice, 4);
    assert_eq!(*element, 77);
}


#[test]
fn test_get_slice_and_number_n_return_two_slides () {
    let slice = [1, 2, 3, 4, 5];
    let (from_zero_to_n_minus_1, from_nth_to_last) = get_slice_and_number_n_return_two_slides(&slice, 3);
    assert_eq!(from_zero_to_n_minus_1, &[1, 2, 3]);
    assert_eq!(from_nth_to_last, &[4, 5]);
}

#[test]
fn test_get_slice_and_return_four_equal_parts () {
    let slice = [11, 22, 33, 44, 55, 66, 77, 88, 99, 100, 200, 300];
    let result = get_slice_and_return_four_equal_parts(&slice);
    assert_eq!(result[0], &[11, 22, 33]);
    assert_eq!(result[1], &[44, 55, 66]);
    assert_eq!(result[2], &[77, 88, 99]);
    assert_eq!(result[3], &[100, 200, 300]);
}