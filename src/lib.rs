extern crate sapper;
extern crate sapper_tmpl;
extern crate sapper_query_params;
extern crate sapper_body_params;
extern crate serde;
extern crate serde_json;

#[macro_export]
macro_rules! res_html {
    ($html:expr, $context:expr) => ({
        use sapper::Response;
	use sapper::header::ContentType;
	use sapper_tmpl::render;

        let res_str = render($html, $context);

        let mut response = Response::new();
        response.headers_mut().set(ContentType::html());
        response.write_body(res_str);

        Ok(response)
    })
}

#[macro_export]
macro_rules! res_redirect {
    ($redirect_uri:expr) => ({
        use sapper::Response;
        use sapper::status;
	use sapper::header::Location;

        let mut response = Response::new();
        response.set_status(status::Found);
        response.headers_mut().set(Location($redirect_uri.to_owned()));

        Ok(response)
    })
}


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
        use serde_json::builder::ObjectBuilder;

        let json2ret = ObjectBuilder::new()
            .insert("success", true)
            .insert("info", $info)
            .unwrap();

        res_json!(json2ret)
    })
}

#[macro_export]
macro_rules! res_json_error {
    ($info:expr) => ({
        use sapper::Response;
        use serde_json;
        use serde_json::builder::ObjectBuilder;

        let json2ret = ObjectBuilder::new()
            .insert("success", false)
            .insert("info", $info)
            .unwrap();

        res_json!(json2ret)
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

    })
}

#[macro_export]
macro_rules! get_params {
    ($req:expr, $tykey:ty) => ({
        match $req.ext().get::<$tykey>() {
            Some(params) => {
                params
            },
            None => return res_400!("no params")
        };
    })
}


#[macro_export]
macro_rules! get_query_params {
    ($req:expr) => ({
        use sapper_query_params::ReqQueryParams;

        get_params!($req, ReqQueryParams)
    })
}

#[macro_export]
macro_rules! get_body_params {
    ($req:expr) => ({
        use sapper_body_params::ReqBodyParams;

        get_params!($req, ReqBodyParams)
    })
}

#[macro_export]
macro_rules! get_json_params {
    ($req:expr) => ({
        use sapper_body_params::ReqJsonParams;

        get_params!($req, ReqJsonParams)
    })
}

#[macro_export]
macro_rules! get_path_params {
    ($req:expr) => ({
        use sapper::ReqPathParams;

        get_params!($req, ReqPathParams)
    })
}


#[macro_export]
macro_rules! check_param {
    ($params:expr, $key:expr) => ({
        match $params.get($key) {
            Some(ref param) => {
                &param[0]
            },
            None => {
                res_400!(format!("no {} value in params", $key))
            }
        }
    })
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
