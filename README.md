## 使用说明

### window

1. 下载 [releases](https://github.com/jiangbo212/single-file-compaion-rust/releases) 下最新的window版本。
2. 解压文件中编辑.env文件，填入自己的minio服务信息
3. SingleFile插件选项中，将保存选项选择“使用 SingleFile Companion 保存”
4. 双击uninstall.bat, 再双击install.bat

以上步骤执行完毕，点击singlefile即可自动上传至minio服务

### mac

1. 下载 [releases](https://github.com/jiangbo212/single-file-compaion-rust/releases) 下最新的mac版本。
2. 解压文件中编辑.env文件，填入自己的minio服务信息
3. SingleFile插件选项中，将保存选项选择“使用 SingleFile Companion 保存”
4. 给uninstall.sh, install.sh赋予执行权限，然后首先执行uninstall.sh, 再执行install.sh

以上步骤执行完毕，点击singlefile即可自动上传至minio服务

## 其他
+ 本应用因[single-file-companion](https://github.com/gildas-lormeau/single-file-companion)启发
+ 本应用参考了[chrome-native-messaging](https://github.com/neon64/chrome-native-messaging)的部分实现

## chrome native message相关
具体协议地址: [https://developer.chrome.com/docs/extensions/develop/concepts/native-messaging](https://developer.chrome.com/docs/extensions/develop/concepts/native-messaging)

### 重点 

Make sure that all output in stdout adheres to the native messaging protocol. If you want to print some data for debugging purposes, write to stderr.

说人话就是不用使用println!, 如果需要打印日志，请使用eprintln!或其他日志包输出日志(以上针对的是rust，其他变成语言也类似)

其他日志包一般使用的是stderr输出日志，这点一定要确认。

### 具体概念

简单来讲，chrome-native-message就是就是chrome将一堆消息以stdin的方式发送给你配置位置的程序(不需要启动程序)，就SingleFile而言你不需要关心输出，因此不需要有stdout,因此一定不要使用println！。

stdin传递过来的数据流，前4byte是一个32的数字，代表数据流长度，因此只需要解析前4byte获取数据流的长度，将流解析成具体数据即可。

### stdin stdout stderr

stdin 用户的输入流

stdout 用户输出流

stderr 异常输出流，不会影响返回用户的数据