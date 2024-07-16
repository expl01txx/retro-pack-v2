use librp2::easy::{pack_file, pack_folder, unpack};

pub fn process(filename: String, password: String) -> String {
    let path = std::path::Path::new(&filename);

    

    if path.is_file() {

        //check file extension 
        if path.extension().unwrap() != "rp2" {
            //pack file
            let output_path = path.parent().unwrap().to_str().unwrap();

            //add to path file name and replace extension
            let output_path = format!("{}\\{}.rp2", output_path, path.file_stem().unwrap().to_str().unwrap());

            let result = pack_file(filename.clone(), output_path.to_string(), password);
            if result.is_err() {
                return result.unwrap_err().to_string();
            }
            return "Pack succeeded!".to_string();
        }
        
        let output_path = path.parent().unwrap().to_str().unwrap().to_string() + "\\";

        let result = unpack(filename.clone(), output_path.to_string(), password);
        if result.is_err() {
            return result.unwrap_err().to_string();
        }
        "Unpack succeeded!".to_string()

    } else {
        let output_path = path.parent().unwrap().to_str().unwrap();
        let output_path = format!("{}\\{}.rp2", output_path, path.file_name().unwrap().to_str().unwrap());

        let result = pack_folder(filename.clone(), output_path.to_string(), password);
        if result.is_err() {
            return result.unwrap_err().to_string();
        }
        "Pack succeeded!".to_string()
    }
}