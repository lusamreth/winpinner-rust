extern crate winpinner_rust;
use winpinner_rust::utils;
use utils::{getactivewindow,test_utils::*};

#[test]
fn test_get_active_win(){
    let ao = time(|| {
        let res = getactivewindow();
        println!("{:#?}",res);
    });
    println!("{:#?}",ao);
}

#[tokio::test]
async fn test_jump_app(){
    use winpinner_rust::WinPinner;
    let jp  = || async move {winpinner_rust::jump_app(&mut WinPinner::new()).await};
    let ptr = utils::AsyncFnPtr::new(jp);
    let t = benchmark_async(ptr).await;
    println!("jump_app took {:#?}",t);
    // last test show 33ms
}

#[test]
fn test_hash(){
    let x = time(|| {
        let op = winpinner_rust::calculate_hash(&"0x06000");
        println!("{:#?}",op)
    });
    println!("{:#?}",x);

}


