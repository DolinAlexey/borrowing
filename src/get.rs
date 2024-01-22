pub mod functions {
    //Принимает мутабельную ссылку на кортеж и bool значение.
    //Если false, возвращает мутабельную ссылку на первый элемент кортежа.
    // Если true, возвращает мутабельную ссылку на второй элемент кортежа.
    pub fn get_mutable_reference_tuple<T>(tuple: &mut (T, T), bool_value: bool) -> &mut T {
        if bool_value {
            &mut tuple.1
        } else {
            &mut tuple.0
        }
    }



    //Принимает мутабельную ссылку на слайс и число N. Возвращает мутабельную ссылку на N-ый элемент.
    pub fn get_mutable_reference_slice<T>(slice: &mut [T], n: usize) -> &mut T {
        &mut slice[n]
    }

    //Принимает слайс и число N. Возвращает ссылку на N-ый элемент слайса с конца.
    pub fn get_slice_and_number_n<T>(slice: &[T], n: usize) -> &T {
        &slice[slice.len() - 1 - n]
    }

    //Принимает слайс и число N. Возвращает два слайса с элементами:
// с нулевого по N-1;
// с N-го по последний;
    pub fn get_slice_and_number_n_return_two_slides<T>(slice: &[T], n: usize) -> (&[T], &[T]) {
        let (from_zero_to_n_minus_1, from_nth_to_last) = slice.split_at(n);
        (from_zero_to_n_minus_1, from_nth_to_last)
    }

    //Принимает слайс и возвращает массив слайсов, содержащий четыре равные (насколько возможно) части исходного слайса.
// Протестировать функции.
// Убедиться, что копилятор не позволит вернуть более одной мутабельной ссылки на один объект.
    pub fn get_slice_and_return_four_equal_parts<T>(slice: &[T]) -> [&[T]; 4] {
        let len = slice.len();
        let fragment_size = len / 4;
        let mut result = [&[][..]; 4];

        let mut start = 0;
        for (i, fragment) in result.iter_mut().enumerate() {
            let remaining = len - start;
            let size = if i < 3 { fragment_size } else { remaining };
            let end = start + size;
            let slice_fragment = &slice[start..end];
            *fragment = slice_fragment;
            start = end;
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
    fn test_get_slice_and_number_n() {
        let slice = [99, 77, 66, 55, 33, 22];
        let element = get_slice_and_number_n(&slice, 4);
        assert_eq!(*element, 77);
    }

    #[test]
    fn test_get_slice_and_number_n_return_two_slides() {
        let slice = [1, 2, 3, 4, 5];
        let (from_zero_to_n_minus_1, from_nth_to_last) =
            get_slice_and_number_n_return_two_slides(&slice, 3);
        assert_eq!(from_zero_to_n_minus_1, &[1, 2, 3]);
        assert_eq!(from_nth_to_last, &[4, 5]);
    }

    #[test]
    fn test_get_slice_and_return_four_equal_parts() {
        let slice = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
        let _result = get_slice_and_return_four_equal_parts(&slice);
        assert_eq!(_result[0], &[1, 2]);
        assert_eq!(_result[1], &[3, 4]);
        assert_eq!(_result[2], &[5, 6]);
        assert_eq!(_result[3], &[7, 8, 9, 10, 11]);
    }
}