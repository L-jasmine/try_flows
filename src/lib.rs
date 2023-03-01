use lambda_flows::{request_received, send_response};

#[no_mangle]
pub extern "C" fn run() {
    request_received(|_qry, _body| {
        let a = String::new();

        send_response(
            200,
            vec![(String::from("content-type"), String::from("text/html"))],
            "ok".as_bytes().to_vec(),
        );
    });
}
