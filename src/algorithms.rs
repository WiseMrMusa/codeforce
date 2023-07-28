fn main(){
}


// O log(min(a,b))
fn gcd(a:u32, b:u32) -> u32 {
    if b == 0 {
        a
    }
    gcd(b,a%b)
}

// 
fn lcm(a:u32, b:u32) -> u32 {
    a*b / gcd(a,b)
}