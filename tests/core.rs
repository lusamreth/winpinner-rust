
use std::thread;
use std::time::Duration;
use winpinner_rust::{WinPinner,utils};

fn launch_decoy_timing(decoy_time:u64,decoy_app:&str){
    let mut u = std::process::Command::new(decoy_app);
    let mut x = u.spawn().expect("Cannot  spawn decoy;");

    thread::sleep(Duration::from_millis(decoy_time));
    thread::spawn(move || x.kill());
    println!("done!");
}

fn launch_decoy(decoy_app:&str) -> std::process::Child {
    let mut u = std::process::Command::new(decoy_app);
    let x = u.spawn().expect("Cannot  spawn decoy;");
    //thread::sleep(Duration::from_millis(decoy_time));
    return x;
}

#[cfg(test)]
mod test_core_win_pinner {

    use super::*;

    use std::sync::{Arc,Mutex};
    use std::process::Child;
    use tokio::time::{sleep,Duration};

    #[tokio::test]
    async fn test_fetch_appid(){

        let mut wp  = WinPinner::new();
        let mock = "arandr";
        
        let child_handle:Arc<Mutex<Vec<Child>>> = Arc::new(Mutex::new(Vec::new()));
        let mux = child_handle.clone();

        let o = thread::spawn(move || {
            let child = launch_decoy(mock);
            mux.lock().unwrap().push(child);
        });

        o.join().unwrap();

        
        let mut lock = child_handle.lock().unwrap();
        let child = lock.get_mut(0).expect("Failed to launch decoy!");
        let proc_id = child.id();
        
        // avoid race condition which allow win pinner to fetch app b4 it was launched; 
        let waitabit = || async {
            println!("waiting...");
            sleep(Duration::from_millis(1000)).await
        };

        waitabit().await;
        match wp.fetch_appid(mock).await {
            Ok(stda) => {
                let o = stda.split_whitespace().next();
                assert!(o.is_some());
                let mut slice = o.unwrap().chars();
                let check_if_id = |slice:&mut std::str::Chars| {
                    let expected = &['0','x'];

                    (0..1).for_each(|i|{
                        let c = slice.nth(i).unwrap();
                        assert!(c == expected[i]);
                    });
                };
                println!("app {} has id of {}",mock,proc_id);
                check_if_id(&mut slice);
                //thread::sleep(Duration::from_millis(2000));
                waitabit().await;
                let _ = child.kill();
            },
            Err(e) => {
                eprintln!("Error! App not found despite deploying decoy!");
                eprintln!("Error type {:#?}",e);
                panic!("Test Failed")
            }
        }
        //thread::sleep(Duration::from_millis(16000));
    }
    
}


#[tokio::test]
async fn test_toggle(){
    let sleep_time:f64 = 2.00;

    let bravetg = || async move {
        let mut w = WinPinner::new();
        w.toggle_mechanism("brave-browser").await;
        println!("Sleep for 2 seconds");
        tokio::time::sleep(Duration::from_secs(2)).await;
        w.toggle_mechanism("brave-browser").await;
    };

    let ptr = utils::AsyncFnPtr::new(bravetg);
    let time = utils::test_utils::benchmark_async(ptr).await;
    let timetook = (time.as_secs_f64() - &sleep_time) * 1000.00;
    println!("time to toggle 1 cycle {:#?} ms",timetook);
    
}

