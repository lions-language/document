## libpackage
### 职责
- 根据配置文件, 决定是否编译 (存在一个 是否编译 的选项)
	- 不需要编译
		- 读取给定包路径中的编译结果; 如果不存在编译结果, 就需要报错
	- 需要编译
		- 编译给定路径中的包, 并将编译结果写入配置文件到指定的输出目录中
- 不管是读编译好的文件, 还是编译包, 都需要将结果写入到 compile 的 PackageControl 中
- 注意(NOTE)
	- libpackage 不会对版本进行检测, 只会根据配置解析

### lib包的依赖
- 每个 lib包 需要将自身的依赖都记录下来
- 最后和 main 链接的时候使用需要将所有的链接都链接起来


## 包工具
### 职责
- 检测是否需要编译
	- 文件的时间戳
- 检测版本