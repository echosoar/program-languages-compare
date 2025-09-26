// - 变量-单个变量
// a 为 不可变变量，即不能改变值
let a: u64 = 1;
// 不可变变量可以重新定义
// rust 支持自动类型推导
let a = 2;
// 使用 mut 关键词，使变量可变
let mut ma = 1;
// ma = "abc"; 报错，类型错误
// ma = 1.2; 报错，类型错误，精度损失
const b = "str"
let c = true;
let d: bool = false;

// - 变量-多个变量
let (a, b, c) = (42, "hello", 5.0);
// - 注释
// 单行注释
/// 三个反斜也可以
/* 多行注释 */ 
// - 基本类型-字符串
// 字符
let char1: char = 'a';
let char2 = 'a';
// 字符串
let str1 = "string";
let str1: &str = "string";
let mut str2 = String::new();
str2 = str1.to_string();
// " 与 ' 都需要使用 \ 转义
let str1 = "\"test\"";
// 如果不想转义，可以使用 原始字符串
let str1 = r#"sam said: "xxx"."#
// 如果原始字符串中出现 #，两侧需要更多的 #
let str1 = r###"##"###;
// - 基本类型-数字
let num = 123; // i32
let num64: i64 = 123;
let float64 = 2.0; // f64
let float32: f32 = 3.0; // f32
// - 基本类型-布尔值
let c = true;
let d: bool = false;
// - 数组-定义
let arr = [1, 2];
// [类型;数量]
let arr: [i32;2] = [1, 2];
// 都初始化为 -1
let arr: [i32;2] = [-1;2];
// 可变长数组 vector
let varr: Vec<i32> = Vec::new();
// rust 提供了宏 vec! 根据值创建 vector
ler mut varr = vec![1,2];
// - 数组-获取长度
arr.len() == 2
// - 数组-遍历
// 迭代值
for val in arr.iter(){
    println!("value is :{}", val);
}
// 迭代索引
for index in 0..arr.len() {
    println!("index is: {} & value is : {}",index, arr[index]);
}
// 迭代值和索引
for (index, value) in arr.iter().enumerate() {
    println!("Index: {}, Value: {}", index, value);
}
// 内联
let index_value_pairs: Vec<String> = arr
        .iter()
        .enumerate()
        .map(|(index, value)| format!("Index: {}, Value: {}", index, value))
        .collect();
// 遍历 vector 数组
for i in &mut varr {
    println!("{}", i);
}
// - 数组-截取
&arr[0..1];
// 包含结尾元素需要加 =
&arr[0..=0];
// - 数组-连接
// "hello world"
["hello", "world"].join(" ");
// [1, 2, 0, 3, 4]
[[1, 2], [3, 4]].join(&0)
// [1, 2, 3, 4]
[[1, 2], [3, 4]].join(&[][..])

// Vec<String> Join(&str)
let va = vec!["1","2","3"];
// 1-2-3
va.join("-");
// - Map-创建
use std::collections::HashMap;
let mut map = HashMap::new();
// 或者也可以通过 array 来初始化
let mut map2 = HashMap::from([
    ("key1", "value1"),
    ("key2", "value2"),
])
// - Map-设置
map.insert(
    "key".to_string(),
    "value"
);
// - Map-长度
map.len();
// - Map-读取
// get 的参数类型为 &Q
if let Some(value) = map.get(&"key".to_string()) {
    // do something
}
map["key"];
map[&"key".to_string()];
// - Map-删除
map.remove(&"key".to_string());
// - Map-遍历
for (key, value) in &map {
    println!("{}: \"{}\"", key, value);
}
// - 结构-定义
struct StructItem {
    str: String,
    i: u32
}
let str = String::from("123");
let s = StructItem {
    str,
    i: 123
}
let s2 = StructItem {
    i: 124,
    ..s
}
// 元祖结构体
struct T1(u32, u32)；
let t = T1(1,2);
// - 结构-获取属性值
s2.str; // 123
t.1; // 2
// - 正则表达式-定义
// 需要添加依赖 regex = "1"
use regex::Regex;
let reg = Regex::new(r"(a|b)").unwrap();

// - 正则表达式-是否匹配
let is_match = reg.is_match("A");

