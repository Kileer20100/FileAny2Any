
mod conwertor;
mod dir;

use crate::conwertor::currency_exchange::currency_exchange;
use crate::dir::search::serch_dir::serch_dir;
use rayon::prelude::*;

pub extern "C" fn StartConwert(input_path: &str, output_path: &str, expansion: &str) -> bool{


    //let input_path = "./image";
    //let output_path = "./output";
    //let expansion = "PNG";


    let (file_list, dir_list) = serch_dir(input_path.to_string(), ).unwrap();


    file_list.par_iter().for_each(|i| {
        currency_exchange(i.clone(), output_path.to_string(), expansion.to_string());
    });

    true // return true to indicate success
}
