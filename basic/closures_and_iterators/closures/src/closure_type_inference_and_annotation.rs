use std::{thread, time::Duration};

fn closure_type_inference_annotation() {
    let _expensive_closure = |num: u32| -> u32 {
        println!("calculating slowly...");

        thread::sleep(Duration::from_secs(2));

        num
    };

    // a function definition
    fn add_one_v1(x: u32) -> u32 {
        x + 1
    }

    // an annotated closure definition
    let _add_one_v2 = |x: u32| -> u32 { x + 1 };
    //let add_one_v3 = |x| {x + 1};
    //let add_one_v4 = |x| x + 1;
}
