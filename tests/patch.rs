extern crate restson;

#[macro_use]
extern crate serde_derive;

use restson::{RestClient,RestPath,Error};

#[derive(Serialize,Deserialize)]
struct HttpBinPatch {
    data: String,
}

impl RestPath<()> for HttpBinPatch {
    fn get_path(_: ()) -> Result<String,Error> { Ok(String::from("patch")) }
}

#[test]
fn basic_patch() {
    let mut client = RestClient::new("http://httpbin.org").unwrap();

    let data = HttpBinPatch { data: String::from("test data")};
    client.patch((), &data).unwrap();
}

#[test]
fn patch_query_params() {
    let mut client = RestClient::new("http://httpbin.org").unwrap();

    let params = vec![("a","2"), ("b","abcd")];
    let data = HttpBinPatch { data: String::from("test data")};
    client.patch_with((), &data, &params).unwrap();
}
