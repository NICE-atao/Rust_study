//1、rust通过所有权机制管理内存
//2、堆和栈
    //编译的时候如果数据类型占用内存的大小是确定的，那么就是分配在栈上的
    //编译的时候如果数据类型占用内存的大小不确定，那就是分配在堆上
//3、作用域
//4、String内存回收
//5、clone克隆
fn main() {
    /*作用域，定义的变量只在他对应的{}区间内有效*/
    // {
    //     let x=2;
    //     let y=3;
    //     println!("x={}",x);      //会报错，因为y的作用域在9-13行
    // }
    let x=2;
    let y=3;
    println!("x={}",x);                   //x和y定义在栈上
    println!("y={}",y);
    println!("Hello, world!");

    /*String内存回收，String是定义在堆上的，编译器在编译之前不知道String的大小*/
    {
        let mut s1=String::from("Hellow");     //String类型离开作用域后会调用Drop方法，释放内存
        s1.push_str(" World!");                    //push之后，String变量的指针ptr不会变，变的只是String的len
        println!("s1={}",s1);

        let s2 = s1;                            //执行这段代码之后，s1移动到s2，s1就无效了，该作用域内只有s2
       // println!("{}",s1);
    }

    /*clone克隆*/
    {
        let mut s1=String::from("Hellow");     //String类型离开作用域后会调用Drop方法，释放内存
        s1.push_str(" World!");                    //push之后，String变量的指针ptr不会变，变的只是String的len
        println!("s1={}",s1);
        let s2 = s1.clone();                   //执行这段代码后，s1克隆到s2，s2会重新分配一段内存，而原来的s1仍然有效，相当于s2和s1指向不同的内存
        println!("{}",s1);
    }

    //copy 特征
    let a=1;
    let b=a;
    println!("{},{}",a,b);                          //代码有效，因为整型变量不像String类型指向一块内存，在执行let语句时，会把原来的a copy到 b,因为没有指针指向内存，所以在离开作用域时就不会出现多次释放同一块内存的情况，所以Rust不会将a无效
    //常用的具有copy特征的类型：
    //所有的整型
    //浮点型
    //布尔值
    //元组
    //字符型 char（注意不是字符串）

    /*函数和作用域*/
    fn  function1 (string1:String) ->String{
        println!("{}",string1);
        string1
    }
    fn function2(int1:i32){
        println!("{}",int1);
    }
    let s1=String::from("hellow");
    let s2=function1(s1);
    println!("{}",s2);             //直接打印会报错，因为s1传入到函数function1之后，进入函数的作用域，出了函数作用域后，会把内存释放。（已修改function1，将s1传出来）
    let a=12;
    function2(a);
    println!("{}",a);
}
