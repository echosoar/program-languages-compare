// - 变量-单个变量
const a: number= 1;
let b: string = 'str';
var c: boolean = false;
// - 变量-多个变量
let d: number = 1, e: number = 2;
const f: number = 1, g: number = 2;
// - 注释
// 单行注释
/* 多行注释 */ 
// - 基本类型-字符串
let str: string = 'str';
// - 基本类型-数字
let num: number = 123;
// - 基本类型-浮点数
let float: number = 1.23;
// - 基本类型-布尔值
const bf: boolean = false;
// - 数组-定义
let arr: Array<number> = [1, 2];
// - 数组-获取长度
arr.length === 2
// - 数组-遍历
arr.forEach((item: number, i: number) => {
  // do something
});
arr.map((item: number, i: number) => {
  return item;
});
// - 数组-截取
arr.slice(0, 1)
// - 数组-连接
var strArr: string[] = ['a', 'b'];
strArr.join(',')
// - Map-创建
let map: Map<string, string> = new Map()
// - Map-设置
map = map.set('a', 'b');
// - Map-读取
const mapValue: string = map.get('a');
// - Map-长度
const mapalen = map.size;
// - Map-删除
map.delete('a');
// - Map-遍历
for (let [key, value] of map) {
   // do something
}
// - Object-定义
interface Obj {
  name: number;
};
let obj: Obj = {
  name: 123
};
// - Object-获取属性值
const objKeys: string[] = Object.keys(obj);

// - 正则表达式-定义
const reg: RegExp = /a(b)/i;
// - 正则表达式-是否匹配
const isMatch: boolean = reg.test('A');
// - 正则表达式-匹配结果
let matchRes = reg.exec('abcd');
let [match, capture1] = matchRes;
// match === 'ab'
// capture1 === 'b'
let matchIndex: number = matchRes.index;
// matchIndex === 0

// - 条件
if (true) {
} else if (true) {
} else { }
// - 循环-for
// 普通for循环
for(let i = 0; i<10; i++) { }
// 仅判断
for(; Math.random()<0.5;) {}
// 无限循环
for(;;) {}
// 遍历索引
for(let index in arr) { }
// 遍历元素
for(let value of arr) {}
// - 循环-while
while(Math.random()<0.5) {}
// - 选择-switch
switch(value) {
  case 1:
    break; // 退出需要break
  case 2:
  case 3:
    break;
}
// - 错误捕获
try {
  throw new Error('msg');
} catch(err) {
  // 错误处理
}

// - 方法-普通方法
function sum(a: number, b: number) {
  return a + b;
}
// - 方法-匿名方法
let sumFun = (a: number, b: number) => {
  return a + b;
}
// - 方法-IIFE
((a: number, b: number) => {
  return a + b;
})(1, 2);

// - 类-声明
class A {
  private a: number;
  constructor(a: number) {
    this.a = a;
  }
  public method() {}
  private log() {}
}
// - 类-继承
class B extends A {
  public method() {
    super.method();
  }
}
// - 类-实例化
let instance: A = new A(123);

// - JSON-定义
`{
  "number": 1,
  "boolean": false,
  "string": "str",
  "arr": [
      12,
      "abc"
  ],
  "obj": {
      "name": "name",
      "age": 12
  }
}`
// - JSON-编码
const jsonStr: string = JSON.stringify({
  name: 123
});
// - JSON-解码
const jsonObj = JSON.parse('{"name": 123}');

// - 时间-当前时间戳
const now: number = Date.now();
// - 时间-获取时间
const date = new Date();
date.getFullYear(); // 年
date.getMonth() + 1; // 月
date.getDate(); // 日
date.getHours(); // 时
date.getMinutes(); // 分
date.getSeconds(); // 秒

// - 系统变量-当前代码文件
__filename
// - 系统变量-当前代码目录
__dirname
// - 系统变量-进程启动路径
process.cwd()
// - 系统变量-环境变量
process.env.ENV_NAME
// - 系统变量-启动时入参
process.argv
// - 系统变量-当前进程id
process.pid
// - 系统变量-退出进程
process.exit(signal)
// - 系统变量-输出
console.log