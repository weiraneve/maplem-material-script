# 冒险岛·枫之传说 材料本自动脚本

## 简介

自己写的脚本，使用`playcover`结合rust自动控制键盘输入，控制刷本流程的方案。

**本脚本使用场景是，苹果区iOS，在Macos的电脑上才能使用，并且Macos电脑只能是M系列芯片的。**

## 刷本流程

1. 在游戏界面，首先把每日挂机时间领取2小时。
2. 切入公会，领取签到奖励。
3. 进入邮件，领取个人奖励，如刚刚领取的副本入门卷
4. 进入材料副本选择界面，使用全部的机会
5. 材料副本进入后，等待整个副本刷完(等待一点的时间保证游戏进入结算页面，角色阵亡之后也会进入那个结算页面)
6. 切换在一个角色
7. 重复以上过程

## 脚本使用

1. 首先下载[playcover](https://github.com/PlayCover/PlayCover/releases)这个软件，在页面中点击dmg文件下载。
2. 下载好`playcover`软件之后，去到[下载ipa的网站](https://decrypt.day)，搜索`冒险岛·枫之传说`，然后下载对应的ipa文件，然后导入`playcover`文件中。
3. 然后在`playcover`软件界面，找到下载并导入好的`冒险岛·枫之传说`游戏，右键选择![](https://s11.ax1x.com/2024/02/25/pFaQlZj.png)

导入[我自己设置好的键盘映射文件](https://github.com/weiraneve/maplem-material-script/blob/main/冒险岛：枫之传说.playmap)

![](https://s11.ax1x.com/2024/02/25/pFalmkR.png)

4. [下载对应的脚本](https://github.com/weiraneve/maplem-material-script/releases) 在这个页面下载最新的release软件，下载好执行文件之后，在自己的macos电脑上，打开终端，把脚本软件拖入到终端之中，就可以执行了。
5. 使用脚本的时候，需要将游戏在电脑中打开，然后登陆到自己的第一个小号，**一定是第二个角色**。然后进入页面之后把可能遮挡界面的游戏内广告叉掉，然后按照刚刚说的方式执行脚本，然后在终端中执行之后，赶紧在电脑中切换到游戏中，聚焦在游戏上，脚本自带4秒钟的延迟开始执行。

### 注意事项

1. 电脑屏幕尺寸是否会影响键盘映射导入的问题，这个还没验证。
2. 我是按照自己账号的9个角色设计的脚本，也就是说只能帮助刷9个角色的，之后再看看有无时间更新脚本，让脚本能具有更多适配的功能
3. 稳定性有待加强QAQ
4. 在运行脚本的时候，千万不能让电脑的聚焦离开游戏，否则流程中断。(在等待刷材料本的时候可以)，所以建议在电脑闲置的时候使用脚本。
5. 从github下载的脚本，要用`chmod +x /path/maplem-material-script`修改文件权限才能在macos电脑上执行。
6. 脚本执行开始时，一定是从第二个角色开始，当然你懂本脚本代码的话，可以自己去修改流程。
7. 材料本固定选择的是60级的图。在对应角色设置的时间内没打完，会直接退出。
8. 本脚本是针对自己的多个角色进行的脚本定制，如果要使用非定制的，可以使用`1.0.4`的release。
