fn main() {
    let Toshiba: f64 = 450_000.0;
    let Mac: f64 = 1_500_000.0;
    let HP: f64 = 750_000.0;
    let Dell: f64 = 2_850_000.0;
    let Acer: f64 = 250_000.0;
    let Total_Number_of_Goods: f64 = 5.0;

    // Total amount of sales
    let Total_Amount_of_Sales = Toshiba + Mac + HP + Dell + Acer;
    println!("Amount from sales is ₦{}", Total_Amount_of_Sales);

    // Average
    let Avg = Total_Amount_of_Sales / Total_Number_of_Goods;
    println!("The average is ₦{}", Avg);
}


