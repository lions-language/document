## 为函数调用做准备工作
- 在编译期, 将函数调用之前需要的参数放入到栈中
- 编译期, 分析出返回值应该存储的位置

## 编译期存储
- 为每一个实际存储(叶子节点)创建一个引用计数
- 如果参数是 Move, 将相应叶子节点的引用计数值设置为0
	- 检测返回值信息
		- 如果返回值是 Move, 取出 move 的索引, 对相应索引指向的叶子节点的引用计数+1
		- 如果返回值是 Create, 说明被移动的参数在函数结束后, 生命周期结束, 所以在函数结束后, 会因为引用计数为0, 而被释放; 同时, 需要在数据区分配一个地址 (并放入引用技术容器中)
- 如果参数是 Move, 所有权交由函数内部, 外部无权释放, 外部要做的就是在调用函数之前, 将实际的地址拿出来给函数, 如果要释放, 函数内部会将给定的地址释放掉
- 如果返回值是一个新创建的值, 那么所有权就交由调用方, 等到调用方相关作用域结束后, 被释放
- 如果返回值不是 Move, 需要检测拥有所有权的变量, 然后依次释放

## 作用域
- 进入作用域
	- 记录进入时的地址值 start_addr
- 出作用域
	- 存在返回值
		- 将返回值存储在 start_addr 中
		- 除了返回值的内存, 其他都需要释放
	- 不存在返回值
		- 将地址值设置为 start_addr
		- 释放作用域内的所有内存
