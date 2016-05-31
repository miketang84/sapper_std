extern crate sapper;
extern crate serde;
extern crate serde_json;

#[macro_export]
macro_rules! res_json {
    ($json:expr) => ({
        use sapper::Response;
        use serde_json;

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
