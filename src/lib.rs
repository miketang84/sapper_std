extern crate sapper;
extern crate sapper_tmpl;
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


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
