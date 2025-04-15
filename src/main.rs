mod managers;

use managers::page;

fn main() {

    page::allocate_page();

    println!("Hello, world!");
}
