use fib::App;
use general_storage_file::{FileStorage, IfDirectoryMissing};

fn main() {
    let storage = FileStorage::next_to_exe("storage", IfDirectoryMissing::Create).unwrap();
    let mut app = App::new(storage);
    println!("{}", app.get());
    app.next_and_save();
}
