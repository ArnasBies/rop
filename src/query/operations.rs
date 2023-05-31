use super::*;
use std::fs;
use regex::Regex;

impl Query{
    pub fn new(args: Vec<String>) -> Self{
        let mut operation = match args.get(1){
            Some(x) => x.to_operation(),
            None => Operation::Help,
        };

        let expression: String = match operation{
            Operation::Help | Operation::Paste => "".to_string(),
            _ => match args.get(2){
                Some(x) => x.to_string(),
                None => panic!("couldn't parse regex expression"),
            },
        };

        let mut item_path = PathBuf::new();

        match operation{
            Operation::Help | Operation::Paste => {},
            _ => match args.get(3){
                Some(x) => item_path.push(x.clone().to_string()),
                None => panic!("couldn't parse regex expression"),
            },
        };

        match &mut operation{
            Operation::Extract(x) => 
                x.push_str(args.get(4).expect("Expected new folder name for extraction")),

            Operation::Move(x) | Operation::Copy(x) => 
                x.push(args.get(4).expect("Expected extra path for this operation")),

            _ => {}

        };

        Query{
            op: operation,
            expression,
            item_path,
        }
    }

    pub fn execute(&self){
        match &self.op{
            Operation::Help => help(),
            Operation::Move(x) => move_files(self, x),
            Operation::Cut => cut(self),
            Operation::List => list(self),
            Operation::Remove => remove(self),
            Operation::Extract(x) => extract(self, x),
            Operation::Paste => paste(self),
            Operation::Copy(x) => copy(self, x),
        }
    }
}

pub fn help(){

}

pub fn move_files(query: &Query, item_path: &PathBuf){
    let paths = fs::read_dir(query.item_path.clone()).expect("failed to read items from directory");

    let expression = Regex::new(&query.expression).expect("couldn't parse directory");

    for path in paths{
        if let Ok(item) = path{
            let path_and_name = (item.path().clone(), item.file_name());
            
            if expression.is_match(path_and_name.1.clone().to_str().unwrap()){
                let mut move_path = item_path.clone();
                move_path.push(path_and_name.1.clone().to_str().unwrap());
                
                match fs::rename(path_and_name.0, move_path){
                    Ok(_) => println!("successfully moved file: {} to {}", path_and_name.1.clone().to_str().unwrap(), &item_path.to_str().unwrap()),
                    Err(_) => println!("failed to move file: {} to {}", path_and_name.1.clone().to_str().unwrap(), &item_path.to_str().unwrap()),
                };
            };
        }
    }
}

pub fn cut(query: &Query){

}

pub fn remove(query: &Query){
    let paths = fs::read_dir(query.item_path.clone()).expect("failed to read items from directory");

    let expression = Regex::new(&query.expression).expect("couldn't parse directory");

    for path in paths{
        if let Ok(item) = path{
            let path_and_name = (item.path().clone(), item.file_name());
            
            if expression.is_match(path_and_name.1.clone().to_str().unwrap()){
                match fs::remove_file(path_and_name.0){
                    Ok(_) => println!("successfully removed file: {}", path_and_name.1.clone().to_str().unwrap()),
                    Err(_) => println!("failed to remove file: {}", path_and_name.1.clone().to_str().unwrap()),
                };
            };
        }
    }
}

pub fn paste(query: &Query){

}

pub fn copy(query: &Query, item_path: &PathBuf){
    let paths = fs::read_dir(query.item_path.clone()).expect("failed to read items from directory");

    let expression = Regex::new(&query.expression).expect("couldn't parse directory");

    for path in paths{
        if let Ok(item) = path{
            let path_and_name = (item.path().clone(), item.file_name());
            
            if expression.is_match(path_and_name.1.clone().to_str().unwrap()){
                let mut copy_path = item_path.clone();
                copy_path.push(path_and_name.1.clone().to_str().unwrap());
                
                match fs::copy(path_and_name.0, copy_path){
                    Ok(_) => println!("successfully coppied file: {} to {}", path_and_name.1.clone().to_str().unwrap(), &item_path.to_str().unwrap()),
                    Err(_) => println!("failed to copy file: {} to {}", path_and_name.1.clone().to_str().unwrap(), &item_path.to_str().unwrap()),
                };
            };
        }
    }
}

pub fn list(query: &Query){
    let paths = fs::read_dir(query.item_path.clone()).expect("failed to read items from directory");

    let expression = Regex::new(&query.expression).expect("couldn't parse directory");
    
    for path in paths{
        if let Some(name) = path.unwrap().file_name().to_str(){
            if expression.is_match(name){
                println!("{name}");
            }
        }   
    }
}

pub fn extract(query: &Query, folder_name: &String){
    let mut folder_path = folder_name.clone();
    folder_path.push('/');
    let mut new_path = query.item_path.clone();
    new_path.push(folder_path);

    fs::create_dir(&new_path).expect("couldn't create new directory here");
    move_files(query, &new_path);
}
