mod env;
fn main(){

    let city: &str = "Berlin,DE";
    let api_key = env::API_KEY;
    let url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&appid={}", city, api_key);
    println!("OPEN {}", url);

    println!("\n\nHerzlich willkommen! \nIn diesem Programm können sie sich informieren wie das Wetter in ihrem Ort ist. \n");
    println!("Bitte geben Sie den Namen des Ortes ein: ");
    println!("\n\n\n");


}


