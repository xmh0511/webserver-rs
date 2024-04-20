### Feature
An out-of-the-box web server that is encapsulated based on salvo. 

### Use DataBase (sea-orm)
1. Enable one of the following features, depending on what you need
> - mysql 
> - sqlite
> - postgres


2. Run the following command if you want to use database
> - sea-orm-cli generate entity -o src/model

3. Import the generated model in your `main.rs` file
> - mod model;

### Use Http3
Enable `http3` feature  


### Example:
````rust
use webserver_rs::prelude::*;
use salvo::prelude::*;
use salvo::jwt_auth::HeaderFinder;
use serde::{Deserialize, Serialize};
use serde_json::json;
use tokio::io::AsyncReadExt;
use webserver_rs::{
    MemoryStream, authorization, build_cros, config::Config, expire_time, html_err,
    json_err, router, FromConfigFile, HttpResult,
};

use webserver_rs::web_core::authorization::gen_token;

#[derive(Debug, Serialize, Deserialize)]
struct JwtClaim {
    username: String,
    exp: i64,
}

#[handler]
pub fn hello(req: &mut Request, res: &mut Response) -> HttpResult<()> {
    let c = req.query::<String>("id");
    println!("{c:?}");
    let p = req.query::<String>("id").ok_or(html_err!(
        400,
        json!({
            "status":"fail",
            "msg":"id not found"
        })
        .to_string()
    ))?;
    res.render(Text::Plain(format!("hello, {p}")));
    Ok(())
}

#[handler]
pub fn login(depot: &mut Depot, res: &mut Response) -> HttpResult<()> {
    let config = depot.obtain::<Config>().map_err(|_e| {
        crate::json_err!(400,{
            "status":"fail",
            "msg":"config not found"
        })
    })?;
    let token = gen_token(
        config.secret_key.clone(),
        JwtClaim {
            exp: crate::expire_time!(Days(1)),
            username: "a".to_string(),
        },
    )
    .map_err(|e| crate::json_err!(400, {"err":e.to_string()}))?;
    res.render(Text::Plain(token));
    Ok(())
}

#[handler]
pub async fn image(res: &mut Response) -> HttpResult<()> {
    let mut file = tokio::fs::File::open("./test.png")
        .await
        .map_err(|_| crate::html_err!(404, ""))?;
    let mut s = Vec::new();
    file.read_to_end(&mut s).await.unwrap();
    res.stream(MemoryStream::new(s, 200));
    Ok(())
}
#[handler]
pub async fn text_json(res: &mut Response) -> HttpResult<()> {
    res.render(Text::Json(
        json!({
            "title":1
        })
        .to_string(),
    ));
    Ok(())
}

mod ab {
    use salvo::handler;

    #[handler]
    pub fn show() -> &'static str {
        "abc"
    }

    pub mod shop {
        #[super::handler]
        pub fn show() -> &'static str {
            "show"
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Config::from_config_file("./config.toml").expect("config file not found");
    let jwt = authorization::gen_jwt_auth::<JwtClaim>(
        config.secret_key.clone(),
        vec![Box::new(HeaderFinder::new())],
    );

    webserver_rs::serve_routes! {
        config =>[
        // http://localhost:8080/hello
        router!([get, post] => @hello)
            .hoop(jwt)
            .hoop(authorization::AuthGuard::new(|_e| html_err!("unauthorized"))),
        // http://localhost:8080/user/login    
        router!([get] => /user/@login),
        // http://localhost:8080/a/b/show
        router!([get, post] => a/b/@ab::show),
        // http://localhost:8080/b/c/show/*	
        router!([get, post, put] => /b/c/@ab::show/<**path>),
        // http://localhost:8080/test_json
        router!([get, post] => @text_json).hoop(build_cros("*")),
        // http://localhost:8080/ab/shop/show    
        router!([get, post] => ...@ab::shop::show).hoop(build_cros("*")),
        ]
    };
    Ok(())
}

````
