use notatin::{
    err::Error,
    cell_key_node::CellKeyNode,
    //parser_builder::{ ParserBuilder, ParserBuilderBase},
    parser_builder::{ ParserBuilder},
};
//use ascii_converter::*;
use std::str;
//gets hostname!


fn main() -> Result<(), Error> {
    println!("[*] Starting...");

    let infile = "E:\\samples\\reg_SYSTEM\\sample1\\SYSTEM";
    let logfile1 = "E:\\samples\\reg_SYSTEM\\sample1\\SYSTEM.LOG1";
    let logfile2 = "E:\\samples\\reg_SYSTEM\\sample1\\SYSTEM.LOG2";

    let mut parser = ParserBuilder::from_path(infile)
        .recover_deleted(false)
        .with_transaction_log(logfile1)
        .with_transaction_log(logfile2)
        .build()?;

    let path = "Controlset001\\Control\\ComputerName\\ComputerName";
    let ckn = parser.get_key(path, false); //returns a CellKeyNode.... I think 
    

    for x in ckn.iter(){
        let y: Option<&CellKeyNode> = x.as_ref();
        let z = y.unwrap();
        let a = z.get_value("ComputerName");
        let bytes = a.unwrap().detail;
        let computername_bytes = bytes.value_bytes().unwrap();
        //println!("{:?}",tmp);
        let cn_str = match str::from_utf8(&computername_bytes) {
            Ok(v) => v,
            Err(_e) => "!!INVALID"
        };
        println!("{}",cn_str);
        
    }


    println!("[.] Done.");
    Ok(())
}
