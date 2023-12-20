fn main() {
    //if语句
    let a:i8=1;
    if a==1{                 //==表判断，返回布尔型
        println!("a的值为1")
    }

    //if，else 语句
    let a:i8=1;
    if a==1{                 //==表判断，返回布尔型
        println!("a的值为1")
    }else {
        println!("a的值不为1")
    }

    //if，else if ,else 语句
    let a:i8=3;
    if a==1{                 //==表判断，返回布尔型
        println!("a的值为1")
    }else if a==2{
        println!("a的值为2")
    }else {
        println!("a的值不为1也不为2")
    }

    //在let中使用if
    let bool_1=true;
    let b:i32=if bool_1 {
        5
    }else {
        6
    };
    println!("b的值为：{}",b);

    //loop 循环
    let mut counter:i32=0;
    loop {
        if counter==10 {
            break;
        }else {
            println!("counter={}",counter);
        }
        counter=counter+1;
    }
    //使用loop 循环为变量赋值
    let result = loop {
        counter = counter+1;
        if counter == 20 {
            break counter;
        }
    };
    println!("{}",result);

    //while循环
    let mut i:i32=0;
    while i<10 {
        i=i+1;
    }
    println!("{}",i);

    //for循环,需要用到迭代器，后面再写

}




