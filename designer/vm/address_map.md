## 地址映射
### 编译期
- 一个作用域一个地址分配器

### 运行期
- 遇到作用域开始, 就生成一个地址映射

### 编译期函数调用
- 准备好每个参数指向的数据的地址索引, 也就是函数调用时的作用域中的地址索引

### 函数定义中接收参数
- 函数定义所在的作用域中的地址分配从参数个数值开始, 如果函数参数中存在3个参数, 那么函数定义所在的作用域中的地址分配起始值就是3(从0开始)
