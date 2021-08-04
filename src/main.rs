use tokio::process;
use dbus::nonblock::SyncConnection;
use dbus_crossroads::{self,IfaceBuilder,IfaceToken,Crossroads};
use dbus_tokio::connection;
use dbus::channel::{MatchingReceiver};
//use std::sync::{Arc,Mutex};
use std::sync::Arc;
use tokio::sync::{Mutex};
mod utils;

use dbus::message::MatchRule;
use utils::ProcessWmList;

extern crate winpinner_rust;
use winpinner_rust::WinPinner;


fn goto(loc:u8){
    let mut b = process::Command::new("wmctrl");
    loc.to_string();
    b.args(&["-s",loc.to_string().as_str()]);
    let _ = b.spawn();
}

fn bruh(){
    let wmgetdk = || {
        let o = wmctrl::desktop::get_current_desktop();
        print!("{}",o);
    };
}

const ADDRS:&'static str = "com.example.test";
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let (resource, conn) = connection::new_session_sync()?;
    tokio::spawn(async {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });


    let mut p = dbus_crossroads::Crossroads::new();
    let c = Arc::new(std::sync::Mutex::new(p));


    let mut x = Arc::new(Mutex::new(WinPinner::new()));


    let token = register_iface(&c,conn.clone());
    {

        let mut cr_lock = c.lock().unwrap();

        println!("appending");
        cr_lock.insert("/newtoggle",&[token],x);
    }

    conn.request_name(ADDRS,false,true,false).await?;

    conn.start_receive(MatchRule::new_method_call(), Box::new(move |msg, conn| {
        print!("handling message");
        let mut cr_lock = c.lock().unwrap();
        cr_lock.handle_message(msg, conn).unwrap();
        true
    }));

    println!("c2");
    futures::future::pending::<()>().await;
    unreachable!()
}


type Do = Arc<Mutex<WinPinner>>;

fn register_iface(cr: &Arc<std::sync::Mutex<Crossroads>>, conn: Arc<SyncConnection>) -> IfaceToken<Do> {

    let cr2 = cr.clone();
    let mut cr_lock = cr2.lock().unwrap();

    println!("invoke toggling");
    cr_lock.register(ADDRS,|b : &mut IfaceBuilder<Do>|{
        b.method("toggle",(),(),|_,winpinner,_:()|{
            //winpinner
            let w = winpinner.clone();
            println!("invoke toggling");
            tokio::spawn(async move {
                let lock = w.lock().await;
                let mut w = lock;
                w.toggle_mechanism("brave-browser").await;
            });
            Ok(())
        });
    })
}

// use for testing





