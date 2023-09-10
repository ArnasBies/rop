use std::path::PathBuf;

mod operations;

pub struct Query{
    op: Operation,
    expression: String,
    item_path: PathBuf,
}

enum Operation{
    Help,
    Move(PathBuf),
    List,
    Remove,
    Extract(String),
    Copy(PathBuf),
}

trait GetOperation{
    fn to_operation(&self) -> Operation;
}

impl GetOperation for String{
    fn to_operation(&self) -> Operation {
        return match self.to_lowercase().as_str(){
            "help" => Operation::Help,
            "move" => Operation::Move(PathBuf::new()),
            "list" => Operation::List,
            "remove" => Operation::Remove,
            "extract" => Operation::Extract(String::new()),
            "copy" => Operation::Copy(PathBuf::new()),
            _ => panic!("invalid operation"),
        }
    }
}
