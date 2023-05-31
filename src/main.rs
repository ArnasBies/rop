mod query;
use query::Query;

fn main(){
    let args: Vec<String> = std::env::args().collect();
    let query = Query::new(args);
    query.execute();
}
