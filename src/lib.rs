extern crate sapper;
extern crate sapper_tmpl;
extern crate sapper_query;
extern crate sapper_body;
extern crate sapper_session;
extern crate sapper_logger;

extern crate serde;
extern crate serde_json;

use sapper::{Request, Response, Result};

pub use sapper::PathParams;
pub use sapper_query::QueryParams;
pub use sapper_body::FormParams;
pub use sapper_body::JsonParams;
pub use sapper_session::{ SessionVal, set_cookie };
pub use sapper_tmpl::Context;
pub use sapper_tmpl::render;

pub fn init(req: &mut Request, cookie_key: &'static str) -> Result<()> {
    sapper_logger::init(req)?;
    sapper_query::parse(req)?;
    sapper_body::parse(req)?;
    sapper_session::session_val(req, cookie_key)?;
    
    Ok(())
}


pub fn finish(req: &Request, res: &mut Response) -> Result<()> {
    sapper_logger::log(req, res)?;

    Ok(())
}


// ============ Status Code ============

#[macro_export]
macro_rules! res_redirect {
    ($redirect_uri:expr) => ({
        use sapper::Response;
        use sapper::status;
        use sapper::header::Location;

        let mut response = Response::new();
        response.set_status(status::Found);
        //response.set_status(status::TemporaryRedirect);
        response.headers_mut().set(Location($redirect_uri.to_owned()));
        response.write_body(format!("redirect to {}", $redirect_uri));

        Ok(response)
    })
}

#[macro_export]
macro_rules! res_400 {
    ($info:expr) => ({
        use sapper::Response;
        use sapper::status;

        let mut response = Response::new();
        response.set_status(status::BadRequest);
        response.write_body($info.to_owned());

        Ok(response)
    })
}

#[macro_export]
macro_rules! res_500 {
    ($info:expr) => ({
        use sapper::Response;
        use sapper::status;

        let mut response = Response::new();
        response.set_status(status::InternalServerError);
        response.write_body($info.to_owned());

        Ok(response)
    })
}

// ============ Json ============

#[macro_export]
macro_rules! res_json {
    ($json:expr) => ({
        use serde_json;
        use sapper::Response;
        use sapper::header::ContentType;

        let mut response = Response::new();
        response.headers_mut().set(ContentType::json());
        response.write_body(serde_json::to_string(&$json).unwrap());

        Ok(response)
    })
}


#[macro_export]
macro_rules! res_json_ok {
    ($info:expr) => ({
        use sapper::Response;
        use serde_json;
        
        let json2ret = json!({
            "success": true,
            "info": $info
        });

        res_json!(json2ret)
    })
}

#[macro_export]
macro_rules! res_json_error {
    ($info:expr) => ({
        use sapper::Response;
        use serde_json;
        
        let json2ret = json!({
            "success": false,
            "info": $info
        });
        
        res_json!(json2ret)
    })
}

// ============ Page Render ============

#[macro_export]
macro_rules! res_html {
    ($html:expr, $context:expr) => ({
        use sapper::Response;
        use sapper::header::ContentType;

        let res_str = render($html, $context);

        let mut response = Response::new();
        response.headers_mut().set(ContentType::html());
        response.write_body(res_str);

        Ok(response)
    })
}


// ============ DB ============

#[macro_export]
macro_rules! get_db {
    ($req:expr, $dbkey:ty) => ({
        let pool_wr = $req.ext().get::<$dbkey>();
        let db = match pool_wr {
            Some(pool) => {
                match pool.connect() {
                    Ok(conn) => conn,
                    Err(_) => {
                        return res_500!("get db connection failed.")
                    }
                }
            },
            None => return res_500!("no db defined.")
        };
        
        db
    })
}

// ============ Params ============

#[macro_export]
macro_rules! get_params {
    ($req:expr, $tykey:ty) => ({
        match $req.ext().get::<$tykey>() {
            Some(params) => {
                params
            },
            None => return res_400!("no params")
        }
    })
}

#[macro_export]
macro_rules! get_path_params {
    ($req:expr) => ({
        get_params!($req, PathParams)
    })
}


#[macro_export]
macro_rules! get_query_params {
    ($req:expr) => ({
        get_params!($req, QueryParams)
    })
}

#[macro_export]
macro_rules! get_form_params {
    ($req:expr) => ({
        get_params!($req, FormParams)
    })
}

#[macro_export]
macro_rules! get_json_params {
    ($req:expr) => ({
        match serde_json::from_value(get_params!($req, JsonParams).clone()) {
            Ok(val) => val,
            Err(_) => {
                return res_400!("Json parameter not match to struct.");
            }
        }
    })
}

/*
#[macro_export]
macro_rules! get_json_params {
    ($req:expr) => ({
        get_params!($req, JsonParams)
    })
}
*/

#[macro_export]
macro_rules! t_condition {
    ($bool:expr, $prompt:expr) => ({
        match $bool {
            true => (),
            false => {
                println!("test param condition result: {}", $prompt);
                return res_400!(format!("test param condition result: {}.", $prompt) );
            }
        }
    })
}

