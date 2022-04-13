
use x11rb::connection::Connection;
use x11rb::protocol::xproto::*;
use x11rb::errors::ReplyOrIdError;
use x11rb::COPY_DEPTH_FROM_PARENT;

pub fn create_win<C: Connection>(
    conn: &C,
    screen: &Screen
) -> Result<Window, ReplyOrIdError> {

    let win = conn.generate_id()?;
    let values_list = CreateWindowAux::default()
        .background_pixel(screen.white_pixel)
        .event_mask(EventMask::EXPOSURE|EventMask::KEY_PRESS);
    conn.create_window(
        COPY_DEPTH_FROM_PARENT,
        win,
        screen.root,
        0,
        0,
        150,
        150,
        10,
        WindowClass::INPUT_OUTPUT,
        screen.root_visual,
        &values_list,
    )?;

    Ok(win)
}

pub fn get_font<C: Connection>(
    conn: &C,
    screen: &Screen,
    win: Window,
    font_name: &str
) -> Result<Gcontext, ReplyOrIdError> {

    let font = conn.generate_id()?;
    conn.open_font(font, font_name.as_bytes())?;

    let gc = conn.generate_id()?;
    let values_list = CreateGCAux::default()
        .foreground(screen.black_pixel)
        .background(screen.white_pixel)
        .font(font);
    conn.create_gc(gc, win, &values_list)?;
    conn.close_font(font)?;
    
    Ok(gc)
}

pub fn draw_text<C:Connection>(
    conn: &C,
    screen: &Screen,
    win: Window,
    x: i16,
    y: i16,
    font_name: &str,
    text: &str
) -> Result<(), Box<dyn std::error::Error>> {

    let gc = get_font(conn, screen, win, font_name)?;
    conn.image_text8(win, gc, x, y, text.as_bytes())?;
    conn.free_gc(gc)?;
    Ok(())
}

