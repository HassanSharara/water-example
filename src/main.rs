use std::collections::HashMap;
use water_http::server:: ServerConfigurations;
use water_http::{InitControllersRoot, WaterController};

type MainHolderType = HashMap<String,String>;
InitControllersRoot!{
   name:MAIN_ROOT,
   holder_type:MainHolderType,
}


#[tokio::main]
async fn main() {

    let  configs = ServerConfigurations::bind("0.0.0.0",80);
    println!("listening on port 80");
    water_http::RunServer!(
       configs,
       MAIN_ROOT,
       MainController
   );
}
WaterController! {
   holder -> crate::MainHolderType,
   name -> MainController,
   functions -> {
       GET => / => any_thing(context) async {
           _=context.send_str("Hello World").await;
       }
   }

}