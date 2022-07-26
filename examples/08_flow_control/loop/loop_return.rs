/**
 * 从循环中返回
 * 
 * loop 有个用途是尝试一个操作直到成功为止，若操作返回一个值，则可能需要将其传递给代码的其余部分
 * 将该值放在break 之后，它就会被loop 表达式返回
 */

 fn main() {
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter  == 10 {
            break counter * 2;
        }
    };

    println!("result:{}",result);
    assert_eq!(result,20);

 }