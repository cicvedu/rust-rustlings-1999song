// primitive_types4.rs
//
// Get a slice out of Array a where the ??? is so that the test passes.
//
// Execute `rustlings hint primitive_types4` or use the `hint` watch subcommand
// for a hint.


#[test]
fn slice_out_of_array() {
    let a = [1, 2, 3, 4, 5];

    let nice_slice =&a[1..4];

    assert_eq!([2, 3, 4], nice_slice)
}
// fn main() {
//     let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

//     // 使用整个数组创建一个切片
//     let slice1: &[i32] = &array;
//     println!("Whole array: {:?}", slice1);

//     // 使用范围创建一个切片
//     let slice2: &[i32] = &array[2..5]; // 包含索引2, 3, 4的元素
//     println!("Slice 2: {:?}", slice2);

//     // 使用起始索引创建从开始到结尾的切片
//     let slice3: &[i32] = &array[3..]; // 包含索引3到结尾的元素
//     println!("Slice 3: {:?}", slice3);

//     // 使用结束索引创建从开头到结束的切片
//     let slice4: &[i32] = &array[..7]; // 包含索引0到6的元素
//     println!("Slice 4: {:?}", slice4);
// }