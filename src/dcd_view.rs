use dcd::*;

/** Datastore Viewer
 
    This is a simple command-line tool to perform various exploratory tasks on the datastore that should also serve as an example of how to use the datastore api. 

    Usage:

    dcd-view CMD ARGS

    Where CMD is one of the following:

    And arguments can be: 
 */
fn main() {
    let ds = DatastoreView::new("/foo/bar");

    

}

fn savepoints(ds : & DatastoreView) {
    let mut s = ds.savepoints();
    let mut num = 0;
    for (_, sp) in s.iter() {
        println!("{}", sp);
        num += 1;
    }
    println!("Total {} savepoints found.", num);
}



