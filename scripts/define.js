const languageMap = {
    ts: {
      lang: 'TypeScript',
      play: 'https://www.typescriptlang.org/play',
    },
    go: {
      lang: 'Golang',
      play: 'https://go.dev/play/',
    },
    rs: {
      lang: 'Rust',
      play: 'https://play.rust-lang.org/',
    },
    cpp: {
      lang: 'C++',
      play: 'https://www.onlinegdb.com/online_c++_compiler',
    }
  };
const table = [
    {
        class: '基本概念',
        child: [
            {
                class: '变量',
                child: [
                    { class: '单个变量' },
                    { class: '多个变量' }
                ]
            },
            {
                class: '注释'
            }
        ]
    },
    {
        class: '类型',
        child: [
            {
                class: '基本类型',
                child: [
                    { class: '字符串' },
                    { class: '数字' },
                    { class: '布尔值' },
                ]
            },
            {
                class: '数组',
                child: [
                    { class: '定义' },
                    { class: '获取长度' },
                    { class: '遍历' },
                    { class: '截取' },
                    { class: '连接' },
                ]
            },
            {
                class: 'Map',
                child: [
                    { class: '创建' },
                    { class: '设置' },
                    { class: '长度' },
                    { class: '读取' },
                    { class: '删除' },
                    { class: '遍历' },
                ]
            },
            {
                class: '结构',
                child: [
                    { class: '定义' },
                    { class: '获取属性值' },
                ]
            },
            {
                class: '正则表达式',
                child: [
                    { class: '定义' },
                    { class: '是否匹配' },
                    { class: '匹配结果' },
                ]
            },
            {
                class: '枚举'
            }

        ]
    },
    {
        class: '语句',
        child: [
            {
                class: '条件'
            },
            {
                class: '循环',
                child: [
                    { class: 'for' },
                    { class: 'while' },
                ]
            },
            {
                class: '选择',
                child: [
                    { class: 'switch' }
                ]
            },
            {
                class: '错误捕获'
            }
        ]
    },
    {
        class: '方法与类',
        child: [
            {
                class: '方法',
                child: [
                    { class: '普通方法' },
                    { class: '匿名方法' },
                    { class: 'IIFE' },
                ]
            },
            {
                class: '类',
                child: [
                    { class: '声明' },
                    { class: '继承' },
                    { class: '实例化' },
                ]
            }
        ]
    },
    {
        class: '数据处理',
        child: [
            {
                class: 'JSON',
                child: [
                    { class: '定义' },
                    { class: '编码' },
                    { class: '解码' },
                ]
            },
            {
                class: '时间',
                child: [
                    { class: '当前时间戳' },
                    { class: '获取时间' }
                ]
            },
            {
                class: '系统变量',
                child: [
                    { class: '当前代码文件' },
                    { class: '当前代码目录' },
                    { class: '进程启动路径' },
                    { class: '环境变量' },
                    { class: '启动时入参' },
                    { class: '当前进程id' },
                    { class: '退出进程' },
                    { class: '输出' },
                ]
            }
        ]
    }
]

module.exports = {
    table,
    languageMap,
}