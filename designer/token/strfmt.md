## strfmt
### 描述
- 在字符串中引用表达式
- 默认方式 **strfmt"${表达式}"**
	```lions-language
	strfmt"hello ${name}"
	```
- 如果字符串中存在 *${* 和 *}*, 可以指定起始串和终结串
	- **strfmt<"起始字符串"><"终结字符串">"[起始字符串]表达式[终止字符串]"**
	```lions-language
	strfmt<"{"><"}">"hello {name}"
	```
- 在词法解析阶段, 应该将strfmt拆解, 如: strfmt"hello ${name}" 拆解后:
	- Str("hello")
	- Plus
	- LeftParenthese
	- Id(name)
	- RightParenthese
- 注意: 需要使用 括号将表达式括起来, 否则会导致优先级错误问题

### 实现
- 实质上第一种方式只是第二种方式的一种特殊情况, 两者的区别就是 起始串 和 终结串 的不同
- 那么总结起来就是: 输入一个起始串和一个终结串, 分解字符串和表达式
```rust
  6     fn id_kw_strfmt_process_content(&mut self, start: &[u8], end: &[u8]) {
  7         // 跳过 "
  8         self.content.skip_next_one();
  9         enum Status {
 10             DoubleQuotes,
 11             EndSymbol
 12         }
 13         enum FindEndStatus {
 14             Finding,
 15             NotFound
 16         }
 17         let mut content = Vec::new();
 18         // let mut buffer = Vec::new();
 19         let mut status = Status::DoubleQuotes;
 20         let mut start_u8_array_is_equal = U8ArrayIsEqual::new(start);
 21         let mut end_u8_array_is_equal = U8ArrayIsEqual::new(end);
 22         let mut find_end_status = FindEndStatus::Finding;
 23         loop {
 24             match self.content.lookup_next_one() {
 25                 Some(c) => {
 26                     match status {
 27                         Status::DoubleQuotes => {
 28                             match c {
 29                                 '"' => {
 30                                     self.content.skip_next_one();
 31                                     self.push_nofunction_token_to_token_buffer(TokenType::Str(content.clone()));
 32                                     break;
 33                                 },
 34                                 _ => {
 35                                     if self.input_str_match_with_u8arrayisequal(&mut start_u8_array_is_equal) {
 36                                         // match start
 37                                         status = Status::EndSymbol;
 38                                         self.push_nofunction_token_to_token_buffer(TokenType::Str(content.clone()));
 39                                         // 模拟生成 token => ... + (...) + ...
 40                                         content.clear();
 41                                         self.push_token_plus();
 42                                         self.push_token_left_parenthese();
 43                                     } else {
 44                                         self.new_line_check(c);
 45                                         content.push(c as u8);
 46                                         self.content.skip_next_one();
 47                                     }
 48                                 }
 49                             }
 50                         },
 51                         Status::EndSymbol => {
 52                             if self.input_str_match_with_u8arrayisequal(&mut end_u8_array_is_equal) {
 53                                 status = Status::DoubleQuotes;
 54                                 self.push_token_right_parenthese();
 55                                 self.push_token_plus();
 56                             } else {
 57                                 if c == '"' {
 58                                     self.panic(&format!("expect {:?}, but arrive \"", &unsafe{String::from_utf8_unchecked(end.to_vec())}));
 59                                 }
 60                                 self.select(c);
 61                             }
 62                         }
 63                     }
 64                 },
 65                 None => {
 66                     match (self.cb)() {
 67                         CallbackReturnStatus::Continue(content) => {
 68                             self.content_assign(content);
 69                             continue;
 70                         },
 71                         CallbackReturnStatus::End => {
 72                             self.panic("expect \", but arrive IO EOF");
 73                         }
 74                     }
 75                 }
 76             }
 77         }
 78     }
```
- 代码分析
	- 首先需要记录一个状态, 用于保存当前是找 双引号, 还是找终结串
		* 多说一句, 在 rust 中, 方法内部是可以定义枚举的(这一点非常喜欢, 这样就不需要担心与外部的冲突了), 而且, 方法的定义在整个编译环境中只有一份, 所以方法内的枚举也是一份, 不需要担心效率问题, 反而这相比定义在外部, 会更加的快速 (毕竟不用走太远去查找定义的地方, 就近就可以)
	- 查找双引号的状态下
		- 遇到结尾的双引号, 说明 strfmt 语句结束了 -> 直接退出循环
		- 如果不是双引号, 调用工具方法 input_str_match, 判断是否和 **起始串** 匹配
			* 匹配: 则更新状态为 查找 **终结串**
			* 不匹配: 将当前的字符写入 buffer 中(说明这是普通的字符)
	- 查找终结串的状态下
		- 调用工具方法 input_str_match, 判断是否和 **终结串** 匹配
			* 匹配: 更新状态为 查找 双引号 (这样就回到了初始状态, 然后继续匹配下一个表达式)
			* 不匹配: 说明是表达式, 那么直接使用词法分析的函数解析之后的字符

