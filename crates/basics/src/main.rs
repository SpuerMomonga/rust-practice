// 函数（Functions）
// 显示声明入参类型
#[allow(dead_code)]
fn add(x: i32, y: i32) -> i32 {
    // 隐式返回（不要分号）
    x + y
}

#[allow(unused_variables)]
#[allow(unused_assignments)]
#[allow(dead_code)]
// 主函数(Main function)
fn main() {
    // 定义不可变变量
    let x: i32 = 1;

    // 整型/浮点型 后缀
    let y: i32 = 13i32;
    let f: f64 = 1.3f64;

    // 类型推导
    // 整型默认推导类型 `i32`，浮点型默认推导 `f64`
    let implicit_x = 1;
    let implicit_y = 1.3;

    // 算术运算
    let sum = x + y + 13;

    // 可变变量
    let mut mutable = 1;
    mutable = 4;
    mutable += 2;

    // 字符串字面量
    let x: &str = "hello world";

    // 输出 - 宏调用
    println!("{} {}", f, x);

    // 一个 `String` - 在堆上分配空间的字符串
    // 堆上存储为 Vec<u8>
    let s = "hello world".to_string();

    // 字符串切片（slice） - 另一个字符串的不可变视图
    // 基本上就是指向一个字符串的不可变指针，它不包含字符串里任何内容，只是一个指向某个东西的指针
    let s_slice: &str = &s;

    // 长度固定的数组（Array）
    let four_ints = [1, 2, 3, 4];

    // 向量（Vector）- 可变数组
    let mut vector = vec![1, 2, 3, 4];
    vector.push(5);

    // 数组切片 - 某个数组(vector/array)的不可变视图
    // 和字符串切片基本一样，只不过是针对数组的
    let slice = &vector;

    // 使用 `{:?}` 按Debug样式输出
    println!("{:?} {:?}", vector, slice);

    // 元组（Tuples）- 元组是固定大小的一组值，可以是不同类型
    let x: (i32, &str, f64) = (1, "hello", 3.4);

    // 解构元组
    let (a, b, c) = x;
    println!("{} {} {}", a, b, c);

    // 元组索引
    println!("{}", x.1);

    // 结构体（Struct）
    struct Point {
        x: i32,
        y: i32,
    }

    // 结构体初始化
    let origin = Point { x: 0, y: 0 };

    // 匿名成员结构体，又叫“元组结构体”（‘tuple struct’）
    struct Point2(i32, i32);

    // 结构体初始化
    let origin2 = Point2(0, 0);

    // 基础的 C 风格枚举类型（enum）
    enum Direction {
        Left,
        Right,
        Up,
        Down,
    }

    let up = Direction::Up;

    // 有成员的枚举类型
    enum OptionalI32 {
        AnI32(i32),
        Nothing,
    }

    let two: OptionalI32 = OptionalI32::AnI32(2);
    let nothing = OptionalI32::Nothing;

    // 泛型 (Generics)
    struct Foo<T> {
        bar: T,
    }

    // 这个在标准库里面有实现，叫 `Option`
    enum Optional<T> {
        SomeVal(T),
        NoVal,
    }

    // 方法 (Methods)
    impl<T> Foo<T> {
        // 方法需要一个显式的 `self` 参数
        fn get_bar(self) -> T {
            self.bar
        }
    }

    let a_foo = Foo { bar: 1 };
    println!("{}", a_foo.get_bar()); // 1

    // 接口（Traits） （其他语言里叫 interfaces 或 typeclasses
    trait Frobnicate<T> {
        fn frobnicate(self) -> Option<T>;
    }

    impl<T> Frobnicate<T> for Foo<T> {
        fn frobnicate(self) -> Option<T> {
            Some(self.bar)
        }
    }

    let another_foo = Foo { bar: 1 };
    println!("{:?}", another_foo.frobnicate()); // Some(1)

    // 模式匹配 (Pattern matching)
    let foo1 = OptionalI32::AnI32(4);
    match foo1 {
        OptionalI32::AnI32(n) => println!("it’s an i32: {}", n),
        OptionalI32::Nothing => println!("it’s nothing!"),
    }

    // 高级模式匹配
    struct FooBar {
        x: i32,
        y: OptionalI32,
    }
    let bar = FooBar {
        x: 15,
        y: OptionalI32::AnI32(32),
    };

    match bar {
        FooBar {
            x: 0,
            y: OptionalI32::AnI32(0),
        } => println!("The numbers are zero!"),
        FooBar {
            x: n,
            y: OptionalI32::AnI32(m),
        } if n == m => println!("The numbers are the same"),
        FooBar {
            x: n,
            y: OptionalI32::AnI32(m),
        } => println!("Different numbers: {} {}", n, m),
        FooBar {
            x: _,
            y: OptionalI32::Nothing,
        } => println!("The second number is Nothing!"),
    }

    // 流程控制 (Control flow)
    // `for` 循环
    let array = [1, 2, 3];
    for i in array {
        println!("{}", i);
    }

    // 区间 (Ranges)
    for i in 0u32..10 {
        print!("{} ", i);
    }
    println!(" ");
    // 输出 `0 1 2 3 4 5 6 7 8 9 `

    // `if`
    // a == 1
    if true {
        println!("Maths is working!");
    } else {
        println!("Oh no...");
    }

    // `if` 可以当表达式
    let value = if true { "good" } else { "bad" };

    // `while` 循环
    // a > 1
    while mutable > 5 {
        mutable += 1;
        println!("The universe is operating normally.");
    }

    // 无限循环
    loop {
        println!("Hello!");
        if mutable > 10 {
            mutable += 1;
            // break 语句用于跳出循环
            break;
        }
    }

    // 内存安全和指针 (Memory safety & pointers)
    // 独占指针 (Owned pointer) - 同一时刻只能有一个对象能“拥有”这个指针
    // 意味着 `Box` 离开他的作用域后，会被安全地释放
    let mut mine: Box<i32> = Box::new(3);
    *mine = 5; // 解引用
               // `now_its_mine` 获取了 `mine` 的所有权。换句话说，`mine` 移动 (move) 了
    let mut now_its_mine = mine;
    *now_its_mine += 2;

    println!("{}", now_its_mine); // 7
                                  // println!("{}", mine); // 编译报错，因为现在 `now_its_mine` 独占那个指针

    // 引用 (Reference) – 引用其他数据的不可变指针
    // 当引用指向某个值，我们称为“借用”这个值，因为是被不可变的借用，所以不能被修改，也不能移动
    // 借用一直持续到生命周期结束，即离开作用域
    let mut var = 4;
    var = 3;
    let ref_var: &i32 = &var;

    println!("{}", var); //不像 `mine`, `var` 还可以继续使用
    println!("{}", *ref_var);
    // var = 5; // 编译报错，因为 `var` 被借用了
    // *ref_var = 6; // 编译报错，因为 `ref_var` 是不可变引用

    // 可变引用 (Mutable reference)
    // 当一个变量被可变地借用时，也不可使用
    let mut var2 = 4;
    let ref_var2: &mut i32 = &mut var2;
    *ref_var2 += 2;

    println!("{}", *ref_var2); // 6
                               // var2 = 2; // 编译报错，因为 `var2` 被借用了
}
