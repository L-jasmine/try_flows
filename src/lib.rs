use http_req::uri::Uri;
use lambda_flows::{request_received, send_response};

#[no_mangle]
pub extern "C" fn run() {
    request_received(|_qry, _body| {
        let mut writer = Vec::new();

        let uri = Uri::try_from("https://api.bilibili.com/x/polymer/web-dynamic/v1/feed/space?offset=&host_mid=401742377&timezone_offset=-480&features=itemOpusStyle").unwrap();
        let req = http_req::request::Request::new(&uri)
            .header("accept", "application/json, text/plain, */*")
            .header("authority", "api.bilibili.com")
            .header("origin", "https://space.bilibili.com")
            .header("referer", "https://space.bilibili.com/401742377/dynamic")
            .send(&mut writer)
            .unwrap();

        send_response(
            req.status_code().into(),
            vec![(String::from("content-type"), String::from("text/html"))],
            writer,
        );
    });
}

// curl 'https://api.bilibili.com/x/polymer/web-dynamic/v1/feed/space?offset=&host_mid=401742377&timezone_offset=-480&features=itemOpusStyle' \
//   -H 'authority: api.bilibili.com' \
//   -H 'accept: application/json, text/plain, */*' \
//   -H 'accept-language: zh-CN,zh;q=0.9,en;q=0.8,en-GB;q=0.7,en-US;q=0.6' \
//   -H $'cookie: buvid3=0F98C18A-B708-DF89-68F6-FA3C45C61A1439119infoc; i-wanna-go-back=-1; _uuid=7610C2C63-C24E-2123-E8E6-105DB144106F5C38691infoc; buvid4=D02574E7-4E95-2E17-E312-292AEF92D71B40439-022080511-vnIKVQMoNN41k8q37RVwXw%3D%3D; buvid_fp_plain=undefined; DedeUserID=16856350; DedeUserID__ckMd5=59f1d8365143ac66; CURRENT_BLACKGAP=0; b_ut=5; LIVE_BUVID=AUTO5316597682657388; b_nut=100; nostalgia_conf=-1; hit-dyn-v2=1; is-2022-channel=1; hit-new-style-dyn=0; rpdid=|(k)~l)R)Yu)0J\'uYY)Y|)|~k; CURRENT_QUALITY=0; bp_article_offset_16856350=762394718617206800; header_theme_version=CLOSE; home_feed_column=4; PVID=1; CURRENT_FNVAL=16; innersign=0; SESSDATA=09002572%2C1693216312%2C1cea8%2A32; bili_jct=25dba5f354dd0abb1cdddfb71510b421; sid=5663rpf6; bp_video_offset_16856350=768051053082968000; fingerprint=eec9b3c849cc0d8f118d596820ecee3a; buvid_fp=eec9b3c849cc0d8f118d596820ecee3a; b_lsid=46A1D2BB_1869CE8389A' \
//   -H 'origin: https://space.bilibili.com' \
//   -H 'referer: https://space.bilibili.com/401742377/dynamic' \
//   -H 'sec-ch-ua: "Chromium";v="110", "Not A(Brand";v="24", "Microsoft Edge";v="110"' \
//   -H 'sec-ch-ua-mobile: ?0' \
//   -H 'sec-ch-ua-platform: "Windows"' \
//   -H 'sec-fetch-dest: empty' \
//   -H 'sec-fetch-mode: cors' \
//   -H 'sec-fetch-site: same-site' \
//   -H 'user-agent: Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/110.0.0.0 Safari/537.36 Edg/110.0.1587.57' \
//   --compressed
