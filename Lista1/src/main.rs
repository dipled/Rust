fn celsius_to_farenheit(ctemp : f32) -> f32 {
    let ftemp : f32 = ctemp * 9.0/5.0 + 32.0;
    ftemp

}


fn main() {
    let ctemp : f32 = 30.0;
    println!("{}", celsius_to_farenheit(ctemp));


}
