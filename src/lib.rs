pub mod utils;
use utils::ProcessWmList;
use std::collections::HashMap;

#[derive(Debug)]
pub struct WinPinner {
    pub win_storage : HashMap<String,String>,
    pub app_cache : HashMap<u64,String>,
    pub winstack : Vec<String>,
}

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash,Hasher};
pub fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}

type ErrorMsg = String;

#[derive(Debug)]
pub enum WinstateError{
    NotFound, // special case 
    PanicState(ErrorMsg)
}

impl WinPinner{
    //
    pub fn new() -> Self {
        return WinPinner {
            win_storage: HashMap::new(),
            app_cache:HashMap::new(),
            winstack:Vec::new()
        }
    }

    pub fn add_win(&mut self,win_id:String){
        let wm = ProcessWmList::fetch();
        if self.win_storage.get(win_id.as_str()) == None {
            if let Some(info) = wm.find_with_id(win_id.as_str()) {
                self.win_storage.insert(win_id,info.to_string());
            }else {
                println!("None found!")
            }
        }
        //self.WinStorage.insert() 
    }
    
    pub async fn toggle_mechanism(&mut self,app:&str) {
        let active = utils::getactivewindow().unwrap();

        self.winstack.push(active.clone());

        match self.fetch_appid(app).await {
            Ok(appid) => {

                if active != appid {
                   utils::wmctrl_goto(appid.as_str());
                }else{
                   let i = self.winstack.first().unwrap();
                   println!("ii {}",i);
                   utils::wmctrl_goto(i);
                }
            },
            Err(_) => panic!("whut")
        }
    }

    pub async fn fetch_appid(&mut self,app:&str) -> Result<String,WinstateError>{

        let hashed = calculate_hash(&app);
        if let Some(wid) = self.app_cache.get(&hashed){
            return Ok(wid.to_owned())
        }
        
        let id_opt = utils::win_root_tree(app).await;
        let res:Result<String,WinstateError> = match id_opt {
            Some(id) => {
                self.app_cache.insert(hashed,id.to_owned());
                Ok(id.to_owned())
            },
            None => Err(WinstateError::NotFound)
        };
        return res
        //utils::wmctrl_goto(id.unwrap().as_str());
    }
    
    pub fn append_active_window(&mut self){
        let wid = utils::getactivewindow();
        self.add_win(wid.unwrap());
    }


}

pub async fn jump_app(wp:&mut WinPinner){
    wp.append_active_window();

    let app = "Brave-browser";
    let id = utils::win_root_tree(app).await;
    utils::wmctrl_goto(id.unwrap().as_str());
}



