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

// - Map-设置

// - Map-长度

// - Map-读取

// - Map-删除

// - Map-遍历

// - 结构-定义

// - 结构-获取属性值

// - 正则表达式-定义

// - 正则表达式-是否匹配

// - 正则表达式-匹配结果

// - 循环-for

// - 循环-while

// - 选择-switch

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