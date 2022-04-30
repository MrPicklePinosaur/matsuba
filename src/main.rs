
use x11rb::connection::Connection;
use matsuba::{
    converter,
    x::XSession,
    db,
    cli,
    error::BoxResult,
};

fn main() -> BoxResult<()> {

    let (conn, screen_num) = x11rb::connect(None)?;
    let screen = &conn.setup().roots[screen_num];

    let mut session = XSession::new(&conn, screen)?;
    session.configure_root()?;
    session.create_completion_box((100, 100), "testingtestingtesting")?;

    while session.is_running() {
        session.render_completion_box()?;
        conn.flush()?;

        let event = conn.wait_for_event()?;
        session.handle_event(&event)?;
    }

    drop(conn);
    Ok(())

}

