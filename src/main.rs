
mod conwertor;
mod dir;

use crate::conwertor::currency_exchange::currency_exchange;
use std::path::PathBuf;
use crate::dir::search::serch_dir::serch_dir;

fn main() {


    let (file_list, dir_list) = serch_dir("./image".to_string()).unwrap();

    for i in file_list {
        currency_exchange(i);
    }
}
