use borrowing::get::functions::*;

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

    let slice = [11, 22, 33, 44, 55, 66, 77, 88, 99, 100];
    let _result = get_slice_and_return_four_equal_parts(&slice);

    let slice = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14];
    let _result = get_slice_and_return_four_equal_parts(&slice);

    // Убедиться, что копилятор не позволит вернуть более одной мутабельной ссылки на один объект.
    let (_left, _right) = get_slice_and_number_n_return_two_slides(&slice, 2);
    //let (_left1, _left2) = get_slice_and_number_n_return_two_slides(&mut slice, 1);
}
