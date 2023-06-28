use std::ops::{Deref, Index};
use std::sync::Arc;

fn arc_only(arc: Arc<Vec<i32>>) {
    println!("arc: {:?}", arc);
}

fn no_arc(vec: Vec<i32>) {
    println!("vec: {:?}", vec);
}

fn slice_only(slice: &[i32]) {
    println!("slice: {:?}", slice);
}

fn ref_only(r: &Vec<i32>) {
    println!("ref: {:?}", r);
}


#[test]
fn test_arc() {
    let arc = Arc::new(vec![1, 2, 3]);
    arc_only(arc.clone());
    no_arc((*arc).clone());
    let s: &Arc<Vec<i32>> = &arc;
    let d = &**s;
    // let ai:&[i32]=(&arc)[..];
    let fs = arc[2];
    let fd = *arc.index(2);
    let ii = ..;
    let si = (&arc).index(..);
    let i = Index::index(&**s, ..);
    let sl: &[i32] = s;
    // let sl: &[i32] = arc;
    let sli: &[i32] = &vec![1, 2, 3];
    slice_only(&s[..]);
    slice_only(s);
    slice_only(&&&&&arc[..]);
    let slice = &arc[..];
    slice_only(slice);
    let v: &Vec<i32> = &&&&vec![1, 2, 3];
    v.into_iter();
    ref_only(&&&&vec![1, 2, 3]);
    let dr = (*s).clone();
    slice_only(s);
}

#[test]
fn showcase() {
    let arc = &Arc::new(vec![1, 2, 3]);
    let mut ad: &Vec<i32> = arc;
    let ad = arc.deref();
    let add = ad.deref();
    let mut slice: &[i32] = arc;
    slice = arc.index(..);
    slice = &arc[..];
    println!("slice: {:?}", slice);
    // let ir: &i32 = &2;
    // let i: i32 = ir;
    // trans(ir);
}

fn trans(i: i32) {
    println!("i: {}", i);
}

struct RawDeref {
    name: &'static str,
}

impl RawDeref {
    fn new(name: &'static str) -> Self {
        Self { name }
    }
    fn name(self) -> Self {
        println!("name: {}", self.name);
        self
    }
}

fn raw_name(raw: RawDeref) {
    raw.name();
}

#[test]
fn test_deref() {
    let mut raw = RawDeref::new("raw");
    let rr = &raw;
    // raw = rr.name();
    // raw_name(rr);
    // let r:RawDeref= rr;
}

#[derive(Clone)]
struct CustomDeref {
    name: &'static str,
}

impl CustomDeref {
    fn new(name: &'static str) -> Self {
        Self { name }
    }
    fn name(self) {
        println!("name: {}", self.name);
    }
}

impl Deref for CustomDeref {
    type Target = &'static str;
    fn deref(&self) -> &Self::Target {
        println!("deref");
        // (*self).clone()
        &self.name
    }
}

fn read_name(name: &str) {
    println!("name: {}", name);
}

#[test]
fn test_custom_deref() {
    let custom = &CustomDeref::new("custom");
    let cc = custom.clone();
    // (&cc).name();
    read_name(custom);
    let s: &str = custom;
}

fn transfer(c: &CustomDeref) -> &str {
    c
}