fn main() {
    let a:i32=2;
    let b:u8=3;
    function_text();
    function_text1(a,b);
    let c:i32=function_text2(a,b);
    println!("Hello, world!");
}


fn function_text(){
    println!("这是一个函数");
}
fn function_text1(a:i32,b:u8){
    println!("a={},b={}",a,b);    //无返回值
}
fn function_text2(a:i32,b:u8) ->i32{
    println!("下面是有返回值的函数");    //有返回值
    let result=a+b as i32;           //在Rust中，不同类型的值不能直接相加，需要转换 b as i32 将b强制转换为i32
    println!("a+b={}",result);
    // return result;                  //或者直接result
    result
}