#[macro_export]
macro_rules! _parse_error {
    ($field:expr) => ({
        println!("parse parameter type error {}.", $field);
        return res_400!(format!("parse parameter type error {}.", $field));
    })
}

#[macro_export]
macro_rules! _missing_or_unrecognized {
    ($field:expr) => ({
        println!("missing or unrecognized parameter {}.", $field);
        return res_400!(format!("missing or unrecognized parameter {}.", $field));
    })
}

#[macro_export]
macro_rules! _use_default {
    ($field:expr, $default:expr) => ({
        println!("missing or unrecognized parameter {}, using default {}.", $field, $default);
        // return default
        $default
    })
}



#[macro_export]
macro_rules! t_param_parse {
    ($params:expr, $field:expr, $tykey:ty) => ({
        match $params.get($field) {
            Some(ref astr) => {
                let _t = &astr[0];
                match _t.parse::<$tykey>() {
                    Ok(output) => output,
                    Err(_) => _parse_error!($field)
                }
            },
            None =>  _missing_or_unrecognized! ($field)
        }
    })
}


// for PathParams, QueryParams and FormParams
#[macro_export]
macro_rules! t_param {
    ($params:expr, $field:expr) => ({
        match $params.get($field) {
            Some(ref astr) => &astr[0],
            None =>  _missing_or_unrecognized! ($field)
        }
    })
}

// for PathParams, QueryParams and FormParams, default version
#[macro_export]
macro_rules! t_param_default {
    ($params:expr, $field:expr, $default:expr) => ({
        match $params.get($field) {
            Some(ref astr) => &astr[0],
            None =>  _using_default! ($field, $default)
        }
    })
}

// ========== for JSON ==========

#[macro_export]
macro_rules! t_str {
    ($params:expr, $field:expr) => ({
        match $params.get(&$field) {
            Some(&Value::String(ref astr)) => astr,
            _ =>  _missing_or_unrecognized! ($field)
        }
    })
}

#[macro_export]
macro_rules! t_str_default {
    ($params:expr, $field:expr, $default:expr) => ({
        match $params.get(&$field) {
            Some(&Value::String(ref astr)) => astr,
            _ =>  _using_default! ($field, $default)
        }
    })
}

#[macro_export]
macro_rules! t_i64 {
    ($params:expr, $field:expr) => ({
        match $params.get(&$field) {
            Some(&Value::I64(int)) => int,
            Some(&Value::U64(int)) => int as i64,
            _ =>  _missing_or_unrecognized! ($field)
        }
    })
}

#[macro_export]
macro_rules! t_i64_default {
    ($params:expr, $field:expr, $default:expr) => ({
        match $params.get(&$field) {
            Some(&Value::I64(int)) => int,
            Some(&Value::U64(int)) => int as i64,
            _ =>  _using_default! ($field, $default)
        }
    })
}

#[macro_export]
macro_rules! t_f64 {
    ($params:expr, $field:expr) => ({
        match $params.get(&$field) {
            Some(&Value::I64(int)) => int as f64,
            Some(&Value::U64(int)) => int as f64,
            Some(&Value::F64(ft)) => ft,
            _ =>  _missing_or_unrecognized! ($field)
        }
    })
}

#[macro_export]
macro_rules! t_f64_default {
    ($params:expr, $field:expr, $default:expr) => ({
        match $params.get(&$field) {
            Some(&Value::I64(int)) => int as f64,
            Some(&Value::U64(int)) => int as f64,
            Some(&Value::F64(ft)) => ft,
            _ =>  _using_default! ($field, $default)
        }
    })
}

#[macro_export]
macro_rules! t_bool {
    ($params:expr, $field:expr) => ({
        match $params.get(&$field) {
            Some(&Value::Boolean(ref tof)) => *tof,
            _ =>  _missing_or_unrecognized! ($field)
        }
    })
}

#[macro_export]
macro_rules! t_bool_default {
    ($params:expr, $field:expr, $default:expr) => ({
        match $params.get(&$field) {
            Some(&Value::Boolean(ref tof)) => *tof,
            _ =>  _using_default! ($field, $default)
        }
    })
}

#[macro_export]
macro_rules! t_map {
    ($params:expr, $field:expr) => ({
        match $params.get(&$field) {
            Some(&Value::Map(ref map)) => map,
            _ =>  _missing_or_unrecognized! ($field)
        }
    })
}

#[macro_export]
macro_rules! t_map_default {
    ($params:expr, $field:expr, $default:expr) => ({
        match $params.get(&$field) {
            Some(&Value::Map(ref map)) => map,
            _ =>  _using_default! ($field, $default)
        }
    })
}

#[macro_export]
macro_rules! t_array {
    ($params:expr, $field:expr) => ({
        match $params.get(&$field) {
            Some(&Value::Array(ref array)) => array,
            _ =>  _missing_or_unrecognized! ($field)
        }
    })
}

#[macro_export]
macro_rules! t_array_default {
    ($params:expr, $field:expr, $default:expr) => ({
        match $params.get(&$field) {
            Some(&Value::Array(ref array)) => array,
            _ =>  _using_default! ($field, $default)
        }
    })
}



#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
