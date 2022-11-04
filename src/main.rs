use dirs::download_dir;
use qrcode_generator::QrCodeEcc;
fn main() {
    //1st arg is always link
    let args = std::env::args().collect::<Vec<String>>();
    let link = &String::from(&args[1]);

    if let Some(pth) = download_dir() {
        if let Ok(out_dir) = pth.into_os_string().into_string() {
            let _res = match qrcode_generator::to_svg_to_file(
                link,
                QrCodeEcc::Low,
                1024,
                None::<&str>,
                format!("{}{}",out_dir,"\\qr_out.svg"),
            )
            {
                Err(e) => panic!("Got error while generating QR err {e}"),
                Ok(()) => println!("Successfully generated QR Code.")
            };
        }
    } else {
        println!("Problem reading /Downloads dir path.")
    }
}