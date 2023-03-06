use core::mem::ManuallyDrop;

union Mouse {
    name: &'static str,
    size: u8,
    weight: u16,
    age: ManuallyDrop<u32>,
}

#[test]
fn test_union() {
    let mut mouse = Mouse { name: "mouse" };
    unsafe {
        println!("{}", mouse.name);
        println!("{}", mouse.size);
        println!("{}", mouse.weight);
        println!("{:?}", mouse.age);
    }
    mouse.name = "rat";
    unsafe {
        println!("{}", mouse.name);
        println!("{}", mouse.size);
        println!("{}", mouse.weight);
        println!("{:?}", mouse.age);
    }
    mouse.size = 1;
    unsafe {
        println!("{}", mouse.name);
        println!("{}", mouse.size);
        println!("{}", mouse.weight);
        println!("{}", *mouse.age);
    }
    /*mouse.age = ManuallyDrop::new(43);
    unsafe {
        println!("{}", mouse.name);
        println!("{}", mouse.size);
        println!("{}", mouse.weight);
        println!("{}", *mouse.age);
    }*/
}

#[derive(Debug)]
struct S(i32);

impl Clone for S {
    fn clone(&self) -> Self {
        todo!()
    }
}

impl Copy for S {}

// not `Copy`, no drop glue
union U {
    f1: ManuallyDrop<Vec<i32>>,
    f2: (S, S),
    f3: i32,
}

#[test]
fn test_manually_drop() {
    let mut u: U = U { f2: (S(42), S(23)) };
// Now `u` is not initialized: `&u`, `&u.f2` and `&u.f2.0` are all rejected.

// We can write into uninitialized inner fields:
    u.f2.1 = S(42);
    unsafe { let _x = &u.f2.1; } // This field is initialized now.
// But this does not change the initialization state of the union itself,
// or any other (inner) field.

// We can initialize by assigning an entire field:
    u.f1 = ManuallyDrop::new(Vec::new());
    unsafe {
        println!("{:?}", u.f1);
        println!("{:?}", u.f2);
        println!("{:?}", u.f3);
    }
// Now *all (nested) fields* of `u` are initialized, including the siblings of `f1`:
    unsafe { let _x = &u.f2; }
    unsafe { let _x = &u.f2.0; }

// Equivalently, we can assign the entire union:
    u = U { f2: (S(42), S(23)) };
// Now `u` is still initialized.

// Copying does not change anything:
    unsafe {
        let _x = u.f3;
// Now `u` is still initialized.

// We can move out of an initialized union:
        let v = u.f1;
// Now `f1` *and its siblings* are no longer initialized (they got "moved out of"):
// `let _x = u.f2;` would hence get rejected, as would `&u.f1` and `foo(u)`.
        u.f1 = v;
// Now `u` and all of its fields are initialized again ("moving back in").

// When we move out of an inner field, the other union fields become uninitialized
// even if they are `Copy`.
        let s = u.f2.1;
// Now `u.f1` and `u.f3` are no longer initialized.  But `u.f2.0` is:
        let s = u.f2.0;
    }
    /*unsafe {
        println!("{:?}", u.f1);
        println!("{:?}", u.f2);
        println!("{:?}", u.f3);
    }*/
}