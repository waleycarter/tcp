use std::io::Read;
use std::io::Write;
use std::net::TcpStream;
use std::net::TcpListener;
use std::fs;

fn main() {
    //在本地7878端口创建tcp链接
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    //读取正常响应文件 resp_html
    let resp_html = fs::read_to_string("hello.html").unwrap();
    //生成正常返回信息字符串
    let success_resp = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
        resp_html.len(),
        resp_html
    );

    //读取异常相应文件 resp_html
    let resp_html = fs::read_to_string("error.html").unwrap();
    //生成异常相应字符串
    let err_resp = format!(
        "HTTP/1.1 404 NOTFOUND\r\nContent-Length:{}\r\n\r\n{}",
        resp_html.len(),
        resp_html
    );

    //监听tcp链接
    for stream in listener.incoming(){
        //获取tcpstream
        let mut stream = stream.unwrap();
        //打印请求信息
        let req = print_request(&stream);
        //通过请求信息判断http请求
        let result = handle_http_method(req.as_str());
        //生成相应信息
        let resp = match result{
            Ok(m) => {
                println!("request method:{}", m.as_str());
                success_resp.as_str()
            },
            Err(_) => err_resp.as_str()
        };
        //将相应信息写入流
        stream.write(resp.as_bytes()).unwrap();
        //刷新流
        stream.flush().unwrap();
    }
}

fn print_request(mut stream: &TcpStream) -> String{
    //在栈上声明一个buffer来存放读取到的数据， 创建缓冲区大小为512字节
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();
    //将缓冲区字节转换为字符串
    let content = String::from_utf8_lossy(&buffer);
    //打印请求内容
    print!("{}", content);
    //返回请求内容
    content.to_string()
}
fn handle_http_method(content: &str) -> Result<HttpMethod, &str> {
    //根据请求内容判断http请求method 
    if content.starts_with("GET"){
        Result::Ok(HttpMethod::GET)//正确则返回ok get枚举
    } else if content.starts_with("POST"){
        Result::Ok(HttpMethod::POST)//正确返回ok post枚举
    } else {
        Result::Err("http method not support")//非 get post 返回错误信息err枚举
    }
}

enum HttpMethod {
    GET,
    POST
}

impl HttpMethod{
    //实现as_str方法便于打印
    pub(crate) fn as_str(&self) -> &'static str {
        match *self {
            HttpMethod::GET => "GET",
            HttpMethod::POST => "POST",
        }
    }
}