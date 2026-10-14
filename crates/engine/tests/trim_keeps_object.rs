//! TRIM and EXTEND keep the object (#1164): the piece that stays is the same object, with the
//! same handle, so its associative dimensions follow the new length as they do after LENGTHEN,
//! and its xdata and hyperlinks stay on it. A trim that leaves two pieces keeps the handle on
//! the first (the one at the object's start); the other piece is new.

use cadcraft_engine::Session;
use cadcraft_engine::doc::{Dimension, EntityKind, Handle};
use cadcraft_engine::geom::Vec2;
use serde_json::{Value, json};

fn handle(v: &Value) -> Handle {
    let s = v.get("handle").unwrap_or(&v["handles"][0]).as_str().unwrap();
    Handle::parse_hex(s).unwrap()
}

fn handles(v: &Value) -> Vec<Handle> {
    v["handles"].as_array().unwrap().iter().map(|h| Handle::parse_hex(h.as_str().unwrap()).unwrap()).collect()
}

fn line(s: &mut Session, a: [f64; 2], b: [f64; 2]) -> Handle {
    handle(&s.execute("line", &json!({ "points": [a, b] })).unwrap())
}

/// The ends of the line `h` (`None` when it is gone or not a line).
fn ends(s: &Session, h: Handle) -> Option<(Vec2, Vec2)> {
    match &s.doc().unwrap().entity(h)?.kind {
        EntityKind::Line(l) => Some((l.a.xy(), l.b.xy())),
        _ => None,
    }
}

fn dim(s: &Session, h: Handle) -> Dimension {
    match &s.doc().unwrap().entity(h).unwrap().kind {
        EntityKind::Dimension(d) => d.clone(),
        k => panic!("not a dimension: {k:?}"),
    }
}

fn near(a: Vec2, b: Vec2) -> bool {
    a.dist(b) < 1e-9
}

/// The text a dimension shows.
fn measured(s: &Session, h: Handle) -> String {
    cadcraft_engine::render::dimension_in(s.doc().unwrap(), &dim(s, h)).value
}

/// A horizontal line from 0,0 to `len`,0 with a linear dimension of it (both origins attached).
fn dimensioned_line(s: &mut Session, len: f64) -> (Handle, Handle) {
    let l = line(s, [0.0, 0.0], [len, 0.0]);
    let d = handle(&s.execute("dimlinear", &json!({ "object": l.hex(), "at": [50, -20] })).unwrap());
    let dm = dim(s, d);
    assert_eq!(dm.assoc.len(), 2, "{:?}", dm.assoc);
    assert!(dm.assoc.iter().all(|a| a.handle == l));
    (l, d)
}

#[test]
fn trim_keeps_the_line_so_its_dimension_follows() {
    let mut s = Session::new();
    let (l, d) = dimensioned_line(&mut s, 100.0);
    let cutter = line(&mut s, [60.0, -10.0], [60.0, 10.0]);
    let r = s.execute("trim", &json!({ "handle": l.hex(), "pick": [90, 0], "edges": [cutter.hex()] })).unwrap();
    assert_eq!(handles(&r), vec![l], "the piece that stays is the same object: {r}");
    assert_eq!(ends(&s, l), Some((Vec2::ZERO, Vec2::new(60.0, 0.0))));
    let dm = dim(&s, d);
    assert_eq!(dm.assoc.len(), 2, "the dimension stays attached: {:?}", dm.assoc);
    assert!(dm.assoc.iter().all(|a| a.handle == l));
    assert!(near(dm.p14.xy(), Vec2::new(60.0, 0.0)), "p14 follows the trimmed end: {:?}", dm.p14);
    assert!(near(dm.p13.xy(), Vec2::ZERO));
    assert_eq!(measured(&s, d), "60.0000");
    // Undo restores the line and the dimension in one step.
    s.undo().unwrap();
    assert_eq!(ends(&s, l), Some((Vec2::ZERO, Vec2::new(100.0, 0.0))));
    assert!(near(dim(&s, d).p14.xy(), Vec2::new(100.0, 0.0)));
}

#[test]
fn a_trim_in_the_middle_keeps_the_handle_on_the_first_piece() {
    let mut s = Session::new();
    let (l, d) = dimensioned_line(&mut s, 100.0);
    let c1 = line(&mut s, [40.0, -10.0], [40.0, 10.0]);
    let c2 = line(&mut s, [60.0, -10.0], [60.0, 10.0]);
    let r = s.execute("trim", &json!({ "handle": l.hex(), "pick": [50, 0], "edges": [c1.hex(), c2.hex()] })).unwrap();
    let hs = handles(&r);
    assert_eq!(hs.len(), 2, "{r}");
    assert_eq!(hs[0], l, "the first piece keeps the handle: {r}");
    assert_ne!(hs[1], l, "the second piece is a new object: {r}");
    assert_eq!(ends(&s, l), Some((Vec2::ZERO, Vec2::new(40.0, 0.0))));
    assert_eq!(ends(&s, hs[1]), Some((Vec2::new(60.0, 0.0), Vec2::new(100.0, 0.0))));
    let dm = dim(&s, d);
    assert_eq!(dm.assoc.len(), 2, "{:?}", dm.assoc);
    assert!(near(dm.p14.xy(), Vec2::new(40.0, 0.0)), "p14 follows the first piece's end: {:?}", dm.p14);
    assert_eq!(measured(&s, d), "40.0000");
}

