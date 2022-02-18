use rasp_leds_v2::Color;

#[test]
fn test_rgb_to_u32() {
    let c = Color::RGB(255, 0, 0);
    let i: u32 = c.into();
    assert_eq!(i, 0xFF0000);

    let c = Color::RGB(0, 255, 0);
    let i: u32 = c.into();
    assert_eq!(i, 0xFF00);

    let c = Color::RGB(0, 0, 255);
    let i: u32 = c.into();
    assert_eq!(i, 0xFF);

    let c = Color::RGB(69, 128, 98);
    let i: u32 = c.into();
    assert_eq!(i, 4554850);

    let c = Color::RGB(125, 32, 68);
    assert_eq!(c.to_int(), 0x7D2044);

    let c = Color::RGB(255, 0, 0);
    assert_eq!(c.to_int(), 0xFF0000);

    let c = Color::RGB(0, 255, 0);
    assert_eq!(c.to_int(), 0xFF00);

    let c = Color::RGB(0, 0, 255);
    assert_eq!(c.to_int(), 0xFF);
}

#[test]
fn test_rgb_to_arr() {
    let c = Color::RGB(255, 0, 0);
    let i: [u8; 4] = c.into();
    assert_eq!(i, [0, 0, 255, 0]);

    let c = Color::RGB(0, 255, 0);
    let i: [u8; 4] = c.into();
    assert_eq!(i, [0, 255, 0, 0]);

    let c = Color::RGB(0, 0, 255);
    let i: [u8; 4] = c.into();
    assert_eq!(i, [255, 0, 0, 0]);

    let c = Color::RGB(69, 128, 98);
    let i: [u8; 4] = c.into();
    assert_eq!(i, [98, 128, 69, 0]);

    let c = Color::RGB(125, 32, 68);
    assert_eq!(c.to_arr(), [68, 32, 125, 0]);

    let c = Color::RGB(255, 0, 0);
    assert_eq!(c.to_arr(), [0, 0, 255, 0]);

    let c = Color::RGB(0, 255, 0);
    assert_eq!(c.to_arr(), [0, 255, 0, 0]);

    let c = Color::RGB(0, 0, 255);
    assert_eq!(c.to_arr(), [255, 0, 0, 0]);
}
