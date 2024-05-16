use lgr_9_comments::color;


fn main() {
    println!("Hello, world!");
    let color = color::RGB{
        r:123,
        g:33,
        b:1
    };
    
    lgr_9_comments::shapes::Rectangle{
        color: color,
        height: 33,
        width: 33
    };
    color::draw_line(32, 32, &color)
}
