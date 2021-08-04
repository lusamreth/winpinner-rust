use futures::future::{BoxFuture,Future}; 
pub mod test_utils;

pub struct AsyncFnPtr<R> {
    func: Box<dyn Fn() -> BoxFuture<'static, R> + Send + 'static>
}

impl <R> AsyncFnPtr<R> {
    pub fn new<F>(f: fn() -> F) -> AsyncFnPtr<F::Output> where F: Future<Output = R> + Send + 'static {
        AsyncFnPtr {
            func: Box::new(move || Box::pin(f())),
        }
    }
    pub async fn run(&self) -> R { (self.func)().await }
}


pub struct ProcessWmList{
    wmlist:Vec<String>
}


fn wm_list<'a>() -> Vec<String>{
    let mut wm = std::process::Command::new("wmctrl");
    wm.arg("-lx");
    let oka = wm.output().unwrap();
    let str_res = String::from_utf8(oka.stdout).expect("failed str convert!");
    return  str_res.split("\n").map(|x|x.to_string()).collect::<Vec<String>>()

}

impl ProcessWmList {
    pub fn fetch() -> Self{
        let list = wm_list();
        return ProcessWmList {
            wmlist : list
        }
    }
    fn base_find(&self,finder:impl Fn(&str) -> bool) -> Option<&str>{
        let wmlist = &self.wmlist;
        let mut q = wmlist.iter().filter(|line|{
            finder(line)
        });
        let res:Option<&str> = q.next().map(|x|x.as_str());
        return res;
    }


    pub fn find_with_name<'a>(&self,appname:&str) -> Option<&str>{
       let name_finder = |line:&str| -> bool {
            let o = line.split_whitespace();
            let mut p = o.skip(2);
            let hay = p.next();
            if let Some(x) = hay {
                let classname = appname.split(".").next();
                return match classname {
                    Some(name) => appname == name,
                    None => x == appname
                }
            }
            return false
            //o.advance_back_by(2);
       };
       self.base_find(name_finder)
    }

    pub fn find_with_id<'a>(&self,winid:&'a str) -> Option<&str>{
        let finder = |line:&str| -> bool {
            let o = line.split_whitespace().next();
            if let Some(x) = o {
                return x == winid 
            }
            return false
        };
        self.base_find(finder)
    }
}

use std::str;
async fn process_command(child_proc : tokio::process::Child)  -> Vec<String>{
    let output = child_proc.wait_with_output().await.expect("Cannot fetch output of command!");
    let u8vec = output.stdout;

    let s = String::from_utf8(u8vec).expect("bad convert");
    return  s.split("\n")
        .map(|x| x.trim().to_string())
        .skip(6).filter(|x| !x.contains("child"))
        .collect();
}

use std::process::Stdio;
use tokio::process;

pub async fn win_root_tree(needle:&str) -> Option<String>{

    let mut xwininfo = process::Command::new("xwininfo");
    xwininfo.arg("-root");
    xwininfo.arg("-tree");

    xwininfo.stdout(Stdio::piped());
    xwininfo.stderr(Stdio::piped());

    let child_proc = xwininfo.spawn().expect("Cannot fetch window root tree!");
    let o = process_command(child_proc).await;
    let mut res = o.iter().filter(|i| {
        
        let mut str_iter = i.split_whitespace();
        let classname = str_iter.nth_back(2)
            .map(|x|x.trim().to_lowercase());

        match classname {
            Some(w) => {
                return w.contains(&needle.to_lowercase())
            },
            None => false
        }
    });

    return res.next().map(|x| {
        let id = x.split_whitespace().next().unwrap();
        id.to_owned()
    });
}

use std::i64;
pub fn getactivewindow() -> Option<String>{

    let mut xdotool = std::process::Command::new("xdotool");
    xdotool.arg("getactivewindow");
    
    match xdotool.output(){
        Ok(output) => {
            if output.stdout.is_empty(){ return None }

            let st = String::from_utf8(output.stdout).unwrap();
            let g = st.trim().to_string();
            let res = format!("0x{:x}",g.parse::<i32>().expect("Failed parsing i32!"));
            Some(res)
            //Some("placeholder".to_string())
        },
        Err(_) => None
    }

}

pub fn wmctrl_goto(appid:&str){
    let mut wmctrl = std::process::Command::new("wmctrl");
    wmctrl.args(&["-ia",appid]);
    let _ = wmctrl.spawn();
}

