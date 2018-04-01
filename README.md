BiliBili-live-notifier
====
B站直播弹幕姬 rust版本

## 说明
基于electron版本的
[直播姬](https://github.com/pandaGao/bilibili-live-helper)
在我的WM和混成器(awesome wm + compton)下有各种显示bug，

基于[lyyyuna](https://github.com/lyyyuna)的
[B站直播弹幕协议详解](http://www.lyyyuna.com/2016/03/14/bilibili-danmu01)
开发rust版本的弹幕姬。

因为notify-rust通过dbus发送消息通知，仅适用linux。

## 进度
- [ ] 数据包解析
- [ ] 消息通知
- [ ] 稳定性优化
- [ ] 通知美化
- [ ] 改为基于tokio的异步版本
- [ ] 后台服务优化
