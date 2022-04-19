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
for index in 0..2 {
    println!("index is: {} & value is : {}",index, arr[index]);
}
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
map.get(&"key".to_string());
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

// - 正则表达式-是否匹配

// - 正则表达式-匹配结果
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

// - 方法-匿名方法

// - 方法-IIFE

// - 类-声明

// - 类-继承

// - 类-实例化

// - JSON-定义

// - JSON-编码

// - JSON-解码

// - 时间-当前时间戳

// - 时间-获取时间

// - 系统变量-当前代码文件

// - 系统变量-当前代码目录

// - 系统变量-进程启动路径

// - 系统变量-环境变量

// - 系统变量-启动时入参

// - 系统变量-当前进程id

// - 系统变量-退出进程

// - 系统变量-输出