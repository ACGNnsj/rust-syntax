#[cfg(test)]
mod test_index;
#[cfg(test)]
mod test_borrow;
#[cfg(test)]
mod test_match;
#[cfg(test)]
mod test_num;
#[cfg(test)]
mod test_vec;
#[cfg(test)]
mod test_loop;
#[cfg(test)]
mod test_type;
#[cfg(test)]
mod test_trait_object;
#[cfg(test)]
mod test_str;
#[cfg(test)]
mod test_iterator;
#[cfg(test)]
mod test_smart_pointer;
#[cfg(test)]
mod test_lifetime;
#[cfg(test)]
mod test_option;
#[cfg(test)]
pub mod test_trait;
#[cfg(test)]
mod test_enum;
#[cfg(test)]
mod test_ref;
#[cfg(test)]
mod test_union;
#[cfg(test)]
mod test_drop;
#[cfg(test)]
mod test_operator;
#[cfg(test)]
mod test_ufcs;
#[cfg(test)]
mod test_raw_pointer;
#[cfg(test)]
mod test_tuple_struct;


mod test_multi_thread;
mod test_result;
mod test_format;
mod test_cpuid;
mod test_asm;
mod test_orphan_rule;
mod test_rdrand;
mod test_calling_convention;
pub mod test_path;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

use std::ffi::{OsStr, OsString};
use std::ops;

pub struct TestBuf {
    inner: OsString,
}

impl TestBuf {
    pub fn new() -> TestBuf {
        TestBuf { inner: OsString::new() }
    }
}

impl AsRef<OsStr> for TestBuf {
    #[inline]
    fn as_ref(&self) -> &OsStr {
        &self.inner[..]
    }
}

pub fn show<P: AsRef<OsStr>>(path: P) {
    println!("{:?}", path.as_ref());
}

pub struct MyWtf8Buf {
    bytes: Vec<u8>,
    is_known_utf8: bool,
}

impl MyWtf8Buf {
    pub fn new() -> MyWtf8Buf {
        MyWtf8Buf {
            bytes: Vec::new(),
            is_known_utf8: true,
        }
    }
    pub fn with_capacity(capacity: usize) -> MyWtf8Buf {
        MyWtf8Buf {
            bytes: Vec::with_capacity(capacity),
            is_known_utf8: true,
        }
    }
}

pub struct MyBuf {
    pub inner: MyWtf8Buf,
}

impl MyBuf {
    pub fn new() -> MyBuf {
        MyBuf { inner: MyWtf8Buf::new() }
    }
}

pub struct TestString {
    inner: MyBuf,
}

impl TestString {
    pub fn new() -> TestString {
        TestString { inner: MyBuf::new() }
    }
}

/*impl ops::Index<ops::RangeFull> for TestString {
    type Output = OsStr;

    #[inline]
    fn index(&self, _index: ops::RangeFull) -> &OsStr {
        OsStr::from_inner(self.inner.as_slice())
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_index() {
        let string = TestString::new();
        // let x=string[..];
    }

    #[test]
    fn test_buf() {
        let test_buf = TestBuf::new();
        // println!("{}",test_buf);
        // println!("{:?}",test_buf);
        // println!("{:#?}",test_buf);
    }

    #[test]
    fn test_as_ref() {
        let test_buf = TestBuf::new();
        show(test_buf);
    }

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn immutable() {
        let imt = 1;
        let copy = imt;
        let reference = &imt;
        let reference2 = &imt;
        // let borrowed = &mut copy;
    }

    #[test]
    fn mutable() {
        let mut m = 1;
        m = 2;
        let reference = &m;
        let borrowed = &mut m;
        *borrowed = 20;
        println!("{}", borrowed);
        println!("{}", m);
        let mut mut_borrowed = &mut m;
        *mut_borrowed = 3;
        println!("{}", mut_borrowed);
        println!("{}", m);
        let mut m2 = 22;
        mut_borrowed = &mut m2;
        println!("{}", mut_borrowed);
        println!("{}", m);
        let mut mut_borrowed2 = &mut m;
        *mut_borrowed2 = 4;
    }
}
