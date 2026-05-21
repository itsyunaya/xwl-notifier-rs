use xcb::{self, x};

fn intern_atom(conn: &xcb::Connection, name: &[u8]) -> x::Atom {
    let cookie = conn.send_request(&x::InternAtom {
        only_if_exists: false,
        name,
    });

    conn.wait_for_reply(cookie).unwrap().atom()
}

fn walk_tree(
    conn: &xcb::Connection,
    window: x::Window,
    atoms: &(x::Atom, x::Atom),
    results: &mut Vec<WindowInfo>,
) {
    let cookie = conn.send_request(&x::QueryTree { window });
    let tree = match conn.wait_for_reply(cookie) {
        Ok(t) => t,
        Err(_) => return,
    };

    for &child in tree.children() {
        let class_raw = {
            let cookie = conn.send_request(&x::GetProperty {
                delete: false,
                window: child,
                property: atoms.0, // wm class
                r#type: x::ATOM_ANY,
                long_offset: 0,
                long_length: 1024,
            });

            conn.wait_for_reply(cookie)
                .map(|r| r.value::<u8>().to_vec())
                .unwrap_or_default()
        };

        if class_raw.is_empty() {
            walk_tree(conn, child, atoms, results);
            continue;
        }

        let mut parts = class_raw.split(|&b| b == 0).filter(|s| !s.is_empty());
        let instance = parts
            .next()
            .map(|s| String::from_utf8_lossy(s).into_owned())
            .unwrap_or_default();

        let class = parts
            .next()
            .map(|s| String::from_utf8_lossy(s).into_owned())
            .unwrap_or_default();

        let command = {
            let cookie = conn.send_request(&x::GetProperty {
                delete: false,
                window: child,
                property: atoms.1, // wm command
                r#type: x::ATOM_ANY,
                long_offset: 0,
                long_length: 1024,
            });

            conn.wait_for_reply(cookie)
                .map(|r| {
                    r.value::<u8>()
                        .split(|&b| b == 0)
                        .filter(|s| !s.is_empty())
                        .map(|s| String::from_utf8_lossy(s))
                        .collect::<Vec<_>>()
                        .join(" ")
                })
                .unwrap_or_default()
        };

        results.push(WindowInfo {
            window: child,
            instance,
            class,
            command,
        });
    }
}

#[allow(dead_code)]
pub struct WindowInfo {
    pub window: x::Window,
    pub instance: String,
    pub class: String,
    pub command: String,
}

pub fn get_x11_windows() -> Vec<WindowInfo> {
    let (conn, screen_num) = xcb::Connection::connect(None).unwrap();

    let atoms = (
        intern_atom(&conn, b"WM_CLASS"),
        intern_atom(&conn, b"WM_COMMAND"),
    );

    let setup = conn.get_setup();
    let screen = setup.roots().nth(screen_num as usize).unwrap();

    let mut results = Vec::new();
    walk_tree(&conn, screen.root(), &atoms, &mut results);

    results.retain(|e| !e.command.is_empty());
    
    results
}
