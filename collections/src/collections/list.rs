
pub mod vectors {
    pub fn read_vector<T: std::fmt::Debug>(vec: Vec<T>){
        for i in vec {
            println!("{:?}", i);
        };
    }

        pub fn read_vector_string<T: std::fmt::Display>(vec: Vec<T>) -> String {
            let mut result = String::from("[");
            for _i in vec {
                let res = format!("{}", _i).to_owned();
                result.push_str(&res[..]);
                result.push_str(",");
            }
            result.push_str("]");
            return result;
        }
}