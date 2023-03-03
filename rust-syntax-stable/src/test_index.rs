use std::any::type_name;
use std::ops::{Range, RangeFrom, RangeFull};

fn type_of<T>(_: T) -> &'static str {
    type_name::<T>()
    // std::intrinsics::type_name::<T>()
}

#[test]
fn test_vec() {
    /// 1. 第一行是创建一个从 0 到 10 的数字范围。
    ///     2. 第二行是从范围创建一个向量。
    ///     3. 第三行打印矢量图。
    ///     4. 第四行是从矢量创建切片。
    ///     5. 第五行打印切片。
    ///     6. 第六行是从向量中打印一个切片。
    ///     7. 第七行是从切片创建数组。
    ///     8. 第八行打印数组。
    // let vec=vec![1..10];
    let i: Range<i32> = 0..10;
    let mut vec: Vec<i32> = i.collect();
    // println!("{:?}",vec[0]);
    // println!("{}",type_of(vec[0]));
    println!("{:?}", vec);
    let vec_slice_reference = &mut vec[RangeFull];
    println!("{:?}", vec_slice_reference);
    let mut vv = &vec_slice_reference;
    let ds = &mut vv;
    // println!("{:?}",vv);
    println!("{:?}", ds);
    println!("{:?}", vv);
    let mut e = vec_slice_reference[0];
    e = 1;
    // println!("{:?}", vec);
    println!("{:?}", vec_slice_reference);
    vec_slice_reference[0] = 1;
    let range = ..;
    println!("{:?}", &vec[range]);
    let range = 0..4;
    println!("{:?}", &vec[range]);
    // let vec_slice=*vec_slice_reference;
    // println!("{:?}",vec_slice);
    let mut vec_slice_array = <&[i32] as TryInto<[i32; 3]>>::try_into(&vec[2..5]).unwrap();
    vec_slice_array[2] = 1;
    println!("{:?}", vec_slice_array);
    println!("{:?}", vec);
}

#[test]
fn test_index() {
    let vec: Vec<i32> = (0..10).collect();
    println!("{:?}", vec);
    let slice_borrow = &vec[2..5];
    let e = slice_borrow[2];
    println!("{}", slice_borrow[2]);
    let sbb = &slice_borrow;
    let e = sbb[1];
    println!("{}", sbb[1]);
}