// - 正则表达式-匹配结果
let text = "abcd";
if let Some(caps) = reg.captures(text) {
    let full_match = &caps[0]; // "ab"
    let capture1 = &caps[1]; // "a"
}
// - 枚举
enum Color {
    Red,
    Green,
    Blue,
}
// 带值的枚举
enum Status {
    Success = 0,
    Error = 1,
}
let color = Color::Red;
let status = Status::Success;

// - 条件
// 条件可以忽略小括号
if a > 0 {
} else if (a < 0) {
} else {
}
// 还能赋值，等同三元运算符
let b = if (a > 0) { 1 } else { 0 }
// - 循环-for
let a = [10, 20, 30, 40, 50];
for i in 0..5 {
    println!("a[{}] = {}", i, a[i]);
}
for i in a.iter() {
    println!("a[{}] = {}", i, a[i]);
}
// - 循环-while
let mut number = 1;
while number != 4 {
    number += 1;
}
// 无限循环

let mut i = 0;
// 支持 break 出来的值赋值
let num = loop {
    if i == 10 {
        break i;
    }
    i += 1;
};
num == 10;
// - 选择-switch
let num = 3;
match num {
    1|2=>println!("<3"),
    3=>println!("=3"),
    4..=20=> println!("4<= x <= 20"),
    _=>println!("other"),
}
// - 方法-普通方法
pub fn sum(a: i32, b: i32) -> i32 {
    a + b
}
// - 方法-匿名方法
// 闭包
let sum = |x: i32, y: i32| -> i32 {
    x + y
};
// - 方法-IIFE
// Rust doesn't have traditional IIFE, but can use closures
let result = (|a: i32, b: i32| -> i32 {
    a + b
})(1, 2);

// - 错误捕获
// Rust uses Result<T, E> instead of exceptions
fn might_fail() -> Result<i32, String> {
    if true {
        Ok(42)
    } else {
        Err("Something went wrong".to_string())
    }
}

match might_fail() {
    Ok(value) => println!("Success: {}", value),
    Err(e) => println!("Error: {}", e),
}

// - 类-声明
struct Person {
    name: String,
    age: i32,
}

impl Person {
    fn new(name: String, age: i32) -> Person {
        Person { name, age }
    }
    
    fn greet(&self) {
        println!("Hello, I'm {}", self.name);
    }
}

// - 类-继承
// Rust doesn't have traditional inheritance, uses traits instead
trait Animal {
    fn make_sound(&self);
}

struct Dog {
    name: String,
}

impl Animal for Dog {
    fn make_sound(&self) {
        println!("Woof!");
    }
}

// - 类-实例化
let person = Person::new("Alice".to_string(), 30);
let dog = Dog { name: "Buddy".to_string() };

// - JSON-定义
// 需要添加依赖 serde = { version = "1.0", features = ["derive"] }
// serde_json = "1.0"
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
struct JsonData {
    name: String,
    values: Vec<i32>,
    obj: JsonObject,
}

#[derive(Serialize, Deserialize)]
struct JsonObject {
    name: String,
    age: i32,
}

// - JSON-编码
use serde_json;
let data = JsonData {
    name: "example".to_string(),
    values: vec![1, 2, 3],
    obj: JsonObject {
        name: "name".to_string(),
        age: 12,
    },
};
let json_str = serde_json::to_string(&data).unwrap();

// - JSON-解码
let json_str = r#"{"name": 123}"#;
let value: serde_json::Value = serde_json::from_str(json_str).unwrap();

// - 时间-当前时间戳
use std::time::{SystemTime, UNIX_EPOCH};
let timestamp = SystemTime::now()
    .duration_since(UNIX_EPOCH)
    .unwrap()
    .as_millis();

// - 时间-获取时间
use chrono::{DateTime, Local};
let now: DateTime<Local> = Local::now();
let year = now.year();
let month = now.month();
let day = now.day();
let hour = now.hour();
let minute = now.minute();
let second = now.second();

// - 系统变量-当前代码文件
file!()

// - 系统变量-当前代码目录
// 需要 std::env
use std::env;
let current_dir = env::current_dir().unwrap();

// - 系统变量-进程启动路径
let current_dir = env::current_dir().unwrap();

// - 系统变量-环境变量
let env_var = env::var("ENV_NAME").unwrap_or_default();

// - 系统变量-启动时入参
use std::env;
let args: Vec<String> = env::args().collect();

// - 系统变量-当前进程id
use std::process;
let pid = process::id();

// - 系统变量-退出进程
process::exit(0);

// - 系统变量-输出
println!("value");
print!("value");
eprintln!("error message");