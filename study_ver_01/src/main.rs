const ATAO:i32=520;   //定义常量，语法格式是const+变量名:数据类型=值

fn main() {
    //变量的可变性
    let mut a=1;     //mut,使变量可变
    let b =2;
    a=3;
    println!("a={}",a);
    println!("b={}",b);
    println!("后面是变量的隐藏性，我已经定义了一个变量b，在后面的代码中还可以定义变量b，后面的变量b会把前面的b隐藏");
    //变量的隐藏性
    let b=4;
    println!("b={}",b);
    //常量
    println!("常量ATAO的值为：{}",ATAO);
}
