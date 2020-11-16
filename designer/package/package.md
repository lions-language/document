## libpackage
### 职责
- 查看包是否编译过
	- 编译过
		- 读取给定包路径中的编译结果
	- 未编译过
		- 编译给定路径中的包
- 不管是读编译好的文件, 还是编译包, 都需要将结果写入到 compile 的 PackageControl 中
- 注意(NOTE)
	- libpackage 不会对版本进行检测, 只会根据配置解析
