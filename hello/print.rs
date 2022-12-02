fn main() {
    // 直接替换变量
    println!("{} days", 31);

    //变量替换字符串
    println!("{0}, this is {1}.\n{1}, this is {0}.", "Alice", "Bob");

    //使用命名参数显示字符串
    println!(
        "{first}, {second} and {last}",
        first = "this is first",
        second = "this is second",
        last = "this is last"
    );

    //使用:
    println!("{}, {:b}", 1, 2);
    //下面语句的数字“2”占用六位宽度
    println!("{} of {:6} people know binary, the other half don't", 1, 2);

    //指定宽度
    //下面语句的数字“9”占用五位宽度，前面4个空格表示方法和前面4个“0”
    println!("{first:>width$}", first = 9, width = 5);
    println!("{last:>0width$}", last = 9, width = 5);

    //检查参数
    println!("{0}, {2} and {1}", "first", "second", "last");

    //练习题目
    /*
        再用一个 println! 宏，通过控制显示的小数位数来打印：Pi is roughly 3.142（Pi 约等于 3.142）。
        为了达到练习目的，使用 let pi = 3.141592 作为 Pi 的近似值
        （提示：设置小数位的显示格式可以参考文档 std::fmt）。
    */

    let pi = 3.141592;
    println!("Pi is roughly {0:>.3}", pi);
    println!("Pi is roughly {:.1$}", pi, 3);
    println!("Pi is roughly {:.*}", 3, pi);
}
