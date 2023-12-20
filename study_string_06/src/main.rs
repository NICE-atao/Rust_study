fn main() {

    /*字符串有两种类型：1、 string 是创建在堆中，可以改变大小
                    2、&str 是字符串切片，不能更改大小，它是对存储在其他地方（如字符串字面量或者 String 类型）的字符串的引用。*/

    /*创建一个新的字符串*/
    //1、使用String::new()
    // push或者push_str向字符串中堆字符，push堆的是单个字符，push_str堆的是字符串
    let mut my_string:String=String::new();
    my_string.push_str("Hellow");
    my_string.push(',');
    my_string.push_str("world");
    my_string.push('!');
    println!("{}",my_string);

    //2、使用字符串字面量创建字符串
    //可以使用 to_string() 或者 String::from() 方法，将字符串字面量转换为 String 类型的值
    let my_string2="hellow,world1".to_string();  //   字符串字面量.to_string()   将字符串字面量转化为字符串
    let my_string3=String::from("hellow,world2");
    println!("{}",my_string2);
    println!("{}",my_string3);

    /*字符串操作*/
    //1、字符串拼接和连接
   //使用 + 操作符或者 format! 宏来拼接和连接字符串
    let  my_string4:String="hellow".to_string();
    let  my_string5:String="world".to_string();
    // let my_string6=my_string4+","+&my_string5;   //第二个字符串需要引用的方式，表示字符串的切片
    let my_string6=my_string4.clone()+","+&my_string5;   //克隆my_string4,因为后面会用到my_string4变量
    println!("{}",my_string6);
    let my_string7=format!("{},{}",my_string4,my_string5);   //直接这样写的话会报错，因为my_string4在第30行被移动到my_string6，原来的my_string4就不能用了
    println!("{}",my_string7);

}
