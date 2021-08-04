extern crate winpinner_rust;
use winpinner_rust::{WinPinner,utils};
use utils::{AsyncFnPtr,win_root_tree,ProcessWmList,test_utils::*};
mod components;

#[tokio::test]
async fn test_win_tree(){
    let test_async = || async move {
        win_root_tree("brave-browser").await;
    };

    let ptr = AsyncFnPtr::new(test_async);
    benchmark_async(ptr).await;
}


fn fetch_appid_xdotool() -> String{
    let mut a = std::process::Command::new("xdotool");
    a.args(&["search","--classname","brave-browser"]);
    let b = a.output().unwrap();
    let val:i32 = String::from_utf8_lossy(&b.stdout).trim().parse().expect("");
    return format!("00x{:x}",val);
}

#[cfg(test)]
mod BenchmarkFetch {
    use super::*;
    const needle:&'static str = "brave-browser";
    type Res<'a> = &'a mut Vec<String>;

    fn wmctrl_fetch<'a>(results:Res<'a>) -> std::time::Duration{
        let o = || {
            let wm = ProcessWmList::fetch();
            let o = wm.find_with_name(needle).expect("what??plz open brave!").to_owned();
            results.push(o);
        };
        time(o)
    }
    
    fn xdotool_test<'a>(results:Res<'a>) -> std::time::Duration {
        time(|| results.push(fetch_appid_xdotool()))
    }

    async fn win_root_tree_time<'a>(results:Res<'a>) -> std::time::Duration{
        use std::time::Instant;
        let now = Instant::now();

        let x = win_root_tree("Brave-browser").await;
        results.push(x.unwrap());
        return now.elapsed();
    }
    
    #[tokio::test]
    async fn compare_appid_fetcher(){
        use std::cmp::Ordering;

        let mut results = Vec::new();

        let wmctrl_time = wmctrl_fetch(&mut results);
        let xdotool_time = xdotool_test(&mut results);
        let win_root_tree_time = win_root_tree_time(&mut results).await;

        println!("\n");
        println!("executed time!");
        

        let res = [wmctrl_time,win_root_tree_time,xdotool_time];

        println!("0.wmctrl time! : {:#?}",res[0]);
        println!("1.win_root_tree_time!: {:#?}",res[1]);
        println!("2.xdotool_time!: {:#?}",res[2]);
        
        let m = res.iter().enumerate().min_by(|(_,a),(_,b)| a.partial_cmp(b).unwrap_or(Ordering::Equal));
        println!("Number {:#?} is the fastest !",m.unwrap().0);
        println!("{:?}",results);
        //let wmctrl_time = 
    }
        
}

#[test]
fn benchmark_find_with_name(){
    let testfind = || {
        let o = ProcessWmList::fetch();
        o.find_with_name("brave-browser");
    };

    let each_run = |recorder:&mut Vec<f64>| {
            let eachtime = time(testfind).as_secs_f64();
            println!("each time {}",eachtime);
            recorder.push(eachtime);
        };

    let avg = avg_run(each_run,1) * 1000.00;
    println!("average runtime : {} ms",avg );
    //assert!(avg < 3.5);

}

#[test]
fn run_winpinner(){
    let mut w = WinPinner::new();
    
    let z = ProcessWmList::fetch();
    let info = z.find_with_name("brave-browser").expect("brave should be open!");
    let id = info.split_whitespace().next().unwrap();

    println!("zz {}",id);
    let x_testadd = || w.add_win(id.to_string());
    time(x_testadd);
}
