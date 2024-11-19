use libemei::page_manage::*;

fn main() {
    let page_size = PageSize::from_system();
    let r = PageHandle::new(64000_usize, page_size);
    println!("{:p}", r.ptr);
    let r = PageHandle::new(64000_usize, page_size);
    println!("{:p}", r.ptr);
    let r = PageHandle::new(64000_usize, page_size);
    println!("{:p}", r.ptr);
    println!();
    let mut rs = vec![];
    for _ in 0..3 {
        let r = PageHandle::new(64000_usize, page_size);
        // println!("{:p}", r.ptr);
        rs.push(r);
    }
    for i in rs {
        println!("{:p}", i.ptr);
    }
}
