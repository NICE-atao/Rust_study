fn main() {
    //布尔 bool
    println!("下面是布尔型变量的例子:");
    let T=true;
    let F=false;
    println!("T是{}，F是{}",T,F);

    //char 字符型，RUST比较特别，是32位的
    println!("——————————————————————");
    println!("下面是字符型变量的例子:");
    let a:char ='a';
    println!("a={}",a);
    let b:char='张';    //如果是C语言的话，这段话会报错，因为C语言中的字符型是16位
    println!("b={}",b);

    //数字类型 i8,i16,i32,i64  u8,u16,u32,u64  f32,f64
    println!("——————————————————————");
    println!("下面是数字型变量的例子:");
    let c:i8=-111;      //i8的范围是
    let d:f32=0.9999;
    println!("i8类型的c的值是:{}",c);
    println!("f32类型的d的值是:{}",d);

    //自适应类型 根据电脑的不同，长度不同，64位的电脑就是64位的，32位的电脑就是32位的
    println!("——————————————————————");
    println!("下面是自适应型变量的例子:");
    println!("有符号isize，无符号usize");
    println!("isize能表示的最大值是：{}",isize::max_value());
    println!("usize能表示的最大值是：{}",usize::max_value());

    //数组，数组的格式是   数组名:[类型;长度]
    println!("——————————————————————");
    println!("下面是数组的例子:");
    let arr:[u32;6]=[1,2,3,4,5,6];
    println!("arr[0]={}",arr[0]);

    //元祖，复合类型，格式是 元祖名:(数据类型1,数据类型2,数据类型3)
    println!("——————————————————————");
    println!("下面是元祖的例子:");
    let tup:(i32,u8,char)=(6,7,'a');       //char,字符型，用''表示，&str
    let tup2:(i32,u8,&str)=(6,7,"atao");   //&str，字符串，用""表示
    println!("元祖中i32的值为{},u8类型的值为{}，char类型的值为{}",tup.0,tup.1,tup.2);
    println!("下面是元祖拆解的例子:");
    let (x,y,z)=tup;
    println!("拆解并重新赋值后元祖中i32的值为{},u8类型的值为{}，char类型的值为{}",x,y,z);

}
