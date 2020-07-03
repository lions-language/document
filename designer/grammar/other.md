## 便于操作的封装
### expect_next_token
- 功能
	- 期待下一个 token, 如果遇到了结束, 将报错
	- 通过回调给出 next token, 用户决定是否需要报错
	- 正确获取到 next token 之后还会返回 next token
- 代码
```rust
165     fn expect_next_token<F>(&mut self, mut f: F, token_prompt: &'static str) -> Option<TokenPointer>
166         /*
167          * 注意: 该方法会先去除全部的空白
168          * */
169         where F: FnMut(&mut GrammarParser<T, CB>, TokenPointer) {
170         let tp = match self.skip_white_space_token() {
171             Some(tp) => {
172                 tp
173             },
174             None => {
175                 /*
176                  * 期望下一个 token, 但是遇到了 IO EOF => 语法错误
177                  * */
178                 self.panic(&format!("expect {}, but arrive IO EOF", token_prompt));
179                 return None;
180             }
181         };
182         /*
183          * TokenPointer 中的 clone, 只是 地址的拷贝
184          * */
185         f(self, tp.clone());
186         Some(tp)
187     }
```
- 上面的实现方式, 语法上没问题, 但是存在效率问题, 使用这种方式创建, 每次函数调用都需要一个闭包的环境, 这将消耗内存(闭包需要捕获外部变量, 导致内存消耗)
- 其实我们很多时候并不需要外部环境, 所以应该改为函数指针
	- rust这一点做的很好, 当函数的参数是函数指针的时候, 调用的时候也可以向闭包的语法一样使用: || {}, (lions-language也会这样实现), 这样的话, 只需要把闭包改为函数指针就行了, 调用的时候一样非常的简洁
