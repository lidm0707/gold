use reqwest::blocking::get;
use scraper::{Html, Selector};
use std::error::Error;

fn fetch_gold_prices() -> Result<(), Box<dyn Error>> {
    // URL ของเว็บไซต์
    let url = "https://www.goldtraders.or.th/";

    // ส่งคำขอ HTTP GET
    let response = get(url)?.text()?;

    // แปลง HTML เป็น Scraper
    let document = Html::parse_document(&response);

    // สร้าง selector สำหรับดึงข้อมูลจาก span ที่มี id ที่เกี่ยวข้อง
    let sell_selector = Selector::parse("#DetailPlace_uc_goldprices1_lblBLSell").unwrap();
    let buy_selector = Selector::parse("#DetailPlace_uc_goldprices1_lblBLBuy").unwrap();
    let omsell_selector = Selector::parse("#DetailPlace_uc_goldprices1_lblOMSell").unwrap();
    let ombuy_selector = Selector::parse("#DetailPlace_uc_goldprices1_lblOMBuy").unwrap();

    // ดึงราคาทองคำจาก span ต่างๆ
    let sell_price = document.select(&sell_selector).next().map(|e| e.text().collect::<String>());
    let buy_price = document.select(&buy_selector).next().map(|e| e.text().collect::<String>());
    let omsell_price = document.select(&omsell_selector).next().map(|e| e.text().collect::<String>());
    let ombuy_price = document.select(&ombuy_selector).next().map(|e| e.text().collect::<String>());

    // แสดงผลลัพธ์
    println!("ราคาทองคำแท่ง 96.5% ขายออก: {}", sell_price.unwrap_or("ไม่พบข้อมูล".to_string()));
    println!("ราคาทองคำแท่ง 96.5% รับซื้อ: {}", buy_price.unwrap_or("ไม่พบข้อมูล".to_string()));
    println!("ราคาทองรูปพรรณ 96.5% ขายออก: {}", omsell_price.unwrap_or("ไม่พบข้อมูล".to_string()));
    println!("ราคาทองรูปพรรณ 96.5% รับซื้อ: {}", ombuy_price.unwrap_or("ไม่พบข้อมูล".to_string()));

    Ok(())
}

fn main() {
    if let Err(e) = fetch_gold_prices() {
        println!("เกิดข้อผิดพลาด: {}", e);
    }
}
