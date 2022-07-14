//题目2   u32 类型求和
pub fn sum_u32(vec: &[u32]) -> Option<u32> {
    let mut sum: u32 = 0;

    for i in vec.iter() {
        //使用checked_add方法判断是否溢出,如果溢出则返回None
        if sum.checked_add(*i) == None {
            return None;
        } else {
            //如果没有溢出则继续求和
            sum += *i;
        }
    }
    //返回求和结果，包裹在Option枚举Some中
    Some(sum)
}
