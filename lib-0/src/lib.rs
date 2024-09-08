#![feature(lazy_cell)]
#![feature(lazy_type_alias)]



use std::sync::*;
use crossbeam::sync::*;



#[no_mangle]
static GLOBAL_LICENSE: LazyLock<Arc<ShardedLock<String>>> = LazyLock::new(|| {
    Arc::new(
        ShardedLock::new(
            "license0".to_string()
        )
    )
});



#[no_mangle]
pub fn client_connect_license_0() {
    // let d0 = Arc::from(&*GLOBAL_LICENSE);
    println!("License process run.");
    let s0 = std::thread::spawn(move || {
        println!("License 0");
        // std::thread::sleep(std::time::Duration::from_secs_f32(7.5));
        std::thread::sleep(std::time::Duration::from_secs_f32(1.5));

        use rand::{thread_rng, Rng};
        let mut rng = thread_rng();
        let a = rng.gen_range(-3..3) as isize;

        {
            println!(
                "\t1 - {:?}",
                &*GLOBAL_LICENSE.read().unwrap().clone(),
            );
        }

        // {
        //     let temp_d0 = &*GLOBAL_LICENSE.read().unwrap().clone();
        //     let mut temp_d1 = &mut *GLOBAL_LICENSE.write().unwrap();
        //     *temp_d1 = format!(
        //         "{}.{}",
        //         temp_d0,
        //         a,
        //     ).to_string();
        // }

        // {
        //     println!(
        //         "\t2 - {:?}",
        //         &*GLOBAL_LICENSE.read().unwrap().clone(),
        //     );
        // }
        
        // std::thread::sleep(std::time::Duration::from_secs_f32(7.5));
        std::thread::sleep(std::time::Duration::from_secs_f32(1.5));
        println!("License 1");



        use chrono::prelude::*;
        let utc: DateTime<Utc> = Utc::now();
        let stamp0 = utc.format("%d . %m . %Y   -   %H : %M : %S   ( UTC )").to_string();



        use argon2::Argon2;

        let a0 = "FLAT license key".to_string();
        let a1 = stamp0.clone();


        let binding = a0.to_string();
        let password = binding.as_bytes();
        let binding = a1.to_string();
        let salt = binding.as_bytes();
    
        let mut token_auth0 = [0u8; 96];
        Argon2::default().hash_password_into(password, salt, &mut token_auth0).unwrap();

        let mut token_str0 = String::new();
        for elem0 in token_auth0.to_vec() {
            let c0 = String::from_utf8([elem0].to_vec()).unwrap_or(".".to_string());
            token_str0 = format!(
                "{}{}",
                token_str0,
                c0
            ).to_string();
        }

        println!(
            "{:?}\n{:?}\n{:?}",
            a0, a1,
            token_str0
        );



        use qrcode::*;
        use qrcode::render::unicode;

        let code = QrCode::with_version(
            token_str0,
            Version::Normal(10),
            // Version::Micro(4),
            EcLevel::L,
            // EcLevel::H
        ).unwrap();
        let string = code.render()
            .dark_color(unicode::Dense1x2::Light)
            .light_color(unicode::Dense1x2::Dark)
            .build();
        println!("{}", string);

    });
    s0.join();
    println!("License process join.");
}



#[no_mangle]
pub extern "C" fn initial_license_key_for_connection0(ptr: *mut *const u8, len: *mut usize) {
    println!("processing fn initial_license_key_for_connection0()");

    let mut license_key = String::new();

    use chrono::prelude::*;
    let utc: DateTime<Utc> = Utc::now();
    let stamp0 = utc.format("%d . %m . %Y   -   %H : %M : %S   ( UTC )").to_string();
    let utc0 = utc.checked_add_signed(chrono::Duration::days(1)).expect("Overflow when adding a day");
    let stamp1 = utc0.format("%d . %m . %Y   -   %H : %M : %S   ( UTC )").to_string();

    let mut data_param = serde_json::json!({
        "marked": "FLAT Alpha Protocol",
        "expired": stamp1,
    });

    
    use argon2::Argon2;

    let a0 = data_param["activate"].clone();
    let a1 = data_param["expired"].clone();

    // let binding = stamp0;
    let binding = a0.to_string();
    let password = binding.as_bytes();
    // let binding = stamp1;
    let binding = a1.to_string();
    let salt = binding.as_bytes();

    let mut output_key_material = [0u8; 100];
    Argon2::default().hash_password_into(password, salt, &mut output_key_material).unwrap();



    println!("HASH : {:?}", output_key_material.to_vec());

    // std::thread::sleep(std::time::Duration::from_secs(1));
    
    // format!("Result from dylib : {:?}", "ssssssss").to_string()
    
    let license_key = output_key_material.to_vec();

    // Передаем указатель на данные и длину
    unsafe {
        *ptr = license_key.as_ptr();
        *len = license_key.len();
    }

    // Важно! Rust по умолчанию освободит память, так как `license_key` выходит из области видимости,
    // поэтому используем `std::mem::forget`, чтобы предотвратить освобождение.
    std::mem::forget(license_key);
}



#[no_mangle]
pub extern "C" fn free_license_key(ptr: *mut *const u8, len: usize) {
    if !ptr.is_null() {
        unsafe {
            // Восстанавливаем Vec<u8> и освобождаем память
            let temp_check0 = Vec::from_raw_parts(ptr, len, len);
            println!(
                "temp_check0: {:?}",
                temp_check0
            );
        }
    }
}


























































pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
