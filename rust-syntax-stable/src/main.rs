pub mod main_mod;
mod test_capture;
mod test_into;
mod test_from;

use rust_syntax_stable::test_path::test_symlink;
fn main(){
    test_symlink();
}
