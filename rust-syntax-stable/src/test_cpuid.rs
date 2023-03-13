use core::arch::x86_64::*;
use std::char::from_digit;

#[test]
fn test() {
    let leaf0;
    unsafe {
        leaf0 = crate::test_cpuid::__cpuid(0);
    }
    println!("leaf0: {:?}", leaf0);
    // let ebx_ascii = hex_string_to_ascii_string(dec_to_hex(leaf0.ebx));
    let ebx_u8_slice = &leaf0.ebx.to_le_bytes();
    let ebx_ascii = std::str::from_utf8(ebx_u8_slice).unwrap();
    let edx_u8_slice: [u8; 4] = u32::to_le_bytes(leaf0.edx);
    let edx_ascii = std::str::from_utf8(&edx_u8_slice).unwrap();
    let ecx_u8_slice: &[u8] = &leaf0.ecx.to_le_bytes();
    let ecx_ascii = std::str::from_utf8(ecx_u8_slice).unwrap();
    let vendor_string = format!("{}{}{}", ebx_ascii, edx_ascii, ecx_ascii);
    println!("vendor_string: {}", vendor_string);
    let leaf1;
    unsafe {
        leaf1 = __cpuid(1);
    }
    println!("leaf1: {:?}", leaf1);
    let family = (leaf1.eax >> 8) & 0xf;
    let model = (leaf1.eax >> 4) & 0xf;
    let stepping = leaf1.eax & 0xf;
    println!("family: {family}, model: {model}, stepping: {stepping}");
    let extended_family = (leaf1.eax >> 20) & 0xff;
    let extended_model = (leaf1.eax >> 16) & 0xf;
    let cpu_features = leaf1.edx;
    let logical_processors = (leaf1.ebx >> 16) & 0xff;
    println!("logical_processors: {}", logical_processors);
    let leaf4;
    unsafe {
        leaf4 = __cpuid(4);
    }
    println!("leaf4: {:?}", leaf4);
    println!("cpu_features: {:032b}", cpu_features);
    let physical_cores = ((leaf4.eax >> 26) & 0x3f) + 1;
    let hyper_threading = cpu_features & (1 << 28) != 0 && physical_cores < logical_processors;
    println!("physical_cores: {}", physical_cores);
    println!("hyper_threading: {}", hyper_threading);


    let leaf_80000000h;
    unsafe {
        leaf_80000000h = __cpuid(0x80000000);
    }
    println!("leaf_80000002h: {:?}", leaf_80000000h);
    if leaf_80000000h.eax < 0x80000004 {
        return;
    }
    let leaf_80000002h;
    let leaf_80000003h;
    let leaf_80000004h;
    unsafe {
        leaf_80000002h = __cpuid(0x80000002);
        leaf_80000003h = __cpuid(0x80000003);
        leaf_80000004h = __cpuid(0x80000004);
    }
    let mut brand_string = String::new();
    for leaf in [leaf_80000002h, leaf_80000003h, leaf_80000004h] {
        let eax_ascii = hex_string_to_ascii_string(dec_to_hex(leaf.eax));
        let ebx_ascii = hex_string_to_ascii_string(dec_to_hex(leaf.ebx));
        let ecx_ascii = hex_string_to_ascii_string(dec_to_hex(leaf.ecx));
        let edx_ascii = hex_string_to_ascii_string(dec_to_hex(leaf.edx));
        brand_string += &*format!("{}{}{}{}", eax_ascii, ebx_ascii, ecx_ascii, edx_ascii);
    }
    println!("brand_string: {}", brand_string);
}

fn dec_to_hex(dec: u32) -> String {
    let mut hex = String::new();
    let mut dec = dec;
    while dec > 0 {
        let rem = dec % 16;
        dec /= 16;
        hex = match rem {
            10 => "A".to_string() + &hex,
            11 => "B".to_string() + &hex,
            12 => "C".to_string() + &hex,
            13 => "D".to_string() + &hex,
            14 => "E".to_string() + &hex,
            15 => "F".to_string() + &hex,
            _ => rem.to_string() + &hex,
        };
    }
    hex
}

fn hex_to_char(s: &str) -> Result<char, std::num::ParseIntError> {
    u8::from_str_radix(s, 16).map(|n| n as char)
}

fn hex_string_to_ascii_string(hex: String) -> String {
    let mut ascii = String::new();
    let mut hex = hex;
    // while hex.len() > 0 {
    //     let rem = &hex[hex.len() - 2..];
    //     hex = hex[..hex.len() - 2].to_string();
    //     let c = hex_to_char(rem).unwrap();
    //     ascii = c.to_string() + &ascii;
    // }
    let len = hex.len();
    let r = (0..len).step_by(2);
    for i in (0..len).step_by(2) {
        let rem = &hex[i..i + 2];
        let c = hex_to_char(rem).unwrap();
        ascii = c.to_string() + &ascii;
    }
    ascii
}
