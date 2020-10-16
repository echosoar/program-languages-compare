const { readdirSync, readFileSync, writeFileSync } = require('fs');
const { join, extname } = require('path');
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
          { class: '字符串'},
          { class: '数字'},
          { class: '布尔值'},
        ]
      },
      {
        class: '数组',
        child: [
          { class: '定义'},
          { class: '获取长度'},
          { class: '遍历'},
          { class: '截取'},
          { class: '连接'},
        ]
      },
      {
        class: 'Map',
        child: [
          { class: '创建'},
          { class: '设置'},
          { class: '长度'},
          { class: '读取'},
          { class: '删除'},
          { class: '遍历'},
        ]
      },
      {
        class: 'Object',
        child: [
          { class: '定义'},
          { class: '获取属性值'},
        ]
      },
      {
        class: '正则表达式',
        child: [
          { class: '定义'},
          { class: '是否匹配'},
          { class: '匹配结果'},
        ]
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
          { class: 'for'},
          { class: 'while'},
        ]
      },
      {
        class: '选择',
        child: [
          { class: 'switch'}
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
const languageMap = {
  ts: {
    lang: 'TypeScript',
  },
  cpp: {
    lang: 'C++',
  },
  go: {
    lang: 'Golang'
  }
};
class Gen {
  data;
  maxClass = 3;
  classWidth = 100;
  constructor() {
    this.main();
  }

  main() {
    this.getData();
    this.render();
    this.renderToMd();
  }
  getData() {
    const dataPath = join(__dirname, 'data');
    const allData = readdirSync(dataPath);
    this.data = [];
    for(const fileName of allData) {
      const ext = extname(fileName).replace(/^\./, '');
      const info = languageMap[ext];
      if (!info) {
        return;
      }
      let code = readFileSync(join(dataPath, fileName)).toString();
      const codeList = code.split('\n');
      this.data.unshift({
        info,
        codeMap: this.formatCode(codeList)
      });
    }
  }

  stripHtml(code) {
    return code.replace(/</g, '&lt;').replace(/>/g, '&gt;');
  }

  formatCode(codeList) {
    let type = '';
    let typeReg = /^\/\/\s+-\s+(.*)\s*$/;
    let codeMap = {};
    codeList.forEach(code => {
      if (typeReg.test(code)) {
        type = typeReg.exec(code)[1];
        codeMap[type] = [{ content: []}];
        return;
      }
      if (!type) {
        return;
      }
      const last = codeMap[type][codeMap[type].length - 1];
      last.content.push(code);
    });
    return codeMap;
  }

  renderLang(lang, className) {
    if (!lang || !lang.codeMap || !lang.codeMap[className]) {
      return '';
    }
    return lang.codeMap[className].map(value => {
      return `<div class="code"><pre><code class="${lang.info.lang.toLowerCase()}">${ this.stripHtml(value.content.join('\n'))}</code></pre></div>`;
    }).join('');
  }

  renderRow(item, level, className) {
    if (!className) {
      className = [];
    }
    if (level > this.maxClass) {
      const classType = className.slice(1).join('-');
      return `<div class="line">${ this.data.map(lang => {
        return `<div class="lang">${ this.renderLang(lang, classType)}</div>`;
      }).join('') }</div>`;
    }
    const isChild = item.child && !!item.child.length;
    let style = '';
    if (!isChild) {
      const padding = (this.maxClass - level) * (this.classWidth + 1);
      style += `padding-right: ${padding}px`;
    }
    return `<div class="line">
      <div class="class" style="${style}">${ item.class || '' }</div>
      <div class="child">${ isChild ? item.child.map(child => {
        return this.renderRow(child, level + 1, className.concat(item.class));
      }).join('') : this.renderRow(null, this.maxClass + 1, className.concat(item.class)) }</div>
    </div>`
  }
  render() {
    const html = `
    <link rel="stylesheet" href="./styles/vs.css">
    <style>
    body {
      margin: 0;
    }
    pre {
      margin: 0;
    }
    .line {
      display: flex;
      font-size: 13px;
      line-height: 24px;
    }
    .headTitle {
      line-height: 36px;
      border: 1px solid #ccc;
      flex: 0 0 ${(this.classWidth + 1) * 3 - 1}px;
      text-align: center;
      background: #ac98d6;
      color: #fff;
      font-weight: bold;
    }
    .langTitle {
      line-height: 36px;
      border: 1px solid #ccc;
      border-left: 0;
      flex: 0 0  400px;
      text-align: center;
      background: #ac98d6;
      color: #fff;
      font-weight: bold;
    }
    .class {
      flex: 0 0 ${this.classWidth}px;
      position: relative;
      width: ${this.classWidth}px;
      display: flex;
      color: #666;
      align-items:  center;
      justify-content: center;
      border-left: 1px solid #ccc;
      border-bottom: 1px solid #ccc;
    }
    .lang {
      flex: 0 0  400px;
      width:  400px;
      overflow: auto;
      min-height: 24px;
      border-left: 1px solid #ccc;
      border-bottom: 1px solid #ccc;
    }
    .lang:last-child {
      border-right: 1px solid #ccc;
    }
    .copyright {
      font-size: 12px;
      padding: 24px;
      color: #999;
    }
    </style>
    <div class="container">
      <div class="line">
        <div class="headTitle">多语言速查表</div>
        ${ this.data.map(lang => {
          return `<div class="langTitle">${ lang.info.lang }</div>`;
        }).join('') }
      </div>
      ${
        table.map(item => {
          return this.renderRow(item, 1);
        }).join('')
      }
      <div class="copyright">Created by echosoar | Welcome star ❤ <a href="https://github.com/echosoar/program-languages-compare">echosoar/program-languages-compare</a> | <a href="./README.md" target="_blank">Markdown Version</a></div>
    </div>
    <script src="./highlight.pack.js"></script>
    <script>hljs.initHighlightingOnLoad();</script>
    `
    writeFileSync(join(__dirname, 'index.html'), html);
  }

  renderLangMd(lang, className) {
    if (!lang || !lang.codeMap || !lang.codeMap[className]) {
      return '';
    }
    return lang.codeMap[className].map(value => {
      const langLower = lang.info.lang.toLowerCase();
      return `\n##### ${langLower}\n\`\`\`${langLower}\n${ value.content.join('\n')}\n\`\`\`\n`;
    }).join('\n');
  }

  renderRowMd(item, level, className) {
    if (!className) {
      className = [];
    }
    if (level > this.maxClass) {
      const classType = className.slice(1).join('-');
      return this.data.map(lang => {
        return this.renderLangMd(lang, classType);
      }).join('');
    }
    const isChild = item.child && !!item.child.length;

    return `${'\n####'.slice(0, level + 2)} ${item.class || ''}\n${ isChild ? item.child.map(child => {
        return this.renderRowMd(child, level + 1, className.concat(item.class));
      }).join('\n') : this.renderRowMd(null, this.maxClass + 1, className.concat(item.class)) }`
  }
  renderToMd() {
    const md = `# 多语言速查表\n\n对比多种编程语言之间的异同\n\nCompare the similarities and differences between various programming languages\n\nOnline: [program-languages-compare](https://echosoar.github.io/program-languages-compare/)\n\n${
      table.map(item => {
        return this.renderRowMd(item, 1);
      }).join('\n')}`;
    writeFileSync(join(__dirname, 'README.md'), md);
  }
}

new Gen();