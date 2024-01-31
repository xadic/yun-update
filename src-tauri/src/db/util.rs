use tiberius::{Config, AuthMethod, Client, Query};
use tokio::{net::TcpStream, fs::read_to_string};
use tokio_util::compat::{TokioAsyncWriteCompatExt, Compat};
use anyhow::{Result, Context};
use chrono::Local;

use crate::config::init_config;

// 备份云检测数据库
pub async fn backup_acdb() -> Result<()> {
    let mut client = get_client().await?;
    let app_config = init_config().await;

    let db_name = app_config.db_name;
    let backup_path = String::from("E:\\backup\\acdb.bak");
    let stmt = format!("backup database @P1 to disk = @P2");
    let stmt = format!("USE [{}] GO {}", db_name, stmt);

    let mut stmt = Query::new(stmt);
    let params = vec![db_name, backup_path];
    for param in params.into_iter() {
        stmt.bind(param);
    }
    stmt.query(&mut client).await?;
    Ok(())
}

async fn get_client() -> Result<Client<Compat<TcpStream>>> {
    let app_config = init_config().await;
    let mut config = Config::new();
    // 数据库连接信息
    config.host(app_config.sql_server.host);
    config.port(app_config.sql_server.port.parse::<u16>().unwrap());
    config.authentication(AuthMethod::sql_server(
        app_config.sql_server.user,
        app_config.sql_server.password,
    ));
    config.trust_cert(); // on production, it is not a good idea to do this
    
    let tcp = TcpStream::connect(config.get_addr()).await.context("无法连接到数据库服务器IP")?;
    tcp.set_nodelay(true)?;

    let client = Client::connect(config, tcp.compat_write()).await.context("无法连接到数据库")?;

    Ok(client)
}

// 清除所有标定记录
pub async fn clear_all_calibration() -> Result<(), String>{
    let app_config = init_config().await;
    let mut config = Config::new();
    // 数据库连接信息
    config.host(app_config.sql_server.host);
    config.port(app_config.sql_server.port.parse::<u16>().unwrap());
    config.authentication(AuthMethod::sql_server(
        app_config.sql_server.user,
        app_config.sql_server.password,
    ));
    config.trust_cert(); // on production, it is not a good idea to do this

    let tcp = TcpStream::connect(config.get_addr()).await.map_err(|err| err.to_string())?;
    tcp.set_nodelay(true).map_err(|err| err.to_string())?;

    let mut client = Client::connect(config, tcp.compat_write()).await.map_err(|err| err.to_string())?;

    let db_name = app_config.db_name;
    let stmt = read_to_string("./sql/drop_and_crate.sql").await.map_err(|err| err.to_string())?;
    let stmt = format!("USE [{}] GO {}", db_name, stmt);

    let mut stmt = Query::new(stmt);
    let params = vec![db_name];
    for param in params.into_iter() {
        stmt.bind(param);
    }
    stmt.query(&mut client).await.map_err(|err| err.to_string())?;

    Ok(())
}

