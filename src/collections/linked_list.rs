//! ```
//! // 1. mut, &, &mut的区别
//! let a = "xxx".to_string();　　
//! // 含义：a绑定到字符串资源A上，拥有资源A的所有权

//! let mut a = "xxx".to_string();　
//! // 含义：a绑定到字符串资源A上，拥有资源A的所有权，同时a还可绑定到新的资源上面去（更新绑定的能力，但新旧资源类型要同）；

//! // value
//! let b = a;
//! // 含义：a绑定的资源A转移给b，b拥有这个资源A

//! let b = &a;　　
//! // 含义：a绑定的资源A借给b使用，b只有资源A的读权限

//! let b = &mut a;　　
//! // 含义：a绑定的资源A借给b使用，b有资源A的读写权限

//! let mut b = &mut a;　　
//! // 含义：a绑定的资源A借给b使用，b有资源A的读写权限。同时，b可绑定到新的资源上面去（更新绑定的能力）
//! 
//! // 2. unwrap()方法：
//! // 在确认Option不为None的情况下，可以用unwrap方法拆解出其中的值，并获取值的所有权，
//! // 如果为None，就会引发panic。这里要强调的是，unwrap会消费Option本身的值，后面就不能再用了。
//! let a = Some(Box::new(5));
//! let d = a.unwrap();
//! // println!("{:?}", a); // cannot use 'a' again.
//! println!("{:?}", d);
//! // 但这里有一个所有权的限制，因为涉及到其内部值的所有权的转移，所以只有Option原始变量本身可以调用unwrap方法，
//! // 其引用（包括可变引用）均不可调用。这和unwrap的实现方法有关系，因为其消费了原始变量。下方代码不可编译：
//! let mut a = Some(Box::new(5));
//! let b = &mut a;
//! let c = b.unwrap(); // Error! cannot move out of `*b` which is behind a mutable reference
//! // 3. take()方法：
//! // take方法可以从Option中取出值，为原来的Option变量留下None值，但原来的变量还有效（但实际上没什么用处了，因为只剩下None了）。
//! // take方法的原始值或者引用必须为mut类型。强调一下，借用情况下可以调用take方法。或者说指向Option的可变引用（指针）可以调用take方法。
//! fn main() {
//!     let mut b = &mut a;
//!     let c = &mut b;
//!     let d = c.take();
//!     println!("{:?}", c);//None
//!     println!("{:?}", d);//Some(5)
//!     println!("{:?}", a);//None
//! }
//! // 4.as_ref()方法：
//! // 但上面这两个方法，都改变了Option变量本身。如果不改变或受到限制无法改变Option本身时，
//! // 只想借用或可变借用其中unwrap的值应该怎么办呢？这也是在实现链表时遇到的让人掉坑的问题。幸好Option提供了 as_ref 和 as_mut方法。
//! fn as_ref(&self) -> Option<&T>
//! // 接受一个不可变引用，不获取所有权，返回一个Option枚举，其中的T为不可变引用类型，不获取所有权。
//! // 该方法将对Option的引用变为对Option所包含对象的不可变引用，并且返回一个新的Option。对这个新的Option进行unwrap操作，
//! // 可以获得原Option所包含的对象的不可变引用。原始Option变量及其引用，均可调用as_ref方法，有点克隆的味道。
//! let a = Some(Box::new(5));
//! let b = a.as_ref();
//! let c = b.unwrap();
//! println!("{:?}", a);//Some(5)
//! println!("{:?}", b);//Some(5)
//! println!("{:?}", c);//5
//! // 5. as_mut()方法
//! // 与as_ref类似，as_mut只是返回了Option包含的数据的可变引用，
//! fn main() {
//!     let mut a = Some(Point { x: 5, y: 5 });
//!     // let b = a.as_mut();
//!     let b = &mut a;
//!     let c = b.as_mut(); // c is a new Option;
//!     let d = c.unwrap();
//!     d.x += 10;
//!     let e = &mut d.y;
//!     *e += 20;
//!     println!("{:?}", d.x);//15
//!     // println!("{:?}", c); // c is not available because of method call of "unwrap".
//!     println!("{:?}", b);//Some(Point { x: 15, y: 25 })
//!     println!("{:?}", a);//Some(Point { x: 15, y: 25 })
//! }
//! // 通过以上代码可以看出，在未改变Option变量a的情况下，通过as_mut方法，改变了其包含的数据的值。
//! // 这个能力对于编写链表时，尤其是节点的插入、删除时，可以灵活的操作指向下一个节点的指针。
//! // 说明一点，调用as_mut方法时，Option变量本身或其引用，必须为可变的，即mut类型
//! fn main() {
//!     let mut a = Some(5);
//!     let b = a.as_mut().unwrap();
//!     *b += 10;
//!     println!("b = {:?}", b);//b = 15
//!     println!("a = {:?}", a);//a=Some(15)
//! }
//! 
//! // 6. if let和while let
//! if let Some(a_T) = a_option { 
//!     //a_T生命周期仅在花括号内生效。
//!     //当a_option不为None时满足条件。
//! }
//! 
//! while let Some(a_T) = a_option { 
//!     //当a_option 为None时退出循环。
//! }
//! // ？操作符的作用就是简化Result枚举的
//!     let f = File::open("username.txt"); 
//!     let mut f = match f {
//!     Ok(file) => file,
//!     Err(e) => return Err(e),
//! };
//! //有了？运算符，就可以改成下面一行话
//! let mut f = File::open("username.txt")?;
//! 
//! ```
//! 
//! 模块系统是显式的(译者注:需要明确的声明)——不存在和文件系统的1:1映射
//! 我们在一个文件的父级目录把它声明为模块，而不是在文件自身
//! mod关键字用于声明子模块
//! 我们需要显式地将函数、结构体等声明为公开的，这样它们才可以被其他模块访问
//! pub关键字把事物声明为公开的
//! use关键字用于简化(缩短)模块路径
//! 我们不需要显式声明第三方的模块
#[derive(Debug)]
struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

struct LinkedList<T> {
    head: Option<Box<Node<T>>>,
    size: usize,
}

impl<T> LinkedList<T> {
    pub fn new() -> self {

    }

    pub fn is_empty() -> bool {

    }

    pub fn len() -> usize {

    }

    pub fn push(&mut self, data: T) -> &mut self {

    }

    pub fn pop(&mut self) -> Option<T> {

    }

    pub fn peek(&self) -> Option<&T> {

    }

    pub fn peek_mut(&mut self) -> Option<&mut T> {

    }

    pub fn reverse(&self) -> LinkedList<T> {
        
    }
}