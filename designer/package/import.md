## 包
### 定义
- 就是 lions-language 的一个代码仓库
- 可以是用于运行的, 也可以是直接提供功能的 lib 包


## 导入规则
### import 关键字导入, 后面加双引号
- import "内容前缀:xxx"

### 内容前缀
- packages: 表示的是第三方包 (在 toml 配置中定义的依赖)
- system: 系统包
- path: 后面跟本地路径
- local: 当前包 (从当前包的根目录开始计算)


## 路径查找规则
- 指定的路径一定是一个目录, 如果不存在 或者 存在但是不是一个目录 都会报错
- 如果指定的路径目录中不存在 mod.lions, 就视为错误


## 模块组织规则
### 基本规则
- 每个模块下面应该有一个 mod.lions 文件, 该文件中必须指明当前模块的名称
    - module xxx
- 和 mod.lions 文件在同一级的所有文件都视为同一个模块
- 在 mod.lions 中可以指定当前目录的 lions 文件, 被指定的文件将会被编译
- 存在 mod.lions 的目录在被指定的情况下(import 指定, 或者 与 main.lions 在同一个目录下, 或者 配置文件指定是库 的包), 一定会编译 其中指定的 lions 文件, 也就是说:
    - 如果 Lions.toml 中配置包为 lib, 那么 最外层(src/) 一定有一个 mod.lions 文件
    - 如果 Lions.toml 中配置包为 binary, 那么 最外层中如果存在 mod.lions 文件, 就会编译指定的lions(不包括 main.lions); 否则, 只会编译 main.lions

### 相关模块
- 考虑到模块文件过多, 放在一个目录中过于杂乱, 引入 相关模块 的概念
- 关键字 "relmod", 指定相关模块的相对当前文件的路径
    - relmod "xxx"
- 相关模块指定的目录不能存在 mod.lions
- relmod 只能出现在 mod.lions 文件中