#[test]
fn trim_at_the_command_line_keeps_the_object_too() {
    let mut s = Session::new();
    let (l, d) = dimensioned_line(&mut s, 100.0);
    line(&mut s, [60.0, -10.0], [60.0, 10.0]);
    // Quick mode (the default): every object is a cutting edge; pick the part to remove.
    s.script("TRIM 90,0\n\n").unwrap();
    assert!(s.running.is_none(), "{}", s.prompt_text());
    assert_eq!(ends(&s, l), Some((Vec2::ZERO, Vec2::new(60.0, 0.0))));
    let dm = dim(&s, d);
    assert_eq!(dm.assoc.len(), 2, "{:?}", dm.assoc);
    assert!(near(dm.p14.xy(), Vec2::new(60.0, 0.0)), "{:?}", dm.p14);
}

/// A LINE from 0,0 to 100,0 with a hyperlink and another application's data, read from DXF.
const XDATA_DXF: &str = "0\nSECTION\n2\nENTITIES\n\
0\nLINE\n5\n2A\n8\n0\n10\n0\n20\n0\n30\n0\n11\n100\n21\n0\n31\n0\n\
1001\nPE_URL\n1000\nhttps://example.com/door-7\n1002\n{\n1000\nDoor 7\n1002\n}\n\
1001\nACME_TAGS\n1000\nD-07\n1070\n3\n\
0\nENDSEC\n0\nEOF\n";

#[test]
fn the_kept_piece_keeps_its_xdata_and_hyperlink() {
    let mut s = Session::empty();
    s.open_drawing(cadcraft_io::read_dxf(XDATA_DXF.as_bytes()).unwrap(), "xdata", None);
    let l = s.doc().unwrap().model.iter().find(|e| matches!(e.kind, EntityKind::Line(_))).map(|e| e.handle).unwrap();
    let before = s.doc().unwrap().entity(l).unwrap().common.clone();
    assert!(before.xdata.iter().any(|t| t.value.trim() == "D-07"), "{before:?}");
    let cutter = line(&mut s, [60.0, -10.0], [60.0, 10.0]);
    let r = s.execute("trim", &json!({ "handle": l.hex(), "pick": [90, 0], "edges": [cutter.hex()] })).unwrap();
    assert_eq!(handles(&r), vec![l], "{r}");
    assert_eq!(ends(&s, l), Some((Vec2::ZERO, Vec2::new(60.0, 0.0))));
    assert_eq!(s.doc().unwrap().entity(l).unwrap().common, before);
}

#[test]
fn extend_keeps_the_line_so_its_dimension_follows() {
    let mut s = Session::new();
    let (l, d) = dimensioned_line(&mut s, 60.0);
    let boundary = line(&mut s, [100.0, -10.0], [100.0, 10.0]);
    s.execute("extend", &json!({ "handle": l.hex(), "pick": [55, 0], "edges": [boundary.hex()] })).unwrap();
    assert_eq!(ends(&s, l), Some((Vec2::ZERO, Vec2::new(100.0, 0.0))));
    let dm = dim(&s, d);
    assert_eq!(dm.assoc.len(), 2, "{:?}", dm.assoc);
    assert!(near(dm.p14.xy(), Vec2::new(100.0, 0.0)), "p14 follows the extended end: {:?}", dm.p14);
    assert_eq!(measured(&s, d), "100.0000");
}

#[test]
fn trimming_an_arc_keeps_the_arc() {
    let mut s = Session::new();
    // A half circle from 10,0 over 0,10 to -10,0, cut by the vertical line at x = 0.
    let a = handle(&s.execute("arc", &json!({ "center": [0, 0], "radius": 10, "start": 0, "end": 180 })).unwrap());
    let cutter = line(&mut s, [0.0, -20.0], [0.0, 20.0]);
    let r = s.execute("trim", &json!({ "handle": a.hex(), "pick": [-7.0710678, 7.0710678], "edges": [cutter.hex()] })).unwrap();
    assert_eq!(handles(&r), vec![a], "{r}");
    match &s.doc().unwrap().entity(a).unwrap().kind {
        EntityKind::Arc(ar) => {
            assert!((ar.start - 0.0).abs() < 1e-9 && (ar.end - std::f64::consts::FRAC_PI_2).abs() < 1e-9, "{ar:?}");
        }
        k => panic!("not an arc: {k:?}"),
    }
}

#[test]
fn trimming_a_hatch_boundary_open_ends_the_hatch_link_and_keeps_the_hatch() {
    let mut s = Session::new();
    let c = handle(&s.execute("circle", &json!({ "center": [0, 0], "radius": 10 })).unwrap());
    let h = handle(&s.execute("hatch", &json!({ "points": [[0, 0]] })).unwrap());
    let cutter = line(&mut s, [0.0, -20.0], [0.0, 20.0]);
    let r = s.execute("trim", &json!({ "handle": c.hex(), "pick": [-10, 0], "edges": [cutter.hex()] })).unwrap();
    assert_eq!(handles(&r), vec![c], "the circle stays the same object, now an arc: {r}");
    assert!(matches!(s.doc().unwrap().entity(c).unwrap().kind, EntityKind::Arc(_)));
    let hh = match &s.doc().unwrap().entity(h).unwrap().kind {
        EntityKind::Hatch(x) => x.clone(),
        k => panic!("not a hatch: {k:?}"),
    };
    assert!(!hh.associative, "an open boundary ends the link");
    assert_eq!(hh.loops.len(), 1, "the hatch keeps its shape");
    assert!(s.log.iter().any(|m| m.contains("no longer encloses an area")), "{:?}", s.log);
}
