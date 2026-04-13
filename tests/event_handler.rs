//! Tests for the injectable `EventHandler::from_receiver` constructor.

use std::path::{Path, PathBuf};

use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use helixir::tui::event::{AppEvent, EventHandler};
use tokio::sync::mpsc;

fn mk_key(code: KeyCode) -> AppEvent {
    AppEvent::Key(KeyEvent::new(code, KeyModifiers::NONE))
}

#[tokio::test]
async fn from_receiver_forwards_pushed_events() {
    let (tx, rx) = mpsc::unbounded_channel();
    let mut handler = EventHandler::from_receiver(rx);

    tx.send(mk_key(KeyCode::Char('q'))).unwrap();
    tx.send(AppEvent::FileChanged(PathBuf::from("/a/b.hxt")))
        .unwrap();
    tx.send(AppEvent::Tick).unwrap();

    assert!(matches!(handler.next().await.unwrap(), AppEvent::Key(_)));
    assert!(matches!(
        handler.next().await.unwrap(),
        AppEvent::FileChanged(p) if p == Path::new("/a/b.hxt")
    ));
    assert!(matches!(handler.next().await.unwrap(), AppEvent::Tick));
}

#[tokio::test]
async fn from_receiver_preserves_order() {
    let (tx, rx) = mpsc::unbounded_channel();
    let mut handler = EventHandler::from_receiver(rx);

    for c in ['a', 'b', 'c'] {
        tx.send(mk_key(KeyCode::Char(c))).unwrap();
    }

    let mut got = String::new();
    for _ in 0..3 {
        if let AppEvent::Key(k) = handler.next().await.unwrap()
            && let KeyCode::Char(ch) = k.code
        {
            got.push(ch);
        }
    }
    assert_eq!(got, "abc");
}

#[tokio::test]
async fn closed_sender_produces_error() {
    let (tx, rx) = mpsc::unbounded_channel::<AppEvent>();
    let mut handler = EventHandler::from_receiver(rx);
    drop(tx);
    assert!(handler.next().await.is_err());
}
