
mod conwertor;
mod dir;

use crate::conwertor::currency_exchange::currency_exchange;
use std::path::PathBuf;
use crate::dir::search::serch_dir::serch_dir;
use rayon::prelude::*;
fn main() {


    let (file_list, dir_list) = serch_dir("./image".to_string()).unwrap();


    file_list.par_iter().for_each(|i| {
        currency_exchange(i.clone());
    });
}
