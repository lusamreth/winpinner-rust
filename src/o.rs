use dbus::nonblock;
use dbus_tokio::connection;
use dbus_crossroads::{MethodErr, Crossroads, IfaceToken, IfaceBuilder};
use futures::future;
use dbus::channel::{MatchingReceiver, Sender};
use dbus::message::MatchRule;
use tokio;

struct Test{
    count:i32
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>>{

    let wp = WinPinner {
       state : false,
       last_window : "",
       cached_workspace : None
    };

    let (resource, c) = connection::new_session_sync()?;
    c.request_name("org.example.bb.rs", false, true, false).await?;

    tokio::spawn(async move {
        let err = resource.await;
        panic!("Lost connection to D-Bus: {}", err);
    });

    let mut crossroad = Crossroads::new();

    crossroad.set_async_support(Some((c.clone(), Box::new(|x| { tokio::spawn(x); }))));

    let iface_token = crossroad.register("org.example.bb.rs",|b: &mut IfaceBuilder<Test>|{

        b.method("hello",("name",),(),|mut ctx,cr,(name,):(String,)|{
            print!("hello");
            Ok(())
        });
        b.signal::<(), _>("CheckComplete", ());
    });


    crossroad.insert("/hello", &[iface_token], Test { count: 0});

    c.start_receive(MatchRule::new_method_call(), Box::new(move |msg, conn| {
        print!("handling message");
        crossroad.handle_message(msg, conn).unwrap();
        true
    }));

    future::pending::<()>().await;
    unreachable!()
}

struct WinPinner<'a>{
   state : bool,
   last_window : &'a str,
   cached_workspace : Option<i16>,
}


impl <'a> WinPinner<'a>{
}



